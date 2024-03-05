use std::{error::Error as ErrorTrait, fmt};

#[derive(Debug)]
pub enum Error {
    SyncError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Error::SyncError(s) => s,
        };

        write!(f, "{}", msg)
    }
}

impl ErrorTrait for Error {
    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn ErrorTrait> {
        self.source()
    }
}
