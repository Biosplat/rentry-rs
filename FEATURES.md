# Pastebin Project - Feature Ideas

## Frontend Features

### Encrypted Pastes
- Implement a feature for creating encrypted pastes where the encryption key remains entirely client-side.
- Consider using `Argon2` for hashing and `ChaCha20Poly1305` for encryption to ensure security.

### Drag and Drop Interface
- Introduce a drag and drop interface to allow users to easily upload files (supporting file types beyond Markdown).

### QR Code Generation
- Add functionality to generate QR codes on the frontend for easy sharing of paste URLs.

## Backend Features

### File Support
- Extend the current system to support pastes of file types other than Markdown, enabling a broader range of use cases.

### Database Change
- Explore the possibility of changing the backend database to use `sled` or `sqlite` for potentially improved performance and simplicity.

### Syntax Highlighting
- Incorporate `syntect` for server-side syntax highlighting to enhance the readability of code snippets and other structured content.

## Interface Enhancements

### Terminal Interface
- Develop a terminal interface similar to `echo something | rentry-rs`, which outputs a URL to the created paste.
- Include additional arguments to allow users to request a URL and an edit code directly from the terminal.

### Expiration and Deletion
- Implement features for paste deletion based on various criteria:
  - Expiration: Allow setting an expiration date when submitting a paste.
  - Reading: Enable deletion of pastes after they have been read.
  - Ownership: Provide a mechanism for paste owners to delete their submissions.

## Additional Tools & References

- To gather inspiration and understand existing solutions in the market, review the list of self-hosted pastebins available at [Awesome Selfhosted Pastebins](https://github.com/awesome-selfhosted/awesome-selfhosted#pastebins).
- For an example of a Rust-written pastebin with extensive features, consider examining [Wastebin by Matze](https://github.com/matze/wastebin).