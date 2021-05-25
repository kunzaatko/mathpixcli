use serde::Deserialize;

// pub struct LineData {{{
/// The _v3/text_ endpoint allows customers to request line by line data by adding a `include_line_data` request parameter to the request. When this parameter is true, the response object then includes a `line_data` field which is a list of LineData objects containing information about all texual line elements detected in the image. Simply concatenating information from the response's `line_data` is enough to recreate the top level `text`, `html`, and `data` fields included in the response JSON.
///
/// Some lines are not supported by the OCR engine, like diagrams, and are therefore simply skipped by the OCR engine. Some lines contain content that is most likely extraneous, like equation numbers. Additionally, sometimes the OCR engine simply cannot recognize the line with proper confidence. In all those cases `included` field is set to `false`, as that line is certainly not part of the final result.
///
/// The following error_id's can occur here:
///
/// 1) `image_not_supported` - OCR engine doesn't accept this type of line
/// 2) `image_max_size` - line is larger than maximal size which OCR engine supports
/// 3) `math_confidence` - OCR engine failed to confidently recognize the content of the line
/// 4) `image_no_content` - line has strange spatial dimensions, e.g. height of the line is zero; this error is very unlikely to happen
#[derive(Debug, Deserialize, PartialEq)]
pub struct LineData {
    /// One of `text`, `math`, `table`, `diagram`, `equation_number`, `diagram_label`
    pub r#type: String,
    /// Either not set, or `chemistry`, or `triangle` (more diagram subtypes coming soon)
    pub subtype: Option<String>,
    /// Countour for line expressed as list of (x,y) pixel coordinate pairs
    pub cnt: Vec<(i32, i32)>,
    /// Whether this line is included in the top level OCR result
    pub included: bool,
    /// Error ID, reason why the line is not included in final result
    pub error_id: Option<String>,
    /// Text (Mathpix Markdown) for line
    pub text: Option<String>,
    /// Estimated probability 100% correct
    pub confidence: Option<f32>,
    /// Estimated confidence of input quality
    pub confidence_rate: Option<f32>,
    /// specifies if the current line occurs after the text line which ended with hyphen
    pub after_hyphen: Option<bool>,
    /// Annotated HTML output for the line
    pub html: Option<String>,
    /// List of [Data](List of Data object's) object's
    pub data: Option<Vec<Data>>,
} //}}}

// pub struct WordData {{{
/// The _v3/text_ endpoint allows customers to request word by word data by adding a `include_word_data` request parameter to the request. When this parameter is true, the response object then includes a `word_data` field which is a list of WordData objects containing information about all word level elements detected in the image.
#[derive(Debug, Deserialize, PartialEq)]
pub struct WordData {
    r#type: String,
    subtype: Option<String>,
    cnt: Vec<Vec<i32>>,
    text: Option<String>,
    latex: Option<String>,
    confidence: Option<f32>,
    confidence_rate: Option<f32>,
} //}}}

// pub struct Data {{{
/// Data objects allow extracting relevant data from an OCR result.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Data {
    /// string 	one of `asciimath`, `mathml`, `latex`, `html`, `svg`, `tsv`
    pub r#type: String,
    /// value corresponding to `type`
    pub value: String,
} //}}}

// pub struct DetectedAlphabets {{{
/// The `detected_alphabets` object in a result contains a field that is `true` of `false` for each known alphabet. The field is `true` if any characters from the alphabet are recognized in the image, regardless of whether any of the result fields contain the characters.
#[derive(Debug, Deserialize, PartialEq)]
pub struct DetectedAlphabets {
    /// English
    pub en: bool,
    /// Hindi Devangari
    pub hi: bool,
    /// Chinese
    pub zh: bool,
    /// Kana Hiragana or Katakana
    pub ja: bool,
    /// Hangul Jamo
    pub ko: bool,
    /// Russian
    pub ru: bool,
    /// Thai
    pub th: bool,
} //}}}

// pub struct GeometryData and friends {{{
#[derive(Debug, Deserialize)]
pub struct GeometryData {
    /// Position object, pixel coordinates
    position: Option<Position>,
    /// List of [ShapeData](https://docs.mathpix.com/?shell#geometry-objects) objects
    shape_list: Vec<ShapeData>,
    /// List of [LabelData](https://docs.mathpix.com/?shell#geometry-objects) objects
    label_list: Vec<LabelData>,
}

#[derive(Debug, Deserialize)]
pub struct Position {}

#[derive(Debug, Deserialize)]
pub struct ShapeData {
    ///	Type of diagram; currently only `triangle` is supported
    r#type: String,
    /// List of [VertexData](https://docs.mathpix.com/?shell#geometry-objects) objects
    vertex_list: Vec<VertexData>,
}

#[derive(Debug, Deserialize)]
pub struct VertexData {
    /// x-pixel coordinate for the vertex, counting from top left
    x: i32,
    /// y-pixel coordinate for the vertex, counting from top left
    y: i32,
    /// List of indices the vertex is connected to, in [ShapeData](https://docs.mathpix.com/?shell#geometry-objects).vertex_list (0 based indexing is used)
    edge_list: i32,
}

#[derive(Debug, Deserialize)]
pub struct LabelData {
    /// Position object, pixel coordinates
    position: Position,
    /// `text` output for OCR-ed label
    text: String,
    /// `latex` output for OCR-ed label
    latex: String,
    /// Estimated probability 100% correct
    confidence: Option<f32>,
    /// Estimated confidence of input quality
    confidence_rate: Option<f32>,
}
// }}}

#[derive(Debug, Deserialize)]
pub struct ErrorInfo {
    // TODO: implement <25-05-21, kunzaatko> //
}

// TESTS {{{
#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize_line_data_1() {
        //{{{
        let response = json!({
            "type": "text",
            "cnt": [
                [
                    859,
                    81
                ],
                [
                    739,
                    91
                ],
                [
                    626,
                    91
                ],
                [
                    -2,
                    66
                ],
                [
                    0,
                    34
                ],
                [
                    739,
                    52
                ],
                [
                    859,
                    63
                ]
            ],
            "included": true,
            "text": "Equivalent resistance between points \\( \\mathrm{A} \\& \\mathrm{B} \\) in the adjacent circuit is",
            "after_hyphen": false,
            "confidence": 0.651358435330524,
            "confidence_rate": 0.9948483133235457
        });
        let deserialized: LineData = serde_json::from_value(response).unwrap();

        let expected = LineData{
            r#type: "text".to_string(),
            cnt: vec![(859,81), (739,91),(626,91),(-2,66),(0,34),(739,52),(859,63)],
            included: true,
            text: Some("Equivalent resistance between points \\( \\mathrm{A} \\& \\mathrm{B} \\) in the adjacent circuit is".to_string()),
            after_hyphen: Some(false),
            confidence: Some(0.651358435330524),
            confidence_rate: Some(0.9948483133235457),
            data: None,
            error_id: None,
            html: None,
            subtype: None,
        };
        assert_eq!(deserialized, expected);
    } //}}}

    #[test]
    fn deserialize_line_data_2() {
        //{{{
        let response = json!({
            "type": "diagram",
            "cnt": [
                [
                    654,
                    244
                ],
                [
                    651,
                    683
                ],
                [
                    7,
                    678
                ],
                [
                    11,
                    238
                ]
            ],
            "included": false,
            "error_id": "image_not_supported"
        });
        let expected = LineData {
            r#type: "diagram".to_string(),
            after_hyphen: None,
            cnt: vec![(654, 244), (651, 683), (7, 678), (11, 238)],
            confidence: None,
            confidence_rate: None,
            included: false,
            error_id: Some("image_not_supported".to_string()),
            data: None,
            subtype: None,
            html: None,
            text: None,
        };
        let deserialized: LineData = serde_json::from_value(response).unwrap();
        assert_eq!(deserialized, expected);
    } //}}}
}
// }}}
