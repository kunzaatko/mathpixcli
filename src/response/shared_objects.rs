use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct LineData {
    pub r#type: String,
    pub subtype: Option<String>,
    pub cnt: Vec<Vec<i32>>,
    pub included: bool,
    pub error_id: Option<String>,
    pub text: Option<String>,
    pub confidence: Option<f32>,
    pub confidence_rate: Option<f32>,
    pub after_hyphen: Option<bool>,
    pub html: Option<String>,
    pub data: Option<Vec<Data>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct WordData {
    // TODO: implement <25-05-21, kunzaatko> //
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Data {
    pub r#type: String,
    pub value: String,
}

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
}

#[derive(Debug, Deserialize)]
pub struct GeometryData {
    // TODO: implement <25-05-21, kunzaatko> //
}

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
            cnt: vec![vec![859,81], vec![739,91],vec![626,91],vec![-2,66],vec![0,34],vec![739,52],vec![859,63]],
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
            cnt: vec![vec![654, 244], vec![651, 683], vec![7, 678], vec![11, 238]],
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
        assert_eq!(deserialized,expected);
    }
}
// }}}
