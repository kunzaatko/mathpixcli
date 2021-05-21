use super::{DataOptions, MetaData, Src};
use serde::Serialize;

pub struct PostText {
    src: Src,
    metadata: Option<MetaData>,
    // TODO: This should instead be a list of enums that specifies the allowed formats.
    // Formats type should be implemented in super. <01-05-21, kunzaatko> //
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
    auto_rotate_confidence_threchold: Option<f32>,
    rm_spaces: Option<bool>,
    rm_fonts: Option<bool>,
    numbers_default_to_math: Option<bool>,
}

// TextFormats {{{
/// Formats possible for the text endpoint
enum TextFormats {
    /// Mathpix markdown formatted text
    Text,
    /// HTML rendered from `text` via mathpix-markdown-it
    Html,
    /// Data extracte from `html` as specified in the `data_options` request parameter
    Data,
    /// Styled Latex, returned only in cases that the whole image can be reduces to a single
    /// equation
    LatexStyled,
}

impl ToString for TextFormats {
    fn to_string(&self) -> String {
        match self {
            TextFormats::Text => "text".to_string(),
            TextFormats::Html => "html".to_string(),
            TextFormats::Data => "data".to_string(),
            TextFormats::LatexStyled => "latex_styled".to_string(),
        }
    }
}
//}}}

// AlphabetsAllowed {{{
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
    use super::AlphabetsAllowed;
    use serde_json::json;

    #[test]
    fn alphabets_allow() {
        //{{{
        let mut alphabets = AlphabetsAllowed::new();
        alphabets
            .allow(vec!["en".to_string(), "ru".to_string()])
            .unwrap();
        assert_eq!(
            alphabets,
            AlphabetsAllowed {
                en: Some(true),
                ru: Some(true),
                hi: None,
                ja: None,
                ko: None,
                th: None,
                zh: None,
            }
        )
    } //}}}

    #[test]
    fn alphabets_disallow() {
        //{{{
        let mut alphabets = AlphabetsAllowed::new();
        alphabets
            .disallow(vec!["en".to_string(), "ru".to_string()])
            .unwrap();
        assert_eq!(
            alphabets,
            AlphabetsAllowed {
                en: Some(false),
                ru: Some(false),
                hi: None,
                ja: None,
                ko: None,
                th: None,
                zh: None,
            }
        )
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
        assert_eq!(
            alphabets,
            AlphabetsAllowed {
                en: Some(true),
                ru: Some(false),
                hi: Some(true),
                ja: None,
                ko: None,
                th: None,
                zh: None,
            }
        )
    } //}}}

    #[test]
    fn alphabets_serialization() {
        let mut alphabets = AlphabetsAllowed::new();
        alphabets
            .disallow(vec!["en".to_string(), "ru".to_string()])
            .unwrap();
        alphabets
            .allow(vec!["en".to_string(), "hi".to_string()])
            .unwrap();
        let serialized = serde_json::to_value(&alphabets).unwrap();
        let acctual = json!({
                "en": true,
                "ru": false,
                "hi": true,
                "ja": serde_json::Value::Null,
                "ko": serde_json::Value::Null,
                "th": serde_json::Value::Null,
                "zh": serde_json::Value::Null,
        });
        assert_eq!(acctual, serialized);
    }
}
//}}}
