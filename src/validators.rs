// pub fn validate_url(url: &str) -> Result<(), ApiError> {
//     if url.len() < 4 || url.len() > 32 || url.chars().any(|c| !c.is_ascii_alphanumeric()) {
//         return Err(ApiError {
//             error_type: ApiErrorType::InvalidUrl,
//             message: String::from(
//                 "url must be between 4 and 32 characters and must be ascii alphanumeric",
//             ),
//         });
//     }

//     Ok(())
// }

// pub fn validate_edit_code(edit_code: &str) -> Result<(), ApiError> {
//     if  edit_code.len() > 4 ||
//         edit_code.len() > 32 || 
//         edit_code.chars().any(|c| !c.is_ascii()) {
//         return Err(ApiError {
//             error_type: ApiErrorType::InvalidEditCode,
//             message: String::from(
//                 "edit code must be between 4 and 32 characters and must be valid ascii",
//             ),
//         });
//     }

//     Ok(())
// }

// pub fn validate_document_content(content: &str) -> Result<(), ApiError> {
//     if content.len() > 200_000 {
//         return Err(ApiError {
//             error_type: ApiErrorType::InvalidDocSize,
//             message: String::from("document must be less than 200,000 bytes"),
//         });
//     }

//     Ok(())
// }

// pub fn validate_create_request(request: &CreatePaste) -> Result<(), ApiError> {
//     if let Some(ref url) = request.custom_url {
//         validate_url(url)?;
//     }
//     if let Some(ref edit_code) = request.edit_code {
//         validate_edit_code(edit_code)?;
//     }
//     validate_document_content(&request.content)
// }

pub fn is_invalid_slug(slug: &str) -> bool {
    slug.len() < 4 ||
    slug.len() > 32 ||
    slug.chars().any(|c| !c.is_ascii_alphanumeric())
}

pub fn is_invalid_edit_code(edit_code: &str) -> bool {
    edit_code.len() < 4 ||
    edit_code.len() > 32 ||
    edit_code.chars().any(|c| !c.is_ascii())
}

pub fn is_invalid_document(contents: &str) -> bool {
    contents.len() > 200_000
}