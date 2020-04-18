use std::{error, fmt};

#[derive(Debug)]
pub struct Error {
    message: String,
    source: Option<Box<dyn error::Error>>,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.source {
            Some(err) => write!(f, "{}. Source error: {}", self.message, err),
            None => write!(f, "{}", self.message),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.source {
            Some(ref err) => Some(&**err),
            None => None,
        }
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self {
            message,
            source: None,
        }
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }
}

// impl From<Box<dyn error::Error>> for Error {
//     fn from(err: Box<dyn error::Error>) -> Self {
//         Self {
//             message: "Unexpected error occurred".into(),
//             source: Some(err),
//         }
//     }
// }

impl<E: Into<Box<dyn error::Error>>> From<E> for Error {
    fn from(err: E) -> Self {
        Self {
            message: "Unexpected error occurred".into(),
            source: Some(err),
        }
    }
}

impl<E: error::Error + 'static> From<(&str, E)> for Error {
    fn from((message, err): (&str, E)) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(err)),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
