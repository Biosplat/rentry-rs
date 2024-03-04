# Rentry-RS

Rentry-RS is a Rust-based web application for sharing markdown documents online with the capability to edit them later using unique URLs and edit codes. Built with Axum for web serving, MongoDB for data persistence, and several other Rust crates, this project demonstrates a simple, yet powerful way to manage and share content securely and efficiently.

## Features

- **Markdown Support**: Users can write and share documents in Markdown, which are then rendered to HTML for viewing.
- **Edit Capability**: Documents can be edited post-creation by using a unique URL and edit code provided upon the document's creation.
- **MongoDB Backend**: Robust and flexible storage using MongoDB, ensuring scalability and performance.
- **Custom URL and Edit Code**: Users have the option to define custom URLs for their documents and specify their own edit codes for added personalization and security.

## Getting Started

To get started with Rentry-RS, follow these steps:

### Prerequisites

Ensure you have the following installed on your system:

- Rust and Cargo (latest stable version recommended)
- MongoDB (running and accessible for your application)

### Installation

1. **Clone the repository:**

```bash
git clone https://your-repository-url/rentry-rs.git
cd rentry-rs
```

2. **Setup MongoDB:**

Ensure MongoDB is running and accessible. Update the MongoDB connection string in `main.rs` if your setup differs from the default configuration.

3. **Build and Run:**

```bash
cargo build --release
cargo run --release
```

The application will start and listen on `127.0.0.1:3000`. You can access the web interface by navigating to `http://127.0.0.1:3000` in your web browser.

### Usage

- **Creating a Paste:**

To create a new document, send a POST request to `/api/paste` with a JSON payload containing the `content`, and optionally, `url` and `edit_code`.

Example:
```json
{
  "content": "Hello, World! This is my first markdown document.",
  "url": "custom-url",
  "edit_code": "customeditcode"
}
```

- **Editing a Paste:**

To edit an existing document, send a POST request to `/api/edit` with a JSON payload containing the `url`, `edit_code`, and new `content`.

Example:
```json
{
  "url": "custom-url",
  "edit_code": "customeditcode",
  "content": "Updated content of the document."
}
```

- **Viewing a Document:**

Navigate to `/p/{id}` where `{id}` is the document's unique URL to view the rendered Markdown document.

### Development

This project is open for development, and contributions are welcome. Please ensure to follow Rust's idiomatic practices and include tests for new features.

## Technology Stack

- **Rust**: The primary programming language used.
- **Axum**: A web application framework for Rust.
- **MongoDB**: The database used for storing documents and URLs.
- **Serde**: Serialization and deserialization framework for Rust, used for working with JSON data.
- **Hex**: For encoding and decoding hexadecimal values.
- **Nanoid**: To generate unique identifiers for URLs and edit codes.
- **Thiserror & Tower-HTTP**: For error handling and HTTP utilities.

## License

This project is licensed under the MIT License - see the LICENSE file for details.