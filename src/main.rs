use axum::{
    extract::{self, Path, State}, http::StatusCode, response::{Html, IntoResponse}, routing::{get, post}, Json, Router
};
use hex::ToHex;
use models::{CreateEdit, CreateEditResponse, CreatePost, CreatePostResponse, Document, Url};
use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client, Collection, IndexModel,
};
use nanoid::nanoid;
use thiserror::Error;
use tower_http::services::ServeDir;

pub mod models;

#[derive(Debug, Clone)]
pub struct DBState {
    pub urls: Collection<Url>,
    pub docs: Collection<Document>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("MongoDB Error: {0}")]
    MongoDB(#[from] mongodb::error::Error)
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            Error::MongoDB(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, "Internal Error").into_response()
    }
}

pub async fn setup_db() -> DBState {
    let client_options = ClientOptions::parse("mongodb://mongoadmin:secret@rocky.home.arpa:27017")
        .await
        .expect("unable to connect to database");

    let client = Client::with_options(client_options).expect("unable to create client");

    let db = client.database("rentry");

    // drop the database while creating rentry-rs
    // db.drop(None).await.expect("unable to drop database");

    let url_collection = db.collection::<Url>("urls");
    let doc_collection = db.collection::<Document>("docs");

    let options = IndexOptions::builder().unique(true).build();

    let url_model = IndexModel::builder()
        .keys(doc! {"url": 1u32})
        .options(options.clone())
        .build();

    let doc_model = IndexModel::builder()
        .keys(doc! {"hash": 1u32})
        .options(options.clone())
        .build();

    url_collection.create_index(url_model, None).await.unwrap();

    doc_collection.create_index(doc_model, None).await.unwrap();

    DBState {
        urls: url_collection,
        docs: doc_collection,
    }
}

pub async fn page(State(db_state): State<DBState>, Path(url): Path<String>) -> (StatusCode, Html<String>) {
    if let Some(url) = db_state.urls.find_one(doc! {"url": url}, None).await.expect("failed to search for url") {
        if let Some(doc) = db_state.docs.find_one(doc!{"hash": url.content_hash}, None).await.expect("failed to search for document") {
            let html = markdown::to_html(&doc.content);
            (StatusCode::OK, Html(html))
        } else {
            (StatusCode::NOT_FOUND, Html("Document Not Found".to_string()))
        }
    } else {
        (StatusCode::NOT_FOUND, Html("Not Found".to_string()))
    }
}

fn is_valid_url(url: &str) -> bool {
    url.len() >= 8 && 
    url.len() <= 32 && 
    url.chars().all(|c| c.is_ascii_alphanumeric())
}

fn is_valid_edit_code(edit_code: &str) -> bool {
    edit_code.len() >= 8 &&
    edit_code.len() <= 32 &&
    edit_code.chars().all(|a| char::is_ascii(&a))
}

/// Creates a document entry if none exists, otherwise returns the hash of the exisiting entry
async fn ensure_doc_entry(db_state: &DBState, content: String, hash: String) -> Result<String, Error> {
    let doc = db_state.docs.find_one(doc!{"hash": &hash}, None).await?;
    match doc {
        Some(doc) => {
            println!("using existing doc: {}", doc.hash);
            Ok(doc.hash)
        },
        None => {
            db_state.docs.insert_one(Document {
                content: content,
                hash: hash.clone(),
            }, None).await?;
            Ok(hash)
        },
    }
}

/// creates an entry in the url table that points to the document with `content_hash`
///
/// IMPORTANT: This method **DOES NOT** check if the url is valid or not!
async fn create_url_entry(db_state: &DBState, url: String, edit_code: String, content_hash: &str) -> Result<Json<CreatePostResponse>, Error> {
    // let edit_code = nanoid!(16);

    db_state.urls.insert_one(Url {
        edit_code: edit_code.clone(),
        url: url.clone(),
        content_hash: content_hash.to_string(),
    }, None).await?;

    Ok(Json(CreatePostResponse {
        url: Some(url),
        edit_code: Some(edit_code),
        success: true,
        message: None,
    }))
}

async fn create_paste(State(db_state): State<DBState>, extract::Json(payload): extract::Json<CreatePost>) -> Result<Json<CreatePostResponse>, Error> {
    let hash: String = md5::compute(&payload.content).encode_hex();
    let hash = ensure_doc_entry(&db_state, payload.content, hash).await?;


    let edit_code = match payload.edit_code {
        Some(edit_code) if is_valid_edit_code(&edit_code) => {
            edit_code
        },
        Some(_invalid_edit_code) => {
            return Ok(Json(CreatePostResponse {
                url: None,
                edit_code: None,
                success: false,
                message: Some("Custom edid code must be valid ascii and between 8 and 32 characters long.".to_string()),
            }));
        },
        None => {
            nanoid!(16)
        }
    };

    if let Some(ref custom_url) = payload.url {
        if !is_valid_url(custom_url) {
            return Ok(Json(CreatePostResponse {
                url: None,
                edit_code: None,
                success: false,
                message: Some("Custom URL must be alphanumeric and at between 8 and 32 characters long.".to_string()),
            }));
        }

        if db_state.urls.count_documents(doc! {"url": custom_url}, None).await? > 0 {
            return Ok(Json(CreatePostResponse {
                edit_code: None,
                url: None,
                success: false,
                message: Some("URL taken.".to_string()),
            }));
        }

        create_url_entry(&db_state, custom_url.clone(), edit_code, &hash).await
    } else {
        let url = nanoid!(8);
        create_url_entry(&db_state, url, edit_code, &hash).await
    }
}

pub async fn edit_paste(
    State(db_state): State<DBState>,
    extract::Json(edit_request): extract::Json<CreateEdit>,
) -> Result<Json<CreateEditResponse>, Error> {
    let url_entry = db_state.urls.find_one(doc! {"url": &edit_request.url}, None).await?;

    match url_entry {
        Some(url) => {
            if url.edit_code == edit_request.edit_code {
                let new_hash: String = md5::compute(&edit_request.content).0.encode_hex();
                let new_document_hash = ensure_doc_entry(&db_state, edit_request.content, new_hash).await?;

                db_state.urls.update_one(
                    doc! {"url": &edit_request.url},
                    doc! {"$set": {"content_hash": new_document_hash}},
                    None
                ).await?;

                Ok(Json(CreateEditResponse {
                    success: true,
                    message: None,
                }))
            } else {
                Ok(Json(CreateEditResponse {
                    success: false,
                    message: Some("Invalid edit code.".to_string()),
                }))
            }
        },
        None => Ok(Json(CreateEditResponse {
            success: false,
            message: Some("URL not found.".to_string()),
        })),
    }
}

pub async fn show_paste(
    State(db_state): State<DBState>,
    Path(url): Path<String>,
) -> Result<Html<String>, StatusCode> {
    let url_entry = match db_state.urls.find_one(doc! {"url": &url}, None).await {
        Ok(Some(url_entry)) => url_entry,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let document = match db_state.docs.find_one(doc! {"hash": &url_entry.content_hash}, None).await {
        Ok(Some(document)) => document,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let options = pulldown_cmark::Options::all();
    let md_parse = pulldown_cmark::Parser::new_ext(&document.content, options);

    let mut unsafe_html = String::new();
    pulldown_cmark::html::push_html(&mut unsafe_html, md_parse);

    let html_content = ammonia::Builder::default()
        .generic_attributes(["id", "name"].into_iter().collect())
        .clean(&unsafe_html);
    
    let full_html = format!(r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Document</title>
        <link rel="stylesheet" href="/markdown-styles.css">
    </head>
    <body style="background-color: #0d1117; color: #c9d1d9">
        <div class="markdown-body" style="width: 800px; margin: 0 auto; padding: 20px;">
        {}
        </div>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github-dark.min.css">
        <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
        <script>hljs.highlightAll();</script>
    </body>
    
    </html>"#, html_content);

    Ok(Html(full_html))
}

#[tokio::main]
async fn main() {
    let db_state = setup_db().await;

    let api_routes = Router::new()
        .route("/paste", post(create_paste))
        .route("/edit", post(edit_paste));

    let static_files = ServeDir::new("static").append_index_html_on_directories(true);

    let app = Router::new()
        .nest("/api", api_routes)
        .nest_service("/", static_files)
        .route("/p/:id", get(show_paste)) // Make sure this matches your intended URL pattern
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not Found") })
        .with_state(db_state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
