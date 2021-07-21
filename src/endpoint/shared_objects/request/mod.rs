use reqwest::Url;
use serde::{Serialize, Serializer};

pub mod base64image;
pub use base64image::Base64Image;

// ImageSrc {{{
#[derive(Debug)]
pub enum ImageSrc {
    Image(Base64Image),
    Url(Url),
}

impl Serialize for ImageSrc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ImageSrc::Image(img) => img.serialize(serializer),
            ImageSrc::Url(url) => url.to_string().serialize(serializer),
        }
    }
} //}}}

// TODO: Ask mathpix what are the possibilities for MetaData <14-05-21, kunzaatko> //
#[derive(Debug, Serialize, PartialEq)]
pub struct MetaData {}

// DataOptions {{{
#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct DataOptions {
    /// > Include math SVG in `html` and `data` formats
    pub include_svg: Option<bool>,
    /// > Include HTML for `html` and `data` outputs (tables only)
    pub include_table_html: Option<bool>,
    /// > Include math mode latex in `data` and `html`
    pub include_latex: Option<bool>,
    /// > Include tab separated values (TSV) in `data` and `html` outputs (tables only)
    pub include_tsv: Option<bool>,
    /// > Include asciimath in `data` and `html` outputs
    pub include_asciimath: Option<bool>,
    /// > Include mathml in `data` and `html` outputs
    pub include_mathml: Option<bool>,
}

impl DataOptions {
    // TODO:  <21-07-21, kunzaatko> //
}

impl Default for DataOptions {
    fn default() -> Self {
        DataOptions {
            include_asciimath: None,
            include_latex: None,
            include_mathml: None,
            include_svg: None,
            include_table_html: None,
            include_tsv: None,
        }
    }
}

//}}}

// CallBack {{{
#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct CallBack {
    /// > URL to which to make POST callback
    pub post: Option<String>,
    /// > Key value pairs of headers to make POST
    pub headers: Option<String>,
    /// > Sets values of `reply` field of callback response object (see [callback response object](https://docs.mathpix.com/?shell#callback-response-object))
    pub reply: Option<String>,
}
//}}}

// AlphabetsAllowed {{{
// NOTE: Serialization adds serde_json::Value::Null when None... This may not work with the API. A
// test is needed. <21-05-21, kunzaatko> //
#[derive(Debug, PartialEq, Serialize, Clone)]
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
    use super::Base64Image;
    use super::{DataOptions, ImageSrc};
    use reqwest::Url;
    use serde_json::{json, Value::Null};
    use std::convert::TryInto;
    use std::path::PathBuf;

    #[test]
    fn serialize_src_url() {
        //{{{
        let url = Url::parse("https://www.duckduckgo.com/").unwrap();
        let src = ImageSrc::Url(url);
        let serialized = serde_json::to_value(&src).unwrap();
        let acctual = json!("https://www.duckduckgo.com/");
        assert_eq!(serialized, acctual);
    } //}}}

    #[test]
    fn serialize_src_image() {
        //{{{
        let image: Base64Image = PathBuf::from("./test/assets/test_encode_base64.jpg".to_string())
            .try_into()
            .unwrap();
        let src = ImageSrc::Image(image);
        let serialized = serde_json::to_value(&src).unwrap();
        let acctual = json!("data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAMCAgICAgMCAgIDAwMDBAYEBAQEBAgGBgUGCQgKCgkICQkKDA8MCgsOCwkJDRENDg8QEBEQCgwSExIQEw8QEBD/wAALCAACAAIBAREA/8QAFAABAAAAAAAAAAAAAAAAAAAACP/EABwQAAEFAQEBAAAAAAAAAAAAAAIBAwQFBgcIAP/aAAgBAQAAPwBfeevPXAt7wLmm63XD+f6PSaPH01tcXFtmYUydZTpEJp1+TIfdbJx55xwzM3DJSIiUlVVVV+//2Q==");
        assert_eq!(serialized, acctual);
    } //}}}

    #[test]
    fn serialize_data_options() {
        //{{{
        let data_options = DataOptions {
            include_asciimath: Some(true),
            include_latex: Some(false),
            include_mathml: None,
            include_svg: None,
            include_table_html: None,
            include_tsv: None,
        };
        let serialized = serde_json::to_value(&data_options).unwrap();
        let acctual = json!({
            "include_asciimath" : true,
            "include_latex" : false,
            "include_mathml"  : Null,
            "include_svg" : Null,
            "include_table_html" : Null,
            "include_tsv" : Null,
        });
        assert_eq!(serialized, acctual);
    } //}}}
}
//}}}
