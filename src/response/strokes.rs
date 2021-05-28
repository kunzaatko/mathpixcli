pub use super::shared_objects::Data;
use serde::Deserialize;

// pub struct ResponseStrokes {{{
#[derive(Debug, Deserialize)]
pub struct ResponseStrokes {
    /// Recognized `text` format, if such is found
    pub text: Option<String>,
    /// Estimated probability 100% correct
    pub confidence: Option<f32>,
    /// Estimated confidence of input quality
    pub confidence_rate: Option<f32>,
    /// List of [data objects](https://docs.mathpix.com/?shell#data-object)
    pub data: Option<Vec<Data>>,
    /// Annotated HTML output
    pub html: Option<String>,
}
// }}}
