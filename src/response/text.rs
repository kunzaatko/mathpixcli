pub use super::shared_objects::{
    Data, DetectedAlphabets, ErrorInfo, GeometryData, LineData, WordData,
};
use serde::Deserialize;


#[derive(Debug,Deserialize)]
pub struct ResponseText {
    pub request_id: String,
    pub text: Option<String>,
    pub latex_styled: Option<String>,
    pub confidence: Option<f32>,
    pub confidence_rate: Option<f32>,
    pub line_data: Option<Vec<LineData>>,
    pub word_data: Option<Vec<WordData>>,
    pub data: Option<Vec<Data>>,
    pub html: Option<String>,
    pub detected_alphabets: Option<DetectedAlphabets>,
    pub is_printed: Option<bool>,
    pub is_handwritten: Option<bool>,
    pub auto_rotate_confidence: Option<f32>,
    pub geometry_data: Option<GeometryData>,
    pub auto_rotate_degrees: Option<i16>,
    pub error: Option<String>,
    pub error_info: Option<ErrorInfo>,
}
