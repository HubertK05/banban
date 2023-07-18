use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Unexpected(anyhow::Error)
}

impl From<anyhow::Error> for AppError {
    fn from(val: anyhow::Error) -> Self {
        error!("{val:?}");
        Self::Unexpected(val)
    }
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
