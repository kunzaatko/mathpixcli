use super::{DataOptions, MetaData};
use json::JsonValue;

pub struct StrokesRequest {
    // NOTE: on construction, `is_array(&self)` should be used to check whether it is an array.
    // Also eltype should be Number (`is_number(&self)`)
    strokes: JsonValue,
    metadata: Option<MetaData>,
    formats: Option<Vec<String>>,
    data_options: Option<DataOptions>,
}