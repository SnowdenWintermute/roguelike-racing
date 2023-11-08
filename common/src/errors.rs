use std::convert::Infallible;
use std::fmt;
use std::io::Write;
use std::io::{self};

#[derive(Debug)]
pub enum AppErrorTypes {
    InvalidInput,
    InsufficientResources,
    ServerError,
    ClientError,
    IO,
}

impl fmt::Display for AppErrorTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_string = match self {
            AppErrorTypes::InvalidInput => "Invalid Input",
            AppErrorTypes::ServerError => "Server Error",
            AppErrorTypes::ClientError => "Client Error",
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
            AppErrorTypes::ClientError => "Client error",
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

impl From<serde_cbor::Error> for AppError {
    fn from(error: serde_cbor::Error) -> Self {
        AppError {
            error_type: AppErrorTypes::ServerError,
            message: "Error serializing with serde_cbor".to_string(),
        }
    }
}
