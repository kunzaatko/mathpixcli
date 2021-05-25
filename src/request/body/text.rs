pub use super::shared_objects::{Base64Image, DataOptions, MetaData, Src};
use serde::Serialize;

// TextBody {{{
#[derive(Serialize, Debug)]
/// This structs contains the possible items that the _text_ endpoint accepts
pub struct TextBody {
    /// Image data, or public URL where image is located
    pub src: Src,
    /// Configuration options for the _text_ endpoint
    #[serde(flatten)]
    pub options: TextBodyOptions,
} //}}}

// TextBodyOptions {{{
#[derive(Serialize, Debug)]
pub struct TextBodyOptions {
    /// Key value object
    pub metadata: Option<MetaData>,
    /// List of formats, one of `text`, `data`, `html`, `latex_styled`, see [Format Descriptions](https://docs.mathpix.com/?shell#format-descriptions)
    pub formats: Option<Vec<TextFormats>>,
    /// See [DataOptions](https://docs.mathpix.com/?shell#dataoptions-object) section, specifies outputs for `data` and `html` return fields
    pub data_options: Option<DataOptions>,
    /// Return detected alphabets
    pub include_detected_alphabets: Option<bool>,
    /// See [AlphabetsAllowed](https://docs.mathpix.com/?shell#alphabetsallowed-object) section, use this to specify which alphabets you don't want in the output
    pub alphabets_allowed: Option<AlphabetsAllowed>,
    // TODO: Add the num bounded trait (is between 0 and 1) <30-04-21, kunzaatko> //
    /// Specifies threshold for triggering confidence errors
    pub confidence_threshold: Option<f32>,
    // TODO: Add the num bounded trait (is between 0 and 1) <30-04-21, kunzaatko> //
    /// Specifies threshold for triggering confidence errors, default `0.75` (symbol level threshold)
    pub confidence_rate_threshold: Option<f32>,
    /// Specifies whether to return information segmented line by line, see [LineData](https://docs.mathpix.com/?shell#linedata-object) object section for details
    pub include_line_data: Option<bool>,
    /// Specifies whether to return information segmented word by word, see [WordData](https://docs.mathpix.com/?shell#worddata-object) object section for details
    pub include_word_data: Option<bool>,
    /// Enable experimental chemistry diagram OCR, via RDKIT normalized SMILES with `isomericSmiles=False`, included in `text` output format, via MMD SMILES syntax `<smiles>...</smiles>`
    pub include_smiles: Option<bool>,
    /// Include InChI data as XML attributes inside `<smiles>` elements, for examples `<smiles inchi="..." inchikey="...">...</smiles>`; only applies when `include_smiles` is true
    pub include_inchi: Option<bool>,
    /// Enable data extraction for geometry diagrams (currently only supports triangle diagrams); see [GeometryData](https://docs.mathpix.com/?shell#geometry-objects)
    pub include_geometry_data: Option<bool>,
    // TODO: Add the num bounded trait (is between 0 and 1) <30-04-21, kunzaatko> //
    /// Specifies threshold for auto rotating image to correct orientation; by default it is set to `0.99`, can be disabled with a value of `1` (see [Auto rotation](https://docs.mathpix.com/?shell#auto-rotation) section for details)
    pub auto_rotate_confidence_threshold: Option<f32>,
    /// Determines whether extra white space is removed from equations in `latex_styled` and `text` formats. Default is `true`.
    pub rm_spaces: Option<bool>,
    /// Determines whether font commands such as `\mathbf` and `\mathrm` are removed from equations in `latex_styled` and `text` formats. Default is `false`.
    pub rm_fonts: Option<bool>,
    /// Specifies whether numbers are always math, e.g., `Answer: \( 17 \)` instead of `Answer: 17`. Default is `false`.
    pub numbers_default_to_math: Option<bool>,
}

impl Default for TextBodyOptions {
    fn default() -> Self {
        TextBodyOptions {
            metadata: None,
            formats: None,
            data_options: None,
            include_detected_alphabets: None,
            alphabets_allowed: None,
            confidence_threshold: None,
            confidence_rate_threshold: None,
            include_line_data: None,
            include_word_data: None,
            include_smiles: None,
            include_inchi: None,
            include_geometry_data: None,
            auto_rotate_confidence_threshold: None,
            rm_spaces: None,
            rm_fonts: None,
            numbers_default_to_math: None,
        }
    }
}
// }}}

// TextFormats {{{
/// Format specifications possible for the _text_ endpoint
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TextFormats {
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
pub struct AlphabetsAllowed {
    /// English
    pub en: Option<bool>,
    /// Hindi Devangari
    pub hi: Option<bool>,
    /// Chinese
    pub zh: Option<bool>,
    /// Kana Hiragana or Katakana
    pub ja: Option<bool>,
    /// Hangul Jamo
    pub ko: Option<bool>,
    /// Russian
    pub ru: Option<bool>,
    /// Thai
    pub th: Option<bool>,
}

impl Default for AlphabetsAllowed {
    fn default() -> Self {
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
}

impl AlphabetsAllowed {
    pub fn disallow(&mut self, alphabets: Vec<String>) -> Result<(), String> {
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

    pub fn allow(&mut self, alphabets: Vec<String>) -> Result<(), String> {
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
    use super::super::shared_objects::Base64Image;
    use super::{AlphabetsAllowed, DataOptions, Src, TextBody, TextBodyOptions, TextFormats};
    use serde_json::{json, Value::Null};
    use std::convert::TryInto;
    use std::path::PathBuf;

    #[test]
    fn alphabets_allow() {
        //{{{
        let mut alphabets = AlphabetsAllowed::default();
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
        let mut alphabets = AlphabetsAllowed::default();
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
        let mut alphabets = AlphabetsAllowed::default();
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
        let mut alphabets = AlphabetsAllowed::default();
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
    fn serialize_textbody() {
        //{{{
        let image: Base64Image = PathBuf::from("./test/assets/test_encode_base64.jpg".to_string())
            .try_into()
            .unwrap();
        let mut alphabets_allowed = AlphabetsAllowed::default();
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

        let text_body_opts = TextBodyOptions {
            metadata: None,
            formats: Some(vec![TextFormats::Text, TextFormats::Data]),
            alphabets_allowed: Some(alphabets_allowed),
            auto_rotate_confidence_threshold: Some(1.),
            confidence_threshold: Some(1.),
            confidence_rate_threshold: Some(1.),
            data_options: Some(data_options),
            include_detected_alphabets: Some(true),
            include_geometry_data: Some(false),
            include_inchi: Some(true),
            include_line_data: Some(false),
            include_smiles: Some(true),
            include_word_data: Some(false),
            rm_fonts: Some(true),
            rm_spaces: Some(false),
            numbers_default_to_math: None,
        };

        let text_body = TextBody {
            src,
            options: text_body_opts,
        };
        let serialized = serde_json::to_value(&text_body).unwrap();
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
            "confidence_rate_threshold": 1.,
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
            "numbers_default_to_math": Null,
        });
        assert_eq!(serialized, expected);
    } //}}}
}
//}}}
