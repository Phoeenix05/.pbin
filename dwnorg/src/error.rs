#![allow(unused)]

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid: {0}")]
    Invalid(String),

    #[error("{0}")]
    FsExtra(#[from] fs_extra::error::Error),
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
