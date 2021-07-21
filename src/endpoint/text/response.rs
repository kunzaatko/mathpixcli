pub use super::super::shared_objects::response::{
    Data, DetectedAlphabets, ErrorInfo, GeometryData, LineData, WordData,
};
use serde::Deserialize;

// pub struct TextResponse {{{
#[derive(Debug, Deserialize)]
pub struct TextResponse {
    /// Request ID, for debugging purposes
    pub request_id: String,
    /// Recognized `text` format, if such is found
    pub text: Option<String>,
    /// Math Latex string of math equation, if the image is of a single equation
    pub latex_styled: Option<String>,
    /// Estimated probability 100% correct
    pub confidence: Option<f32>,
    /// Estimated confidence of input quality
    pub confidence_rate: Option<f32>,
    /// List of [LineData](https://docs.mathpix.com/?shell#linedata-object) objects
    pub line_data: Option<Vec<LineData>>,
    /// List of [WordData](https://docs.mathpix.com/?shell#worddata-object) objects
    pub word_data: Option<Vec<WordData>>,
    /// List of [Data](https://docs.mathpix.com/?shell#data-object) objects
    pub data: Option<Vec<Data>>,
    /// Annotated HTML output
    pub html: Option<String>,
    /// [DetectedAlphabet](https://docs.mathpix.com/?shell#detectedalphabet-object) object
    pub detected_alphabets: Option<DetectedAlphabets>,
    /// Specifies if printed content was detected in an image
    pub is_printed: Option<bool>,
    /// Specifies if handwritten content was detected in an image
    pub is_handwritten: Option<bool>,
    /// Estimated probability that image needs to be rotated, see [Auto rotation](https://docs.mathpix.com/?shell#auto-rotation)
    pub auto_rotate_confidence: Option<f32>,
    /// List of [GeometryData](https://docs.mathpix.com/?shell#geometry-objects) objects
    pub geometry_data: Option<Vec<GeometryData>>,
    /// Estimated angle of rotation in degrees to put image in correct orientation, see [Auto rotation](https://docs.mathpix.com/?shell#auto-rotation)
    pub auto_rotate_degrees: Option<i16>,
    /// US locale error message
    pub error: Option<String>,
    /// Error info object
    pub error_info: Option<ErrorInfo>,
} //}}}
