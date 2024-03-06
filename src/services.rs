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

pub fn markdown_to_html(markdown_src: &str) -> String {
    let parser = pulldown_cmark::Parser::new_ext(markdown_src, pulldown_cmark::Options::all());
    let mut unsafe_html = String::new();
    pulldown_cmark::html::push_html(&mut unsafe_html, parser);
    let html_content = ammonia::Builder::default()
        .generic_attributes(["id", "name"].into_iter().collect())
        .clean(&unsafe_html);

    html_content.into()
}