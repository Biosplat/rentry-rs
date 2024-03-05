use thiserror::Error;



#[derive(Debug, Error)]
pub enum Error {
    #[error("Sled Error: {0}")]
    Sled(#[from] sled::Error),

    #[error("Bincode Error: {0}")]
    Bincode(#[from] bincode::Error)
}