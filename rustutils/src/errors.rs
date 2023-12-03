use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct GeneralError {
    pub name: String,
    pub message: String,
}

impl GeneralError {
    pub fn new(name: &str, message: &str) -> GeneralError {
        GeneralError {
            name: name.to_string(),
            message: message.to_string(),
        }
    }
    pub fn boxed(name: &str, message: &str) -> Box<GeneralError> {
        Box::new(Self::new(name, message))
    }
}

impl Debug for GeneralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for GeneralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{{ GeneralError: name={:?} message={:?} }}",
            self.name, self.message
        ))
    }
}

impl Error for GeneralError {}
