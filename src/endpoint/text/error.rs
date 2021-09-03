use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextError {
    // TODO:  <21-07-21, kunzaatko> //
    #[error("RequestError: {0}")]
    RequestError(#[from] reqwest::Error),
}
