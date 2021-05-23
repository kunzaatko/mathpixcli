use super::{DataOptions, MetaData, Src};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PostText {
    src: Src,
    metadata: Option<MetaData>,
    formats: Option<Vec<TextFormats>>,
    data_options: Option<DataOptions>,
    include_detected_alphabets: Option<bool>,
    alphabets_allowed: Option<AlphabetsAllowed>,
    // TODO: Add the num bounded trait (is between 0 and 1) <30-04-21, kunzaatko> //
    confidence_threshold: Option<f32>,
    include_line_data: Option<bool>,
    include_word_data: Option<bool>,
    include_smiles: Option<bool>,
    include_inchi: Option<bool>,
    include_geometry_data: Option<bool>,
    // TODO: Add the num bounded trait (is between 0 and 1) <30-04-21, kunzaatko> //
    auto_rotate_confidence_threshold: Option<f32>,
    rm_spaces: Option<bool>,
    rm_fonts: Option<bool>,
    idiomatic_eqn_arrays: Option<bool>,
    numbers_default_to_math: Option<bool>,
}

// TextFormats {{{
/// Formats possible for the text endpoint
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum TextFormats {
    /// Mathpix markdown formatted text
    Text,
    /// HTML rendered from `text` via mathpix-markdown-it
    Html,
    /// Data extracte from `html` as specified in the `data_options` request parameter
    Data,
    /// Styled LaTeX, returned only in cases that the whole image can be reduces to a single
    /// equation
    #[serde(rename = "latex_styled")]
    LaTeXStyled,
}

impl ToString for TextFormats {
    fn to_string(&self) -> String {
        match self {
            TextFormats::Text => "text".to_string(),
            TextFormats::Html => "html".to_string(),
            TextFormats::Data => "data".to_string(),
            TextFormats::LaTeXStyled => "latex_styled".to_string(),
        }
    }
}
//}}}

// AlphabetsAllowed {{{
// NOTE: Serialization adds serde_json::Value::Null when None... This may not work with the API. A
// test is needed. <21-05-21, kunzaatko> //
#[derive(Debug, PartialEq, Serialize)]
struct AlphabetsAllowed {
    /// English
    en: Option<bool>,
    /// Hindi Devangari
    hi: Option<bool>,
    /// Chinese
    zh: Option<bool>,
    /// Kana Hiragana or Katakana
    ja: Option<bool>,
    /// Hangul Jamo
    ko: Option<bool>,
    /// Russian
    ru: Option<bool>,
    /// Thai
    th: Option<bool>,
}

impl AlphabetsAllowed {
    fn new() -> Self {
        AlphabetsAllowed {
            en: None,
            hi: None,
            zh: None,
            ja: None,
            ko: None,
            ru: None,
            th: None,
        }
    }

    fn disallow(&mut self, alphabets: Vec<String>) -> Result<(), String> {
        for alphabet in alphabets {
            match alphabet.as_str() {
                "en" => self.en = Some(false),
                "hi" => self.hi = Some(false),
                "zh" => self.zh = Some(false),
                "ja" => self.ja = Some(false),
                "ko" => self.ko = Some(false),
                "ru" => self.ru = Some(false),
                "th" => self.th = Some(false),
                other => {
                    return Err(format!(
                    "UnknownAlphabet: {} is not in known alphabets (en, hi, zh, ja, ko, ru, th)",
                    other
                ))
                }
            }
        }
        Ok(())
    }

    fn allow(&mut self, alphabets: Vec<String>) -> Result<(), String> {
        for alphabet in alphabets {
            match alphabet.as_str() {
                "en" => self.en = Some(true),
                "hi" => self.hi = Some(true),
                "zh" => self.zh = Some(true),
                "ja" => self.ja = Some(true),
                "ko" => self.ko = Some(true),
                "ru" => self.ru = Some(true),
                "th" => self.th = Some(true),
                other => {
                    return Err(format!(
                    "UnknownAlphabet: {} is not in known alphabets (en, hi, zh, ja, ko, ru, th)",
                    other
                ))
                }
            }
        }
        Ok(())
    }
}
// }}}

// TESTS {{{
#[cfg(test)]
mod test {
    use super::{AlphabetsAllowed, DataOptions, PostText, Src, TextFormats};
    use crate::base64image::Base64Image;
    use serde_json::{json, Value::Null};
    use std::convert::TryInto;
    use std::path::PathBuf;

    #[test]
    fn alphabets_allow() {
        //{{{
        let mut alphabets = AlphabetsAllowed::new();
        alphabets
            .allow(vec!["en".to_string(), "ru".to_string()])
            .unwrap();
        let expected = AlphabetsAllowed {
            en: Some(true),
            ru: Some(true),
            hi: None,
            ja: None,
            ko: None,
            th: None,
            zh: None,
        };
        assert_eq!(alphabets, expected)
    } //}}}

    #[test]
    fn alphabets_disallow() {
        //{{{
        let mut alphabets = AlphabetsAllowed::new();
        alphabets
            .disallow(vec!["en".to_string(), "ru".to_string()])
            .unwrap();
        let expected = AlphabetsAllowed {
            en: Some(false),
            ru: Some(false),
            hi: None,
            ja: None,
            ko: None,
            th: None,
            zh: None,
        };
        assert_eq!(alphabets, expected);
    } //}}}

    #[test]
    fn alphabets_disallow_and_allow() {
        //{{{
        let mut alphabets = AlphabetsAllowed::new();
        alphabets
            .disallow(vec!["en".to_string(), "ru".to_string()])
            .unwrap();
        alphabets
            .allow(vec!["en".to_string(), "hi".to_string()])
            .unwrap();
        let expected = AlphabetsAllowed {
            en: Some(true),
            ru: Some(false),
            hi: Some(true),
            ja: None,
            ko: None,
            th: None,
            zh: None,
        };
        assert_eq!(alphabets, expected)
    } //}}}

    #[test]
    fn serialize_alphabets() {
        //{{{
        let mut alphabets = AlphabetsAllowed::new();
        alphabets
            .disallow(vec!["en".to_string(), "ru".to_string()])
            .unwrap();
        alphabets
            .allow(vec!["en".to_string(), "hi".to_string()])
            .unwrap();
        let serialized = serde_json::to_value(&alphabets).unwrap();
        let expected = json!({
                "en": true,
                "ru": false,
                "hi": true,
                "ja": Null,
                "ko": Null,
                "th": Null,
                "zh": Null,
        });
        assert_eq!(expected, serialized);
    } //}}}

    #[test]
    fn serialize_text_formats() {
        //{{{
        let text_formats: Vec<TextFormats> = vec![
            TextFormats::Text,
            TextFormats::Html,
            TextFormats::Data,
            TextFormats::LaTeXStyled,
        ];
        let serialized = serde_json::to_value(&text_formats).unwrap();
        let expected = json!(["text", "html", "data", "latex_styled"]);
        assert_eq!(serialized, expected);
    } //}}}

    #[test]
    fn serialize_text_post() {
        //{{{
        let image: Base64Image = PathBuf::from("./test/assets/test_encode_base64.jpg".to_string())
            .try_into()
            .unwrap();
        let mut alphabets_allowed = AlphabetsAllowed::new();
        alphabets_allowed
            .allow(vec!["ru".to_string(), "en".to_string()])
            .unwrap();
        let data_options = DataOptions {
            include_asciimath: Some(true),
            include_latex: Some(false),
            include_mathml: None,
            include_svg: None,
            include_table_html: None,
            include_tsv: None,
        };
        let src = Src::Image(image);

        let post_text = PostText {
            src,
            metadata: None,
            formats: Some(vec![TextFormats::Text, TextFormats::Data]),
            alphabets_allowed: Some(alphabets_allowed),
            auto_rotate_confidence_threshold: Some(1.),
            confidence_threshold: Some(1.),
            data_options: Some(data_options),
            include_detected_alphabets: Some(true),
            include_geometry_data: Some(false),
            include_inchi: Some(true),
            include_line_data: Some(false),
            include_smiles: Some(true),
            include_word_data: Some(false),
            rm_fonts: Some(true),
            rm_spaces: Some(false),
            idiomatic_eqn_arrays: Some(true),
            numbers_default_to_math: None,
        };
        let serialized = serde_json::to_value(&post_text).unwrap();
        let expected = json!({
            "src" : "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAMCAgICAgMCAgIDAwMDBAYEBAQEBAgGBgUGCQgKCgkICQkKDA8MCgsOCwkJDRENDg8QEBEQCgwSExIQEw8QEBD/wAALCAACAAIBAREA/8QAFAABAAAAAAAAAAAAAAAAAAAACP/EABwQAAEFAQEBAAAAAAAAAAAAAAIBAwQFBgcIAP/aAAgBAQAAPwBfeevPXAt7wLmm63XD+f6PSaPH01tcXFtmYUydZTpEJp1+TIfdbJx55xwzM3DJSIiUlVVVV+//2Q==",
            "metadata": Null,
            "formats": ["text", "data"],
            "alphabets_allowed": {
                "en" : true,
                "ru" : true,
                "hi" : Null,
                "zh" : Null,
                "ja" : Null,
                "ko" : Null,
                "th" : Null,
            },
            "auto_rotate_confidence_threshold": 1.,
            "confidence_threshold": 1.,
            "data_options": {
                "include_asciimath": true,
                "include_latex": false,
                "include_mathml": Null,
                "include_svg": Null,
                "include_table_html": Null,
                "include_tsv": Null,
            },
            "include_detected_alphabets": true,
            "include_geometry_data": false,
            "include_inchi": true,
            "include_line_data": false,
            "include_smiles": true,
            "include_word_data": false,
            "rm_fonts": true,
            "rm_spaces": false,
            "idiomatic_eqn_arrays": true,
            "numbers_default_to_math": Null,
        });
        assert_eq!(serialized, expected);
    } //}}}
}
//}}}
