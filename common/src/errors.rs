use std::fmt;

#[derive(Debug)]
pub enum AppErrorTypes {
    InvalidInput,
    InsufficientResources,
    ServerError,
}

#[derive(Debug)]
pub struct AppError {
    pub error_type: AppErrorTypes,
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_message = match self.error_type {
            AppErrorTypes::InvalidInput => "Invalid input",
            AppErrorTypes::InsufficientResources => {
                "Insufficient resources to perform the requested action"
            }
            AppErrorTypes::ServerError => "A problem with the game server occurred",
        };

        write!(f, "{}", error_message)
    }
}
