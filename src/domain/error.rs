use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("{0} is not a valid user name.")]
    ParseUserNameError(String),
    #[error("{0} is not a valid user email.")]
    ParseUserEmailError(String),
}

