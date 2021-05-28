pub use super::shared_objects::{DetectedAlphabets, ErrorInfo, Position};
use serde::Deserialize;

// pub struct ResponseLaTeX {{{
#[derive(Debug, Deserialize)]
pub struct ResponseLaTeX {
    /// Recognized `text` format
    pub text: Option<String>,
    /// Recognized `text_display` format
    pub text_display: Option<String>,
    /// Recognized `latex_normal` format
    pub latex_normal: Option<String>,
    /// Recognized `latex_simplified` format
    pub latex_simplified: Option<String>,
    /// Recognized `latex_styled` format
    pub latex_styled: Option<String>,
    /// Recognized `latex_list` format
    pub latex_list: Option<Vec<String>>,
    /// Recognized MathML format
    pub mathml: Option<String>,
    /// Recognized AsciiMath format
    pub asciimath: Option<String>,
    /// Recognized Wolfram format
    pub wolfram: Option<String>,
    /// Position object, pixel coordinates
    pub position: Option<Position>,
    /// Detects image properties (see [image properties](https://docs.mathpix.com/?shell#image-properties))
    pub detection_list: Option<Vec<String>>,
    /// US locale error message
    pub error: Option<String>,
    /// Error info object
    pub error_info: Option<ErrorInfo>,
    /// Estimated probability 100% correct
    pub latex_confidence: Option<f32>,
    /// Estimated confidence of input quality
    pub latex_confidence_rate: Option<f32>,
    /// `n_best` results
    pub candidates: Option<Candidates>,
    /// [DetectedAlphabet](https://docs.mathpix.com/?shell#detectedalphabet-object) object
    pub detected_alphabets: Option<DetectedAlphabets>,
    /// Estimated probability that image needs to be rotated, see [Auto rotation](https://docs.mathpix.com/?shell#auto-rotation)
    pub auto_rotate_confidence: Option<f32>,
    /// Estimated angle of rotation in degrees to put image in correct orientation, see [Auto rotation](https://docs.mathpix.com/?shell#auto-rotation)
    pub auto_rotate_degrees: Option<i16>,
}
// }}}

#[derive(Debug, Deserialize)]
pub struct Candidates {}
