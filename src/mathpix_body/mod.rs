use super::base64image::Base64Image;
use reqwest::Url;
use serde::{Serialize, Serializer};

mod batch;
mod latex;
mod pdf;
mod strokes;
mod text;

use self::batch::PostBatch;
use self::latex::PostLaTeX;
use self::pdf::PostPDF;
use self::strokes::PostStrokes;
use self::text::PostText;

// TODO: Add docs <30-04-21, kunzaatko> //
pub enum MathpixPost {
    Text(PostText),
    PDF(PostPDF),
    Strokes(PostStrokes),
    Batch(PostBatch),
    LaTeX(PostLaTeX),
}

// this will be very usefull: https://serde.rs/container-attrs.html#into
// TODO: Serialize should be implemented manually to convert pictures to base64 Strings and urls
// to standard Strings <22-05-21, kunzaatko> //
// Src {{{
#[derive(Debug)]
enum Src {
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
#[derive(Debug, Serialize)]
struct MetaData {}

// DataOptions {{{
#[derive(Debug, Serialize)]
struct DataOptions {
    /// Include math SVG in `html` and `data` formats
    include_svg: Option<bool>,
    /// Include HTML for `html` and `data` outputs (tables only)
    include_table_html: Option<bool>,
    /// Include math mode latex in `data` and `html`
    include_latex: Option<bool>,
    /// Include tab separated values (TSV) in `data` and `html` outputs (tables only)
    include_tsv: Option<bool>,
    /// Include asciimath in `data` and `html` outputs
    include_asciimath: Option<bool>,
    /// Include mathml in `data` and `html` outputs
    include_mathml: Option<bool>,
}
//}}}

// FormatOptions {{{
#[derive(Debug, Serialize)]
struct FormatOptions {
    /// Array of transformation names
    transforms: Vec<String>,
    // TODO: Add the constraint of ony two stings supplied <14-05-21, kunzaatko> //
    /// [begin, end] delimiters for math mode (for example ["\(","\)"])
    math_delims: Vec<String>,
    // TODO: Add the constraint of ony two stings supplied <14-05-21, kunzaatko> //
    /// [begin, end] delimiters for displaymath mode (for example ["\(","\)"])
    dispaymath_delims: Vec<String>,
}
//}}}

// Region {{{
#[derive(Debug, Serialize)]
struct Region {
    top_left_x: Option<u32>,
    top_left_y: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
}
//}}}

// CallBack {{{
#[derive(Debug, Serialize)]
struct CallBack {
    /// URL to which to make POST callback
    post: Option<String>,
    /// Key value pairs of headers to make POST
    headers: Option<String>,
    /// Sets values of `reply` field of callbakc response object (see callback response object)
    reply: Option<String>,
}
//}}}

// TESTS {{{
#[cfg(test)]
mod test {
    use super::{DataOptions, Src};
    use reqwest::Url;
    use serde_json::{json, Value::Null};

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
    fn serialize_data_options() {
        //{{{
        let data_options = DataOptions{
            include_asciimath : Some(true),
            include_latex : Some(false),
            include_mathml  : None,
            include_svg : None,
            include_table_html : None,
            include_tsv : None,
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
    }//}}}
}
//}}}
