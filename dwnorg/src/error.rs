#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid file name: {0}")]
    InvalidName(String),

    #[error("failed to move file {0} from {1} to {2}")]
    FileMoveFail(String, String, String),
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
