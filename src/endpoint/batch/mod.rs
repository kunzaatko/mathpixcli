mod error;
mod options;
mod response;

use error::BatchError;
pub use options::BatchOptions;
use response::BatchResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BatchBody {
    // TODO:  <21-07-21, kunzaatko> //
}
