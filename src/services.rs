use chrono::Utc;

use crate::{
    db::{Database, DocumentRecord, SlugRecord},
    errors::Error,
};

pub fn create_paste(
    db: &Database,
    slug: &str,
    edit_code: &str,
    content: &str,
) -> Result<(), Error> {
    let hash = db.insert_document(&DocumentRecord {
        content: content.to_string(),
        created: Utc::now(),
    })?;

    db.insert_slug(
        slug,
        &SlugRecord {
            document_hash: hash,
            edit_code: edit_code.to_string(),
        },
    )?;

    Ok(())
}

pub fn edit_paste(db: &Database, slug: &str, edit_code: &str, content: &str) -> Result<(), Error> {
    let hash = db.insert_document(&DocumentRecord {
        content: content.to_string(),
        created: Utc::now(),
    })?;

    db.insert_slug(
        slug,
        &SlugRecord {
            document_hash: hash,
            edit_code: edit_code.to_string(),
        },
    )?;

    Ok(())
}