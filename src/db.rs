use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sled::IVec;

use crate::errors::Error;

#[derive(Debug, Clone)]
pub struct Database {
    db: sled::Db,

    slugs: sled::Tree,     // stores all urls
    documents: sled::Tree, // stores all docs
}

impl Database {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Result<Self, sled::Error> {
        let db = sled::Config::default()
            .use_compression(true)
            // .mode(sled::Mode::HighThroughput)
            .compression_factor(10)
            .path(path)
            .open()?;

        let slugs = db.open_tree("slugs")?;
        let documents = db.open_tree("documents")?;

        Ok(Self {
            db,
            slugs,
            documents,
        })
    }

    pub fn insert_document(&self, doc: &DocumentRecord) -> Result<DocumentHash, Error> {
        let hash = *blake3::hash(&doc.content.as_bytes()).as_bytes();
        let hash = DocumentHash(hash);
        Self::insert_and_transform::<_, _, DocumentRecord>(&self.documents, &hash, doc)?;
        Ok(hash)
    }

    // pub fn insert_document(&self, hash: &DocumentHash, doc: &DocumentRecord) -> Result<Option<DocumentRecord>, Error> {
    //     Self::insert_and_transform(&self.documents, hash, doc)
    // }

    pub fn get_document(&self, hash: &DocumentHash) -> Result<Option<DocumentRecord>, Error> {
        Self::get_and_transform(&self.documents, hash)
    }

    pub fn remove_document(&self, hash: &DocumentHash) -> Result<Option<DocumentRecord>, Error> {
        Self::remove(&self.documents, hash)
    }

    pub fn contains_document(&self, hash: &DocumentHash) -> Result<bool, Error> {
        Self::contains_key(&self.documents, hash)
    }

    pub fn insert_slug<S: AsRef<str>>(
        &self,
        slug: S,
        record: &SlugRecord,
    ) -> Result<Option<SlugRecord>, Error> {
        Self::insert_and_transform(&self.slugs, slug.as_ref(), record)
    }

    pub fn get_slug<S: AsRef<str>>(&self, slug: S) -> Result<Option<SlugRecord>, Error> {
        Self::get_and_transform(&self.slugs, slug.as_ref())
    }

    pub fn remove_slug<S: AsRef<str>>(&self, slug: S) -> Result<Option<SlugRecord>, Error> {
        Self::remove(&self.slugs, slug.as_ref())
    }

    pub fn contains_slug<S: AsRef<str>>(&self, slug: S) -> Result<bool, Error> {
        Self::contains_key(&self.slugs, slug.as_ref())
    }

    fn iter<K, V>(store: &sled::Tree) -> impl Iterator<Item = Result<(K, V), Error>>
    where
        K: FromIVec,
        V: FromIVec,
    {
        store.iter().map(|result| match result {
            Ok((k, v)) => {
                let k = K::from_ivec(&k)?;
                let v = V::from_ivec(&v)?;
                Ok::<_, Error>((k, v))
            }
            Err(e) => Err(e.into()),
        })
    }

    fn insert_and_transform<K, V, T>(
        store: &sled::Tree,
        key: K,
        value: V,
    ) -> Result<Option<T>, Error>
    where
        K: IntoIVec,
        V: IntoIVec,
        T: FromIVec,
    {
        let previous = store.insert(key.to_ivec()?, value.to_ivec()?)?;
        Ok(previous.map(|p| T::from_ivec(&p)).transpose()?)
    }

    fn get_and_transform<K, V>(store: &sled::Tree, key: K) -> Result<Option<V>, Error>
    where
        K: IntoIVec,
        V: FromIVec,
    {
        let value = store.get(key.to_ivec()?)?;
        Ok(value.map(|p| FromIVec::from_ivec(&p)).transpose()?)
    }

    fn remove<K, V>(store: &sled::Tree, key: K) -> Result<Option<V>, Error>
    where
        K: IntoIVec,
        V: FromIVec,
    {
        Ok(store
            .remove(key.to_ivec()?)?
            .map(|p| FromIVec::from_ivec(&p))
            .transpose()?)
    }

    fn contains_key<K>(store: &sled::Tree, key: K) -> Result<bool, Error>
    where
        K: IntoIVec,
    {
        store.contains_key(key.to_ivec()?).map_err(Into::into)
    }
}

pub trait IntoIVec: Sized {
    fn to_ivec(&self) -> Result<IVec, bincode::Error>;
}

pub trait FromIVec: Sized {
    fn from_ivec(ivec: &IVec) -> Result<Self, bincode::Error>;
}

impl<T> IntoIVec for T
where
    T: Serialize,
{
    fn to_ivec(&self) -> Result<IVec, bincode::Error> {
        bincode::serialize(self).map(Into::into)
    }
}

impl<T> FromIVec for T
where
    T: for<'de> Deserialize<'de>,
{
    fn from_ivec(ivec: &IVec) -> Result<Self, bincode::Error> {
        bincode::deserialize(ivec)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentHash([u8; 32]);

impl AsRef<[u8; 32]> for DocumentHash {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentRecord {
    pub content: String,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlugRecord {
    pub document_hash: DocumentHash,
    pub edit_code: String,
}
