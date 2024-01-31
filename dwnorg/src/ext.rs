#[derive(Debug, Clone)]
pub(crate) struct PathBuf(pub ::std::path::PathBuf);

impl ::std::fmt::Display for PathBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0.display())
    }
}

impl<'s> From<&'s ::std::ffi::OsStr> for PathBuf {
    fn from(value: &'s ::std::ffi::OsStr) -> Self {
        Self(value.to_string_lossy().to_string().into())
    }
}

impl TryFrom<Option<::std::path::PathBuf>> for PathBuf {
    type Error = crate::error::Error;

    fn try_from(value: Option<::std::path::PathBuf>) -> Result<Self, Self::Error> {
        match value {
            Some(buf) => Ok(Self(buf)),
            None => Err(crate::error::Error::Invalid(
                "could not find download directory".into(),
            )),
        }
    }
}
