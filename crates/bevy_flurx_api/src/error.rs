use serde::{Serialize, Serializer};
use std::error::Error;
use std::fmt::Debug;

#[allow(unused)]
pub(crate) type ApiResult<V = ()> = Result<V, ApiError>;

#[derive(Debug)]
pub(crate) struct ApiError(Box<dyn Error + Send + Sync>);

impl Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.to_string().as_str())
    }
}

impl<E: Error + Send + Sync + 'static> From<E> for ApiError {
    fn from(value: E) -> Self {
        Self(Box::new(value))
    }
}

#[cfg(feature = "http")]
pub mod http {
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    pub(crate) struct DenyOrigin(pub String);

    impl Display for DenyOrigin {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{} is not a valid deny origin", self.0))
        }
    }

    impl Error for DenyOrigin {}
}

#[cfg(feature = "fs")]
pub mod fs {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};

    pub(crate) struct NotPermittedPath;

    impl NotPermittedPath {
        const MESSAGE: &'static str =
            "Try to access to any of specified files isn't permitted by the application. ";
    }

    impl Debug for NotPermittedPath {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(NotPermittedPath::MESSAGE)
        }
    }

    impl Display for NotPermittedPath {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(NotPermittedPath::MESSAGE)
        }
    }

    impl Error for NotPermittedPath {}
}
