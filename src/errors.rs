use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileNameError;



impl Display for FileNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "filename should be provided in order to write file")
    }
}

impl Error for FileNameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
    fn description(&self) -> &str {
        "cannot save file without filename"
    }
}
