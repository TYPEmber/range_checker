use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("check failed! check_statement: {check_statement}, failed value: {ident} = {value}")]
    CheckFailed {
        ident: String,
        value: String,
        check_statement: String,
    },
    #[error("fallback success! check_statement: {check_statement}, failed value: {ident} = {value}, fallback to: {fallback}")]
    Fallback {
        ident: String,
        value: String,
        check_statement: String,
        fallback: String,
    },
}

pub trait Check {
    fn check(&self) -> Result<(), ()>;
    fn check_with_fallback(&mut self) -> Result<(), ()>;
}

pub trait CheckVerbose {
    fn check(&self) -> Result<(), Vec<Error>>;
    fn check_with_fallback(&mut self) ->  Result<Vec<Error>, Vec<Error>>;
}
