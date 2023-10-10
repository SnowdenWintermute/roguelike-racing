use std::{
    fmt,
    io::{self, Write},
};

#[derive(Debug)]
pub enum AppErrorTypes {
    InvalidInput,
    InsufficientResources,
    ServerError,
    IO,
}

impl fmt::Display for AppErrorTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_string = match self {
            AppErrorTypes::InvalidInput => "Invalid Input",
            AppErrorTypes::ServerError => "Server Error",
            AppErrorTypes::InsufficientResources => "InsufficientResources",
            AppErrorTypes::IO => "IO",
        };
        write!(f, "{}", as_string)
    }
}

#[derive(Debug)]
pub struct AppError {
    pub error_type: AppErrorTypes,
    pub message: String,
}

const GENERIC_SERVER_ERROR_MESSAGE: &str = "A problem with the game server occurred";

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_message = match self.error_type {
            AppErrorTypes::InvalidInput => "Invalid input",
            AppErrorTypes::InsufficientResources => {
                "Insufficient resources to perform the requested action"
            }
            AppErrorTypes::ServerError => GENERIC_SERVER_ERROR_MESSAGE,
            AppErrorTypes::IO => GENERIC_SERVER_ERROR_MESSAGE,
        };

        write!(f, "{}", error_message)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            error_type: AppErrorTypes::IO,
            message: error.to_string(),
        }
    }
}
