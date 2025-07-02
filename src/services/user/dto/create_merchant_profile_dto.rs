use crate::services::error::{Result, ServiceError};
use axum::{body::Bytes, extract::Multipart};

pub struct CreateMerchantProfileDto {
    pub display_name: String,
    pub cover: Option<Bytes>,
    pub address: String,
    pub business_registration_number: Option<String>,
}

/*
 * TODO: Write a macro that converts multipart to dto
 */

// Multipart limits the default file size to 2MB
pub async fn from_multipart_to_dto(mut form: Multipart) -> Result<CreateMerchantProfileDto> {
    let mut display_name: Option<String> = None;
    let mut address: Option<String> = None;
    let mut business_registration_number: Option<String> = None;
    let mut cover: Option<Bytes> = None;

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

    let (display_name, address) = match (display_name, address) {
        (Some(t), Some(d)) => (t, d),
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
    })
}
