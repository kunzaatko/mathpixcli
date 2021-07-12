pub use super::shared_objects::{DataOptions, MetaData};
use serde::Serialize;
use serde_json::Value as JsonValue;

// StrokesBody {{{
#[derive(Serialize, Debug)]
/// This structs contains the possible items that the _strokes_ endpoint accepts
pub struct StrokesBody {
    // NOTE: on construction, `is_array(&self)` should be used to check whether it is an array.
    // Also eltype should be Number (`is_number(&self)`)
    /// > Strokes in JSON with appropriate format.
    pub strokes: JsonValue,
    /// Configuration options for the _strokes_ endpoint
    #[serde(flatten)]
    pub options: StrokesBodyOptions,
}
// }}}

// StrokesBodyOptions {{{
#[derive(Serialize, Debug)]
pub struct StrokesBodyOptions {
    /// > Key value object
    pub metadata: Option<MetaData>,
    /// > List of formats, one of `text`, `data`, `html`
    pub formats: Option<Vec<StrokesFormats>>,
    /// > see [DataOptions](https://docs.mathpix.com/?shell#dataoptions-object) section above, specifies outputs for `data` and `html` return fields
    pub data_options: Option<DataOptions>,
}

impl Default for StrokesBodyOptions {
    fn default() -> Self {
        StrokesBodyOptions {
            metadata: None,
            formats: None,
            data_options: None,
        }
    }
}
//}}}

// StrokesFormats {{{
/// Format specifications possible for the _strokes_ endpoint
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StrokesFormats {
    /// > Mathpix markdown formatted text
    Text,
    /// > HTML rendered from `text` via mathpix-markdown-it
    Html,
    /// > Data extracte from `html` as specified in the `data_options` request parameter
    Data,
}
// }}}

// TESTS {{{
#[cfg(test)]
mod test {
    use super::StrokesFormats;
    use serde_json::json;

    #[test]
    fn serialize_strokes_formats() {
        //{{{
        let strokes_formats: Vec<StrokesFormats> = vec![
            StrokesFormats::Text,
            StrokesFormats::Html,
            StrokesFormats::Data,
        ];
        let serialized = serde_json::to_value(&strokes_formats).unwrap();
        let expected = json!(["text", "html", "data"]);
        assert_eq!(serialized, expected);
    } //}}}
}
//}}}
