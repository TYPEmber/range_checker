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
