pub use super::shared_objects::{DataOptions, MetaData};
use serde::Serialize;
use serde_json::Value as JsonValue;

// StrokesBody {{{
#[derive(Serialize, Debug)]
/// This structs contains the possible items that the _strokes_ endpoint accepts
pub struct StrokesBody {
    // NOTE: on construction, `is_array(&self)` should be used to check whether it is an array.
    // Also eltype should be Number (`is_number(&self)`)
    /// Strokes in JSON with appropriate format.
    pub strokes: JsonValue,
    /// Key value object
    pub metadata: Option<MetaData>,
    /// List of formats, one of `text`, `data`, `html`
    pub formats: Option<Vec<String>>,
    /// see [DataOptions](https://docs.mathpix.com/?shell#dataoptions-object) section above, specifies outputs for `data` and `html` return fields
    pub data_options: Option<DataOptions>,
} //}}}
