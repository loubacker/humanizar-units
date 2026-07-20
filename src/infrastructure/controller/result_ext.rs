use axum::http::Uri;

use crate::domain::exception::UnitException;
use crate::infrastructure::controller::handler::UnitHttpError;

pub trait ApplicationResultExt<T> {
    fn for_uri(self, uri: &Uri) -> Result<T, UnitHttpError>;
}

impl<T> ApplicationResultExt<T> for Result<T, UnitException> {
    fn for_uri(self, uri: &Uri) -> Result<T, UnitHttpError> {
        self.map_err(|exception| UnitHttpError::new(exception, uri.path()))
    }
}
