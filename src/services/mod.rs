use chrono::Utc;
use nanoid::nanoid;

use crate::{
    db::{Database, DocEntry, UrlEntry},
    errors::{ApiError, ApiErrorType},
    routes::api::{CreatePaste, EditPaste},
    validators::validate_create_request,
};

pub async fn create_paste(
    db: &Database,
    request: CreatePaste,
) -> Result<(UrlEntry, DocEntry), ApiError> {
    validate_create_request(&request)?;

    let url = match request.custom_url {
        Some(custom_url) if db.contains_url(&custom_url)? => {
            return Err(ApiError {
                error_type: ApiErrorType::UrlTaken,
                message: String::from("the requested url is already taken"),
            });
        }
        Some(custom_url) => custom_url,
        None => {
            let url = nanoid!(16);
            if db.contains_url(&url)? {
                return Err(ApiError {
                    error_type: ApiErrorType::UrlTaken,
                    message: String::from("unable to generate a valid url. url space may be full"),
                });
            }
            url
        }
    };

    let edit_code = request.edit_code.unwrap_or(nanoid!(16));
    let doc_hash = *blake3::hash(&request.content.as_bytes()).as_bytes();

    let doc_entry = DocEntry {
        doc_id: doc_hash,
        content: request.content,
        created: Utc::now(),
    };

    let url_entry = UrlEntry {
        url_id: url.clone(),
        doc_id: doc_hash,
        edit_code,
    };

    db.insert_doc(&doc_hash, &doc_entry)?;
    db.insert_url(&url, &url_entry)?;

    todo!()
}

pub async fn edit_paste(
    db: &Database,
    request: EditPaste,
) -> Result<(UrlEntry, DocEntry), ApiError> {
    match db.get_url(&request.url)? {
        Some(url) if url.edit_code == request.edit_code => todo!(),
        Some(_) => {
            return Err(ApiError {
                error_type: ApiErrorType::InvalidEditCode,
                message: String::from("the edit code is not valid"),
            })
        }
        None => {
            return Err(ApiError {
                error_type: ApiErrorType::InvalidUrl,
                message: String::from("the url mentioned does not exist"),
            })
        }
    }

    todo!()
}
