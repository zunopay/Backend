use crate::db::entity::sea_orm_active_enums::TransferStatus;
use crate::db::entity::{payment, transfer};
use crate::db::entity::{
    payment::Entity as Payment,
    transfer::{Entity as Transfer, Model as TransferModel},
    user::Model as UserModel,
};
use crate::services::error::MathErrorType;
use crate::services::{
    AppState,
    error::{Result, ServiceError, Web3ErrorType},
};
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use solana_instruction::{AccountMeta, Instruction};
use solana_message::{Message, VersionedMessage, compiled_instruction::CompiledInstruction};
use solana_signature::Signature;
use solana_transaction::{Transaction, versioned::VersionedTransaction};
use solana_transaction_status_client_types::{
    EncodedTransactionWithStatusMeta, UiTransactionEncoding, UiTransactionStatusMeta,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::TokenInstruction;
use spl_token::solana_program::pubkey::Pubkey;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use validator::{Validate, ValidateRange};

pub struct Indexer {
    pub duration: u64,
}

pub struct TransferInstructionData {
    source: Pubkey,
    destination: Pubkey,
    authority: Pubkey,
    mint: Option<Pubkey>,
}

impl Indexer {
    pub fn new(duration: u64) -> Self {
        Indexer { duration }
    }

    // To spawn a thread for this function, all the futures in the function should be 'Send' as well.
    pub async fn poll_payment(
        state: Arc<AppState>,
        reference: String,
        receipt: String,
        mint: String,
        amount: u64,
    ) -> Result<()> {
        let mut ticker = interval(Duration::from_secs(1));
        let (status, signature) = loop {
            ticker.tick().await;
            let status = state
                .web3
                .clone()
                .find_reference(reference.clone(), None)
                .await;

            // Todo: Keep loop for 10 minutes, 8 minutes for user and 2 minutes buffer check
            let reference = Pubkey::from_str(&reference)?;
            let wallet_address = Pubkey::from_str(&receipt)?;
            let mint = Pubkey::from_str(&mint)?;

            match status {
                Ok(status) => {
                    let transfer_status = Self::validate_payment(
                        &state,
                        &status,
                        &reference,
                        &wallet_address,
                        &mint,
                        amount,
                    )
                    .await?;

                    break Ok((transfer_status, status.signature));
                }
                Err(ServiceError::Web3Error(Web3ErrorType::ReferenceError)) => continue,
                Err(e) => break Err(e),
            }
        }?;

        let transfer_data = transfer::ActiveModel {
            signature: Set(Some(signature)),
            status: Set((status)),
            ..Default::default()
        };

        Transfer::update(transfer_data)
            .filter(transfer::Column::ReferenceKey.eq(reference))
            .exec(state.db())
            .await?;
        //todo: Send websocket event

        Ok(())
    }

    async fn validate_payment(
        state: &AppState,
        status: &RpcConfirmedTransactionStatusWithSignature,
        reference: &Pubkey,
        receipt: &Pubkey,
        mint: &Pubkey,
        amount: u64,
    ) -> Result<TransferStatus> {
        //todo: amount check should be done after fee and also check the fee transfer instruction in validation.
        let transaction_response =
            Self::validate_transfer(state, &status.signature, reference, receipt, mint, amount)
                .await?;

        let transfer_status = if status.err.is_some() {
            TransferStatus::Rejected
        } else {
            TransferStatus::Completed
        };

        Ok(transfer_status)
    }

    async fn validate_transfer(
        state: &AppState,
        signature: &String,
        reference: &Pubkey,
        receipt: &Pubkey,
        mint: &Pubkey,
        amount: u64,
    ) -> Result<EncodedTransactionWithStatusMeta> {
        let response = state.web3.rpc_client.get_transaction(
            &Signature::from_str(&signature).map_err(|_| {
                ServiceError::Web3Error(Web3ErrorType::ValidateTransferError(
                    "Error parsing signature from string".to_string(),
                ))
            })?,
            UiTransactionEncoding::Json,
        )?;
        let transaction =
            response
                .transaction
                .transaction
                .decode()
                .ok_or(ServiceError::Web3Error(
                    Web3ErrorType::ValidateTransferError("Not Found".to_string()),
                ))?;

        let meta = response
            .transaction
            .meta
            .as_ref()
            .ok_or(ServiceError::Web3Error(
                Web3ErrorType::ValidateTransferError("Missing meta".to_string()),
            ))?;

        let compiled_ix =
            transaction
                .message
                .instructions()
                .last()
                .cloned()
                .ok_or(ServiceError::Web3Error(
                    Web3ErrorType::ValidateTransferError("Invalid instruction".to_string()),
                ))?;

        let instruction = Self::decompile_instruction(compiled_ix, &transaction.message)?;
        let (pre_balance, post_balance) = Self::validate_spl_transfer(
            &instruction,
            &transaction.message,
            meta,
            receipt,
            mint,
            reference,
        )
        .await?;

        let is_transferred = post_balance
            .checked_sub(pre_balance)
            .map(|balance| balance.less_than(amount))
            .flatten()
            .ok_or(ServiceError::MathError(MathErrorType::NumericalOverflow))?;

        if !is_transferred {
            return Err(ServiceError::Web3Error(
                Web3ErrorType::ValidateTransferError("Amount not transferred".to_string()),
            ));
        }

        Ok(response.transaction)
    }

    async fn validate_spl_transfer(
        instruction: &Instruction,
        message: &VersionedMessage,
        meta: &UiTransactionStatusMeta,
        receipt: &Pubkey,
        mint: &Pubkey,
        reference: &Pubkey,
    ) -> Result<(u64, u64)> {
        let receipt_ata = get_associated_token_address(receipt, mint);
        // 1. receipt_ata exists
        let ata_index = instruction
            .accounts
            .iter()
            .position(|account| account.pubkey.eq(&receipt_ata))
            .ok_or(ServiceError::Web3Error(
                Web3ErrorType::ValidateTransferError("Receipt not found".to_string()),
            ))?;

        // 2. check if spl transfer instruction (checked or normal)
        let (_, remaining_accounts, _) = Self::decode_transfer_instruction_data(instruction)?;

        // 3. reference key exists  (todo: reference could be array)
        remaining_accounts
            .iter()
            .find(|account| account.pubkey.eq(reference))
            .ok_or(ServiceError::Web3Error(
                Web3ErrorType::ValidateTransferError("Invalid reference".to_string()),
            ))?;

        // 4. amount check
        let pre_balance = meta
            .pre_balances
            .get(ata_index)
            .or_else(|| Some(&0))
            .ok_or(ServiceError::MathError(MathErrorType::NumericalOverflow))?;

        let post_balance = meta
            .post_balances
            .get(ata_index)
            .or_else(|| Some(&0))
            .ok_or(ServiceError::MathError(MathErrorType::NumericalOverflow))?;

        Ok((pre_balance.to_owned(), post_balance.to_owned()))
    }

    fn decode_transfer_instruction_data(
        instruction: &Instruction,
    ) -> Result<(TransferInstructionData, Vec<AccountMeta>, u64)> {
        let token_instruction = TokenInstruction::unpack(&instruction.data).map_err(|_| {
            ServiceError::Web3Error(Web3ErrorType::ValidateTransferError(
                "Failed to unpack token instruction".to_string(),
            ))
        })?;

        //TODO: Check program id
        let decoded_instruction_data = match token_instruction {
            TokenInstruction::Transfer { amount } => {
                let source = instruction.accounts[0].clone();
                let destination = instruction.accounts[1].clone();
                let authority = instruction.accounts[2].clone();
                let remaining_accounts = instruction.accounts[3..].to_vec();

                let transfer_data = TransferInstructionData {
                    source: source.pubkey,
                    destination: destination.pubkey,
                    authority: authority.pubkey,
                    mint: None,
                };

                Ok((transfer_data, remaining_accounts, amount))
            }
            TokenInstruction::TransferChecked { amount, decimals } => {
                let source = instruction.accounts[0].clone();
                let mint = instruction.accounts[1].clone();
                let destination = instruction.accounts[2].clone();
                let authority = instruction.accounts[3].clone();

                let remaining_accounts = instruction.accounts[4..].to_vec();

                let transfer_data = TransferInstructionData {
                    source: source.pubkey,
                    destination: destination.pubkey,
                    authority: authority.pubkey,
                    mint: None,
                };

                Ok((transfer_data, remaining_accounts, amount))
            }
            _ => Err(ServiceError::Web3Error(
                Web3ErrorType::ValidateTransferError("Invalid transfer".to_string()),
            )),
        }?;

        Ok(decoded_instruction_data)
    }

    fn decompile_instruction(
        compiled_ix: CompiledInstruction,
        message: &VersionedMessage,
    ) -> Result<Instruction> {
        let keys = message.static_account_keys();

        let account_metas = compiled_ix
            .accounts
            .iter()
            .map(|account_index| -> Result<AccountMeta> {
                let account_index = usize::try_from(*account_index).map_err(|_| {
                    ServiceError::MathError(super::error::MathErrorType::NumericalOverflow)
                })?;
                let is_signer = message.is_signer(account_index);

                // Safe to use for client side validation
                let is_writable = message.is_maybe_writable(account_index, None);

                Ok(AccountMeta {
                    pubkey: keys[account_index].clone(),
                    is_signer,
                    is_writable,
                })
            })
            .collect::<Result<Vec<AccountMeta>>>()?;

        let program_id_index = usize::try_from(compiled_ix.program_id_index)
            .map_err(|_| ServiceError::MathError(super::error::MathErrorType::NumericalOverflow))?;
        Ok(Instruction {
            program_id: keys[program_id_index].clone(),
            accounts: account_metas,
            data: compiled_ix.data,
        })
    }
}
