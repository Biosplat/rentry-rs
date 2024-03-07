use chrono::Utc;
use pulldown_cmark::{CodeBlockKind, Event, Tag, TagEnd};
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};

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
        .generic_attributes(["id", "name", "class"].into_iter().collect())
        .clean(&unsafe_html);

    html_content.into()
}

pub fn markdown_to_html_pretty(markdown_src: &str) -> String {
    let parser = pulldown_cmark::Parser::new_ext(markdown_src, pulldown_cmark::Options::all());

    let mut new_parser = Vec::new();
    let mut in_code_block = false;
    let mut to_highlight = String::new();
    
    
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let mut syntax = ss.find_syntax_plain_text();
    let theme = &ts.themes["InspiredGitHub"];

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(l))) => {
                in_code_block = true;           
                syntax = ss.find_syntax_by_extension(&l).unwrap_or(ss.find_syntax_plain_text());
            },
            Event::End(TagEnd::CodeBlock) => {
                if in_code_block {
                    let html = highlighted_html_for_string(&to_highlight, &ss, &syntax, &theme).unwrap();
                    new_parser.push(Event::Html(html.into()));
                    to_highlight = String::new();
                    in_code_block = false;
                }
            },
            Event::Text(t) => {
                if in_code_block {
                    to_highlight.push_str(&t);
                } else {
                    new_parser.push(Event::Text(t))
                }
            },
            e => new_parser.push(e)
        }
    }

    let mut s = String::new();
    pulldown_cmark::html::push_html(&mut s, new_parser.into_iter());

    s
}