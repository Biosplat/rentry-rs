use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sled::IVec;

use crate::errors::Error;

#[derive(Debug, Clone)]
pub struct Database {
    db: sled::Db,

    urls: sled::Tree, // stores all urls
    docs: sled::Tree, // stores all docs
    // html: sled::Tree, // stores cached html for docs
}

impl Database {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Result<Self, sled::Error> {
        let db = sled::Config::default()
            .use_compression(true)
            .compression_factor(10)
            .path(path)
            .open()?;

        let urls = db.open_tree("urls")?;
        let docs = db.open_tree("docs")?;
        // TODO: maybe add html caching for when converting the markdown documents into html
        // let html = db.open_tree("html")?;

        Ok(Self {
            db,
            urls,
            docs,
            // html
        })
    }

    pub fn insert_doc(&self, key: &DocId, doc: &DocEntry) -> Result<Option<DocEntry>, Error> {
        Self::insert_and_transform(&self.docs, key, doc)
    }

    pub fn get_doc(&self, key: &DocId) -> Result<Option<DocEntry>, Error> {
        Self::get_and_transform(&self.docs, key)
    }

    pub fn remove_doc(&self, key: &DocId) -> Result<Option<DocEntry>, Error> {
        Self::remove(&self.docs, key)
    }

    pub fn insert_url(&self, key: &UrlId, url: &UrlEntry) -> Result<Option<UrlEntry>, Error> {
        Self::insert_and_transform(&self.urls, key, url)
    }

    pub fn get_url(&self, key: &UrlId) -> Result<Option<UrlEntry>, Error> {
        Self::get_and_transform(&self.urls, key)
    }

    pub fn remove_url(&self, key: &UrlId) -> Result<Option<UrlEntry>, Error> {
        Self::remove(&self.urls, key)
    }

    /// Inserts a key-value pair into the specified sled `Tree`, potentially replacing the previous value
    /// associated with the same key. If a previous value exists, it attempts to transform it into the
    /// specified type `T` using the `FromIVec` trait.
    ///
    /// # Parameters
    ///
    /// - `store`: Reference to the sled `Tree` where the key-value pair will be inserted.
    /// - `key`: The key for the insertion. Must implement `IntoIVec` to allow conversion into `IVec`,
    ///   which is the key format used by sled.
    /// - `value`: The value to insert. Must also implement `IntoIVec` to be converted into `IVec`
    ///   for storage in the sled database.
    ///
    /// # Type Parameters
    ///
    /// - `K`: The type of the key. Must implement the `IntoIVec` trait.
    /// - `V`: The type of the value to be inserted. Must implement the `IntoIVec` trait.
    /// - `T`: The type into which the previous value (if any) will be transformed. Must implement
    ///   the `FromIVec` trait.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(T))` if a previous value was present and successfully transformed into type `T`.
    /// - `Ok(None)` if no previous value was present.
    /// - `Err(Error)` if an error occurs during the insertion, serialization, deserialization,
    ///   or transformation process.
    ///
    /// # Errors
    ///
    /// This function can return an error if the key or value serialization fails, if the sled operation
    /// itself fails (e.g., due to IO issues), or if transforming the previous value into type `T` fails.
    pub fn insert_and_transform<K, V, T>(
        store: &sled::Tree,
        key: K,
        value: V
    ) -> Result<Option<T>, Error>
    where
        K: IntoIVec,
        V: IntoIVec,
        T: FromIVec,
    {
        let previous = store.insert(key.to_ivec()?, value.to_ivec()?)?;
        Ok(previous.map(|p| T::from_ivec(&p)).transpose()?)
    }
    
    /// Retrieves the value associated with the specified key from the sled `Tree` and attempts to
    /// transform it into the specified type `V` using the `FromIVec` trait.
    ///
    /// # Parameters
    ///
    /// - `store`: Reference to the sled `Tree` from which the value will be retrieved.
    /// - `key`: The key whose associated value is to be retrieved. Must implement `IntoIVec` to
    ///   allow conversion into `IVec`, which is the key format used by sled.
    ///
    /// # Type Parameters
    ///
    /// - `K`: The type of the key. Must implement the `IntoIVec` trait.
    /// - `V`: The type into which the retrieved value will be transformed. Must implement
    ///   the `FromIVec` trait.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(V))` if a value was present for the key and successfully transformed into type `V`.
    /// - `Ok(None)` if no value was present for the key.
    /// - `Err(Error)` if an error occurs during the retrieval, deserialization,
    ///   or transformation process.
    ///
    /// # Errors
    ///
    /// This function can return an error if the key serialization fails, if the sled operation itself
    /// fails (e.g., due to IO issues), or if transforming the retrieved value into type `V` fails.
    pub fn get_and_transform<K, V>(
        store: &sled::Tree,
        key: K
    ) -> Result<Option<V>, Error>
    where
        K: IntoIVec,
        V: FromIVec
    {
        let value = store.get(key.to_ivec()?)?;
        Ok(value.map(|p| FromIVec::from_ivec(&p)).transpose()?)
    }

    /// Removes an entry from a sled `Tree` by its key and returns the removed value, if any.
    ///
    /// # Parameters
    /// - `store`: A reference to the sled `Tree`.
    /// - `key`: The key of the entry to remove. Must be convertible to `IVec`.
    ///
    /// # Type Parameters
    /// - `K`: Type of the key, must implement `IntoIVec`.
    /// - `V`: Type of the value to return, must implement `FromIVec`.
    ///
    /// # Returns
    /// - `Ok(Some(V))` if an entry was found and removed, converted to type `V`.
    /// - `Ok(None)` if no entry was found for the key.
    /// - `Err(Error)` on failure, such as serialization issues.
    ///
    /// # Example
    /// ```no_run
    /// // Assuming `store` is a sled::Tree, `key` is the key to remove
    /// let result = remove_entry(&store, key);
    /// ```
    pub fn remove<K, V>(
        store: &sled::Tree, 
        key: K
    ) -> Result<Option<V>, Error>
    where
        K: IntoIVec,
        V: FromIVec
    {
        Ok(store.remove(key.to_ivec()?)?.map(|p| FromIVec::from_ivec(&p)).transpose()?)
    }

}

/// A trait for converting a value into an `IVec`, the byte vector type used by sled.
/// 
/// This trait facilitates serialization of types that implement `Serialize` into `IVec`,
/// making it easier to store them in a sled database.
pub trait IntoIVec: Sized {
    /// Converts the implementing type into an `IVec`.
    ///
    /// # Returns
    /// 
    /// A `Result` wrapping an `IVec` on success, or a `bincode::Error` if serialization fails.
    fn to_ivec(&self) -> Result<IVec, bincode::Error>;
}

/// A trait for converting an `IVec` back into a value.
/// 
/// This trait is used for deserializing data stored as `IVec` in a sled database back
/// into its original type, provided it implements `Deserialize`.
pub trait FromIVec: Sized {
    /// Converts an `IVec` back into the implementing type.
    ///
    /// # Parameters
    /// 
    /// * `ivec`: A reference to the `IVec` to be deserialized.
    ///
    /// # Returns
    /// 
    /// A `Result` wrapping the deserialized value on success, or a `bincode::Error` if deserialization fails.
    fn from_ivec(ivec: &IVec) -> Result<Self, bincode::Error>;
}

impl<T> IntoIVec for T
where
    T: Serialize,
{
    /// Implements the conversion of a value that implements `Serialize` into an `IVec`.
    /// This is done by serializing the value using `bincode`.
    ///
    /// # Returns
    /// 
    /// A `Result` wrapping an `IVec` on success, or a `bincode::Error` if serialization fails.
    fn to_ivec(&self) -> Result<IVec, bincode::Error> {
        bincode::serialize(self).map(Into::into)
    }
}

impl<T> FromIVec for T
where
    T: for<'de> Deserialize<'de>,
{
    /// Implements the conversion of an `IVec` back into a value that implements `Deserialize`.
    /// This is done by deserializing the `IVec` using `bincode`.
    ///
    /// # Parameters
    /// 
    /// * `ivec`: A reference to the `IVec` to be deserialized.
    ///
    /// # Returns
    /// 
    /// A `Result` wrapping the deserialized value on success, or a `bincode::Error` if deserialization fails.
    fn from_ivec(ivec: &IVec) -> Result<Self, bincode::Error> {
        bincode::deserialize(ivec)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocEntry {
    content: String,
    created: DateTime<Utc>,
    expiry: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocId(pub [u8; 32]);

impl From<[u8; 32]> for DocId {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrlId(pub String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UrlEntry {
    url_id: UrlId,
    doc_id: DocId,
}





#[cfg(test)]
mod tests {
    use super::*; // Adjust this import based on your file structure
    use tempfile::Builder;

    #[test]
    fn test_insert_and_get_doc() {
        let temp_dir = Builder::new().prefix("temp_db").tempdir().unwrap();
        let db = Database::new(temp_dir.path()).unwrap();

        let doc_id = DocId::from([0; 32]); // Replace with appropriate DocId initialization
        let doc_entry = DocEntry {
            content: "Test content".into(),
            created: Utc::now(),
            expiry: None, // Set expiry as needed
        };

        // Insert document
        assert!(db.insert_doc(&doc_id, &doc_entry).is_ok());
        
        // Retrieve document
        let retrieved = db.get_doc(&doc_id).unwrap();
        assert_eq!(retrieved, Some(doc_entry));
    }

    #[test]
    fn test_remove_doc() {
        let temp_dir = Builder::new().prefix("temp_db").tempdir().unwrap();
        let db = Database::new(temp_dir.path()).unwrap();

        let doc_id = DocId::from([1; 32]); // Use a different key for clarity
        let doc_entry = DocEntry {
            content: "Content to remove".into(),
            created: Utc::now(),
            expiry: None,
        };

        // Ensure the document is inserted
        db.insert_doc(&doc_id, &doc_entry).unwrap();

        // Now remove it
        let removed = db.remove_doc(&doc_id).unwrap();
        assert_eq!(removed, Some(doc_entry));

        // Verify it's no longer there
        let retrieved_after_removal = db.get_doc(&doc_id).unwrap();
        assert!(retrieved_after_removal.is_none());
    }

    #[test]
    fn test_insert_and_get_url() {
        let temp_dir = Builder::new().prefix("temp_db").tempdir().unwrap();
        let db = Database::new(temp_dir.path()).unwrap();
        
        let url_id = UrlId("random_id".to_string()); // Ensure your UrlId can be constructed as shown
        let doc_id = DocId::from([2; 32]);
        let url_entry = UrlEntry { doc_id, url_id: url_id.clone() };
        
        // Insert URL
        assert!(db.insert_url(&url_id, &url_entry).is_ok());
    
        // Retrieve URL
        let retrieved = db.get_url(&url_id).unwrap();
        assert_eq!(retrieved, Some(url_entry));
    }
}