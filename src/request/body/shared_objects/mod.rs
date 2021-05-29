use reqwest::Url;
use serde::{Serialize, Serializer};

pub mod base64image;
pub use base64image::Base64Image;

// Src {{{
#[derive(Debug)]
pub enum Src {
    Image(Base64Image),
    Url(Url),
}

impl Serialize for Src {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Src::Image(img) => img.serialize(serializer),
            Src::Url(url) => url.to_string().serialize(serializer),
        }
    }
} //}}}

// TODO: Ask mathpix what are the possibilities for MetaData <14-05-21, kunzaatko> //
#[derive(Debug, Serialize, PartialEq)]
pub struct MetaData {}

// DataOptions {{{
#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct DataOptions {
    /// Include math SVG in `html` and `data` formats
    pub include_svg: Option<bool>,
    /// Include HTML for `html` and `data` outputs (tables only)
    pub include_table_html: Option<bool>,
    /// Include math mode latex in `data` and `html`
    pub include_latex: Option<bool>,
    /// Include tab separated values (TSV) in `data` and `html` outputs (tables only)
    pub include_tsv: Option<bool>,
    /// Include asciimath in `data` and `html` outputs
    pub include_asciimath: Option<bool>,
    /// Include mathml in `data` and `html` outputs
    pub include_mathml: Option<bool>,
}

impl DataOptions {
    field_builder![include_asciimath, bool];
    field_builder![include_latex, bool];
    field_builder![include_mathml, bool];
    field_builder![include_svg, bool];
    field_builder![include_table_html, bool];
    field_builder![include_tsv, bool];
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
#[derive(Debug, Serialize)]
pub struct CallBack {
    /// URL to which to make POST callback
    post: Option<String>,
    /// Key value pairs of headers to make POST
    headers: Option<String>,
    /// Sets values of `reply` field of callback response object (see [callback response object](https://docs.mathpix.com/?shell#callback-response-object))
    reply: Option<String>,
}
//}}}

// TESTS {{{
#[cfg(test)]
mod test {
    use super::Base64Image;
    use super::{DataOptions, Src};
    use reqwest::Url;
    use serde_json::{json, Value::Null};
    use std::convert::TryInto;
    use std::path::PathBuf;

    #[test]
    fn serialize_src_url() {
        //{{{
        let url = Url::parse("https://www.duckduckgo.com/").unwrap();
        let src = Src::Url(url);
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
        let src = Src::Image(image);
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

    #[test]
    fn builder_data_options() {
        //{{{
        let mut data_options = DataOptions::default();
        data_options
            .include_asciimath(true)
            .include_latex(true)
            .include_mathml(true)
            .include_svg(false)
            .include_table_html(false)
            .include_tsv(false);
        let expected = DataOptions {
            include_asciimath: Some(true),
            include_latex: Some(true),
            include_mathml: Some(true),
            include_svg: Some(false),
            include_table_html: Some(false),
            include_tsv: Some(false),
        };
        assert_eq!(data_options, expected);
    } //}}}
}
//}}}
