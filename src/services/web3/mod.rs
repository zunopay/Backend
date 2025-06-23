use crate::{
    config::config,
    constants::TREASURY_PUBKEY,
    services::error::{MathErrorType, Result, ServiceError},
};
use solana_client::{
    rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient},
    rpc_response::RpcConfirmedTransactionStatusWithSignature,
};
use solana_keypair::Keypair;
use solana_message::Message;
use solana_signature::Signature;
use solana_signer::Signer;
use solana_transaction::Transaction;
use spl_associated_token_account::get_associated_token_address;
use spl_token::{
    ID as TOKEN_PROGRAM_ID,
    instruction::transfer,
    solana_program::{instruction::AccountMeta, pubkey::Pubkey},
};
use std::{
    pin::Pin,
    str::FromStr,
    sync::{Arc, OnceLock},
};
use tokio::sync::OnceCell;
use validator::ValidateLength;

pub struct Web3Service {
    pub rpc_client: Arc<RpcClient>,
    fee_faucet: Keypair,
}

impl Web3Service {
    pub fn new() -> Result<Self> {
        let rpc_client = Arc::new(RpcClient::new(&config().RPC_URL));
        let fee_faucet = Self::get_fee_faucet_keypair()?;

        Ok(Web3Service {
            rpc_client,
            fee_faucet,
        })
    }

    pub async fn create_transfer_transaction(
        self: &Self,
        sender_wallet: &String,
        receiver_wallet: &String,
        amount: u64,
        token_mint_address: &String,
        reference_key: Pubkey,
    ) -> Result<Transaction> {
        let sender = Pubkey::from_str(sender_wallet)?;
        let receiver = Pubkey::from_str(receiver_wallet)?;
        let token_mint = Pubkey::from_str(token_mint_address)?;

        // Check and create token accounts ?
        let sender_token_account = get_associated_token_address(&sender, &token_mint);
        let receiver_token_account = get_associated_token_address(&receiver, &token_mint);

        let treasury = Pubkey::from_str(TREASURY_PUBKEY)?;
        let treasury_token_account = get_associated_token_address(&treasury, &token_mint);

        // 1% fee for transaction processing
        let fee = amount
            .checked_mul(1)
            .ok_or(ServiceError::MathError(MathErrorType::NumericalOverflow))?
            .checked_div(100)
            .ok_or(ServiceError::MathError(MathErrorType::NumericalOverflow))?;

        let amount_after_fee = amount
            .checked_sub(fee)
            .ok_or(ServiceError::MathError(MathErrorType::NumericalOverflow))?;

        let transfer_fee_instruction = transfer(
            &TOKEN_PROGRAM_ID,
            &sender_token_account,
            &treasury_token_account,
            &sender,
            &[&sender],
            fee,
        )?;

        let mut transfer_instruction = transfer(
            &TOKEN_PROGRAM_ID,
            &sender_token_account,
            &receiver_token_account,
            &sender,
            &[&sender],
            amount_after_fee,
        )?;
        transfer_instruction.accounts.push(AccountMeta {
            pubkey: reference_key,
            is_signer: false,
            is_writable: false,
        });

        let fee_faucet_pubkey = self.get_fee_faucet_pubkey();
        let mut transfer_transaction = Transaction::new_with_payer(
            &[transfer_fee_instruction, transfer_instruction],
            Some(&fee_faucet_pubkey),
        );

        let latest_blockhash = self.rpc_client.get_latest_blockhash()?;
        transfer_transaction.sign(&[&self.fee_faucet], latest_blockhash);

        // todo: check grid

        Ok(transfer_transaction)
    }

    /**
     * For recursive function, rust compiler is not able to calculate the size of return type due to function calling itself probably infinite times
     * So we use `dyn Future`, returning heap allocated Future (dyn to calculate size on runtime, Box to allocate on heap)
     * the return type should also implement `Pin`: This makes .await non movable, since awaited functions have to be reference again internally to resume after execution.
     * if awaited function moved in memory, the internal references to that function will become invalid resulting in undefined behavior
     *
     * `async fn` doesn't return `dyn Future` so change to `fn` because `async fn` resolves to `impl Future` (Concerete type)
     *
     *  use `async move {}` to capture the ownership and await the function and wrap in Box::pin to coerce into dyn Future.
     */
    pub fn find_reference(
        self: Arc<Self>,
        reference: String,
        before: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<RpcConfirmedTransactionStatusWithSignature>> + Send>>
    {
        //todo: limit to 10 depth at max
        let rpc_client = self.rpc_client.clone();

        Box::pin(async move {
            let reference_key = Pubkey::from_str(&reference)?;
            let mut config = GetConfirmedSignaturesForAddress2Config::default();

            config.before = match before {
                Some(before) => Some(Signature::from_str(&before).map_err(|e| {
                    ServiceError::Web3Error(super::error::Web3ErrorType::Custom(e.to_string()))
                })?),
                None => None,
            };

            let mut signatures =
                rpc_client.get_signatures_for_address_with_config(&reference_key, config)?;

            if signatures.is_empty() {
                return Err(ServiceError::Web3Error(
                    super::error::Web3ErrorType::ReferenceError,
                ));
            }

            let recent_transaction = signatures.pop().ok_or(ServiceError::Web3Error(
                super::error::Web3ErrorType::ReferenceError,
            ))?;
            if signatures.len() < 1000 {
                return Ok(recent_transaction);
            }

            let response = self
                .find_reference(reference, Some(recent_transaction.signature.clone()))
                .await;

            match response {
                Ok(found) => Ok(found),
                Err(ServiceError::Web3Error(super::error::Web3ErrorType::ReferenceError)) => {
                    Ok(recent_transaction)
                }
                Err(e) => Err(e),
            }
        })
    }

    fn get_fee_faucet_pubkey(&self) -> Pubkey {
        self.fee_faucet.pubkey()
    }

    fn get_fee_faucet_keypair() -> Result<Keypair> {
        let secret_key = config().FEE_FAUCET_SECRET_KEY.as_bytes();
        let private_key = config().FEE_FAUCET_PRIVATE_KEY.as_bytes();

        let key = [private_key, secret_key].concat();
        let keypair = Keypair::from_bytes(&key)
            .map_err(|_| ServiceError::KeypairError("Failed to derive keypair".to_string()))?;

        Ok(keypair)
    }

    async fn sign_transaction(transaction: Transaction) -> Transaction {
        todo!()
    }
}
