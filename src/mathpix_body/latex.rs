use super::{CallBack, MetaData, Src};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct PostLaTeX {
    src: Src,
    formats: Vec<LaTeXFormats>,
    ocr: Option<Vec<Ocr>>,
    format_options: Option<FormatOptions>,
    skip_recrop: Option<bool>,
    // TODO: bounded 0-1. <01-05-21, kunzaatko> //
    confidence_threshold: Option<f32>,
    // TODO: this should be bounded. It is from 1-5. <01-05-21, kunzaatko> //
    beam_size: Option<u8>,
    // TODO: this should be bounded. It is from 1-beam_size. <01-05-21, kunzaatko> //
    n_best: Option<u8>,
    region: Option<Region>,
    callback: Option<CallBack>,
    metadata: Option<MetaData>,
    include_detected_alphabets: Option<bool>,
    // TODO: bounded 0-1. <01-05-21, kunzaatko> //
    auto_rotate_confidence_threshold: Option<f32>,
}

// LaTeXFormats {{{
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum LaTeXFormats {
    /// Text mode output, with math inside delimiters, eg. test \(x^2\), inline math by default
    Text,
    /// Same as text, except uses block mode math instead of inline mode when in doubt
    TextDisplay,
    /// Direct LaTeX representation of the input
    #[serde(rename = "latex_normal")]
    LaTeXNormal,
    /// Modified output to improve the visual appearance such as adding '\left' and '\right' around parenthesized expressions that contain tall expressions like subscript or superscript
    #[serde(rename = "latex_styled")]
    LaTeXStyled,
    /// Modified output for symbolic processing such as shortening operator names, replacing long division with a fraction, and converting a column of operands into a single formula
    #[serde(rename = "latex_simplified")]
    LaTeXSimplified,
    /// Output split into a list of simplified strings to help process multiple equations
    #[serde(rename = "latex_list")]
    LaTeXList,
    /// The MathML for the recognized math
    #[serde(rename = "mathml")]
    MathML,
    /// The AsciiMath for the recognized math
    #[serde(rename = "asciimath")]
    AsciiMath,
    /// A string compatible with the Wolfram Alpha engine
    Wolfram,
}
// }}}

// Ocr {{{
// TODO: Ask Mathpix if the math in mandatory in this field and there are only options "math" and ["math", "text"]. This would be implied by the docs. <22-05-21, kunzaatko> //
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum Ocr {
    /// Process math from the input
    Math,
    /// Process text from the image
    Text,
}
// }}}

// FormatOptions {{{
#[derive(Debug, Serialize)]
struct FormatOptions {
    /// Array of transformation names
    transforms: Option<Vec<Transforms>>,
    // TODO: Add the constraint of ony two stings supplied <14-05-21, kunzaatko> //
    /// [begin, end] delimiters for math mode (for example ["\(","\)"])
    math_delims: Option<Vec<String>>,
    // TODO: Add the constraint of ony two stings supplied <14-05-21, kunzaatko> //
    /// [begin, end] delimiters for displaymath mode (for example ["\(","\)"])
    displaymath_delims: Option<Vec<String>>,
}
//}}}

// Transforms {{{
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum Transforms {
    /// Omit spaces around LaTeX groups and other places where spaces are superfluous
    RmSpaces,
    /// Uses spaces instead of newlines between text lines in paragraphs
    RmNewlines,
    ///	Omit mathbb, mathbf, mathcal, and mathrm commands
    RmFonts,
    /// Replace styled commands with unstyled versions, e.g., bigoplus becomes oplus
    RmStyleSyms,
    ///	Omit text to the left or right of math
    RmText,
    /// Convert longdiv to frac
    LongFrac,
}
// }}}

// Region {{{
#[derive(Debug, Serialize)]
struct Region {
    top_left_x: Option<u32>,
    top_left_y: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
}
//}}}

// TESTS {{{
#[cfg(test)]
mod test {
    use super::{FormatOptions, LaTeXFormats, Ocr, PostLaTeX, Region, Src, Transforms};
    use reqwest::Url;
    use serde_json::{json, Value::Null};

    #[test]
    fn serialize_latexformats() {
        //{{{
        let latexformats = vec![
            LaTeXFormats::AsciiMath,
            LaTeXFormats::LaTeXList,
            LaTeXFormats::LaTeXNormal,
            LaTeXFormats::LaTeXSimplified,
            LaTeXFormats::LaTeXStyled,
            LaTeXFormats::MathML,
            LaTeXFormats::Text,
            LaTeXFormats::TextDisplay,
            LaTeXFormats::Wolfram,
        ];
        let serialized = serde_json::to_value(&latexformats).unwrap();
        let expected = json!([
            "asciimath",
            "latex_list",
            "latex_normal",
            "latex_simplified",
            "latex_styled",
            "mathml",
            "text",
            "text_display",
            "wolfram"
        ]);
        assert_eq!(serialized, expected);
    } //}}}

    #[test]
    fn serialize_ocr() {
        //{{{
        let ocr = vec![Ocr::Math, Ocr::Text];
        let serialized = serde_json::to_value(&ocr).unwrap();
        let expected = json!(["math", "text"]);
        assert_eq!(serialized, expected);
    } //}}}

    #[test]
    fn serialize_format_options() {
        //{{{
        let format_options = FormatOptions {
            transforms: Some(vec![Transforms::RmSpaces, Transforms::RmFonts]),
            displaymath_delims: Some(vec!["\\[".to_string(), "\\]".to_string()]),
            math_delims: None,
        };
        let serialized = serde_json::to_value(&format_options).unwrap();
        let expected = json!({
            "transforms": ["rm_spaces","rm_fonts"],
            "displaymath_delims" : ["\\[", "\\]"],
            "math_delims" : Null
        });
        assert_eq!(serialized, expected);
    } //}}}

    #[test]
    fn serialize_region() {
        //{{{
        let region = Region {
            top_left_x: Some(42),
            top_left_y: None,
            width: Some(40),
            height: Some(50),
        };
        let serialized = serde_json::to_value(region).unwrap();
        let expected = json!({
            "top_left_x": 42,
            "top_left_y": Null,
            "width": 40,
            "height": 50,
        });
        assert_eq!(serialized, expected);
    } //}}}

    #[test]
    fn serialize_post_latex() {
        let url = Url::parse("https://www.duckduckgo.com/").unwrap();
        let src = Src::Url(url);
        let formats = vec![
            LaTeXFormats::AsciiMath,
            LaTeXFormats::LaTeXNormal,
            LaTeXFormats::LaTeXStyled,
            LaTeXFormats::Text,
            LaTeXFormats::Wolfram,
        ];
        let ocr = Some(vec![Ocr::Math, Ocr::Text]);
        let format_options = Some(FormatOptions {
            transforms: Some(vec![Transforms::RmNewlines, Transforms::RmStyleSyms]),
            displaymath_delims: None,
            math_delims: Some(vec!["\\(".to_string(), "\\)".to_string()]),
        });
        let region = Some(Region {
            top_left_x: None,
            top_left_y: Some(40),
            width: Some(42),
            height: Some(666),
        });

        let post_latex = PostLaTeX {
            src,
            formats,
            ocr,
            format_options,
            skip_recrop: Some(false),
            confidence_threshold: Some(1.0),
            beam_size: Some(4),
            n_best: Some(3),
            region,
            callback: None,
            metadata: None,
            include_detected_alphabets: Some(true),
            auto_rotate_confidence_threshold: Some(0.5),
        };

        let serialized = serde_json::to_value(&post_latex).unwrap();

        let expected = json!({
            "src": "https://www.duckduckgo.com/",
            "formats" : ["asciimath", "latex_normal", "latex_styled", "text", "wolfram"],
            "ocr": ["math", "text"],
            "format_options": {
                "transforms": ["rm_newlines", "rm_style_syms"],
                "displaymath_delims": Null,
                "math_delims": ["\\(", "\\)"],
            },
            "skip_recrop" : false,
            "confidence_threshold" : 1.0,
            "beam_size": 4,
            "n_best": 3,
            "region": {
                "top_left_x": Null,
                "top_left_y": 40,
                "width": 42,
                "height": 666,
            },
            "callback": Null,
            "metadata": Null,
            "include_detected_alphabets": true,
            "auto_rotate_confidence_threshold": 0.5,
        });
        assert_eq!(serialized,expected);
    }
}
// }}}
