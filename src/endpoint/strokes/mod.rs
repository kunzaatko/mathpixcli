mod error;
mod options;
mod response;

pub use super::shared_objects::request::{DataOptions, MetaData};
use error::StrokesError;
pub use options::{StrokesFormats, StrokesOptions};
use response::StrokesResponse;
use serde::Serialize;
use serde_json::Value as JsonValue;

// Strokes {{{
#[derive(Serialize, Debug)]
/// This structs contains the possible items that the _strokes_ endpoint accepts
pub struct Strokes {
    // NOTE: on construction, `is_array(&self)` should be used to check whether it is an array.
    // Also eltype should be Number (`is_number(&self)`)
    /// > Strokes in JSON with appropriate format.
    pub src: JsonValue,
    /// Configuration options for the _strokes_ endpoint
    #[serde(flatten)]
    pub options: StrokesOptions,
}
// }}}
