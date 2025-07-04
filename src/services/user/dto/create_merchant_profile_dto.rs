use crate::{
    db::entity::sea_orm_active_enums::MerchantCategory,
    services::error::{Result, ServiceError},
};
use axum::{body::Bytes, extract::Multipart};
use sea_orm::IntoActiveValue;

pub struct CreateMerchantProfileDto {
    pub display_name: String,
    pub cover: Option<Bytes>,
    pub address: String,
    pub business_registration_number: Option<String>,
    pub category: MerchantCategory,
}

/*
 * TODO: Write a macro that converts multipart to dto
 */

// Multipart limits the default file size to 2MB
pub async fn from_multipart_to_create_merchant_profle_dto(
    mut form: Multipart,
) -> Result<CreateMerchantProfileDto> {
    let mut display_name: Option<String> = None;
    let mut address: Option<String> = None;
    let mut business_registration_number: Option<String> = None;
    let mut cover: Option<Bytes> = None;
    let mut category: Option<MerchantCategory> = None;

    while let Some(field) = form.next_field().await? {
        match field.name() {
            Some("displayName") => {
                display_name = Some(
                    field
                        .text()
                        .await
                        .map_err(|_| ServiceError::DtoError("Display name is invalid".into()))?,
                );
            }
            Some("address") => {
                address = Some(
                    field
                        .text()
                        .await
                        .map_err(|_| ServiceError::DtoError("Address is invalid".into()))?,
                );
            }
            Some("category") => {
                let cat = field
                    .text()
                    .await
                    .map_err(|_| ServiceError::DtoError("Category is invalid".into()))?;
                category = Some(
                    cat.parse::<MerchantCategory>()
                        .map_err(|_| ServiceError::DtoError("failed to parse Category ".into()))?,
                );
            }
            Some("business_registration_number") => {
                business_registration_number = Some(field.text().await.map_err(|_| {
                    ServiceError::DtoError("Business registration number is invalid".into())
                })?);
            }
            Some("cover") => {
                cover = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|_| ServiceError::DtoError("Cover is invalid".into()))?,
                );
            }
            _ => {}
        }
    }

    let (display_name, address, category) = match (display_name, address, category) {
        (Some(t), Some(d), Some(c)) => (t, d, c),
        _ => {
            return Err(ServiceError::DtoError(
                "Missing one or more required fields".into(),
            ));
        }
    };

    Ok(CreateMerchantProfileDto {
        display_name,
        business_registration_number,
        address,
        cover,
        category,
    })
}
