use super::super::shared_objects::request::{CallBack, ImageSrc, MetaData};
use serde::Serialize;

// LaTeXOptions {{{
#[derive(Debug, Serialize, PartialEq)]
pub struct LaTeXOptions {
    /// > Process only math `["math"]` or both math and text `["math", "text"]`
    pub ocr: Option<Vec<Ocr>>,
    /// > Options for specific formats (see [Formatting](https://docs.mathpix.com/?shell#format-options) section)
    pub format_options: Option<FormatOptions>,
    /// > Force algorithm to consider whole image
    pub skip_recrop: Option<bool>,
    // TODO: bounded 0-1. <01-05-21, kunzaatko> //
    /// > Set threshold for triggering confidence errors
    pub confidence_threshold: Option<f32>,
    // TODO: this should be bounded. It is from 1-5. <01-05-21, kunzaatko> //
    /// > Number of results to consider during recognition (an integer 1-5)
    pub beam_size: Option<u8>,
    // TODO: this should be bounded. It is from 1-beam_size. <01-05-21, kunzaatko> //
    /// > Number of highest-confidence results to return (an integer 1-`beam_size`)
    pub n_best: Option<u8>,
    /// > Specify the image area with the pixel coordinates `top_left_x`, `top_left_y`, `width`, and `height`
    pub region: Option<Region>,
    /// > Callback request object
    pub callback: Option<CallBack>,
    /// > Key value object
    pub metadata: Option<MetaData>,
    /// > Return detected alphabets
    pub include_detected_alphabets: Option<bool>,
    // TODO: bounded 0-1. <01-05-21, kunzaatko> //
    /// > Specifies threshold for auto rotating image to correct orientation; by default it is set to `0.99`, can be disabled with a value of `1` (see [Auto rotation](https://docs.mathpix.com/?shell#auto-rotation) section for details)
    pub auto_rotate_confidence_threshold: Option<f32>,
}

impl Default for LaTeXOptions {
    fn default() -> Self {
        LaTeXOptions {
            ocr: None,
            format_options: None,
            skip_recrop: None,
            confidence_threshold: None,
            beam_size: None,
            n_best: None,
            region: None,
            callback: None,
            metadata: None,
            include_detected_alphabets: None,
            auto_rotate_confidence_threshold: None,
        }
    }
}

impl LaTeXOptions {
    field_builder![ocr, Vec<Ocr>];
    field_builder![format_options, FormatOptions];
    field_builder![skip_recrop, bool];
    field_builder![confidence_threshold, f32];
    field_builder![beam_size, u8];
    field_builder![n_best, u8];
    field_builder![region, Region];
    field_builder![callback, CallBack];
    field_builder![metadata, MetaData];
    field_builder![include_detected_alphabets, bool];
    field_builder![auto_rotate_confidence_threshold, f32];
}
// }}}

// LaTeXFormats {{{
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LaTeXFormats {
    /// > Text mode output, with math inside delimiters, eg. test \(x^2\), inline math by default
    Text,
    /// > Same as text, except uses block mode math instead of inline mode when in doubt
    TextDisplay,
    /// > Direct LaTeX representation of the input
    #[serde(rename = "latex_normal")]
    LaTeXNormal,
    /// > Modified output to improve the visual appearance such as adding '\left' and '\right' around parenthesized expressions that contain tall expressions like subscript or superscript
    #[serde(rename = "latex_styled")]
    LaTeXStyled,
    /// > Modified output for symbolic processing such as shortening operator names, replacing long division with a fraction, and converting a column of operands into a single formula
    #[serde(rename = "latex_simplified")]
    LaTeXSimplified,
    /// > Output split into a list of simplified strings to help process multiple equations
    #[serde(rename = "latex_list")]
    LaTeXList,
    /// > The MathML for the recognized math
    #[serde(rename = "mathml")]
    MathML,
    /// > The AsciiMath for the recognized math
    #[serde(rename = "asciimath")]
    AsciiMath,
    /// > A string compatible with the Wolfram Alpha engine
    Wolfram,
}
// }}}

// Ocr {{{
// TODO: Ask Mathpix if the math in mandatory in this field and there are only options "math" and ["math", "text"]. This would be implied by the docs. <22-05-21, kunzaatko> //
#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Ocr {
    /// > Process math from the input
    Math,
    /// > Process text from the image
    Text,
}
// }}}

// FormatOptions {{{
#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct FormatOptions {
    /// > Array of transformation names
    pub transforms: Option<Vec<Transforms>>,
    // TODO: Add the constraint of ony two stings supplied <14-05-21, kunzaatko> //
    /// > [begin, end] delimiters for math mode (for example `["\(","\)"]`)
    pub math_delims: Option<Vec<String>>,
    // TODO: Add the constraint of ony two stings supplied <14-05-21, kunzaatko> //
    /// > [begin, end] delimiters for displaymath mode (for example `["\(","\)"]`)
    pub displaymath_delims: Option<Vec<String>>,
}
//}}}

// Transforms {{{
#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Transforms {
    /// > Omit spaces around LaTeX groups and other places where spaces are superfluous
    RmSpaces,
    /// > Uses spaces instead of newlines between text lines in paragraphs
    RmNewlines,
    /// > Omit mathbb, mathbf, mathcal, and mathrm commands
    RmFonts,
    /// > Replace styled commands with unstyled versions, e.g., bigoplus becomes oplus
    RmStyleSyms,
    /// > Omit text to the left or right of math
    RmText,
    /// > Convert longdiv to frac
    LongFrac,
}
// }}}

// Region {{{
#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct Region {
    pub top_left_x: Option<u32>,
    pub top_left_y: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}
//}}}

// TESTS {{{
#[cfg(test)]
mod test {
    use super::super::*;
    use super::*;
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
    fn serialize_latex() {
        //{{{
        let url = Url::parse("https://www.duckduckgo.com/").unwrap();
        let src = ImageSrc::Url(url);
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

        let latex_body_opts = LaTeXOptions {
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

        let latex_body = LaTeX {
            src,
            formats,
            options: latex_body_opts,
        };
        let serialized = serde_json::to_value(&latex_body).unwrap();

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
        assert_eq!(serialized, expected);
    } //}}}

    #[test]
    fn builder_latex_body_options() {
        //{{{
        let format_options = FormatOptions {
            transforms: Some(vec![Transforms::RmFonts, Transforms::RmNewlines]),
            math_delims: Some(vec!["\\(".to_string(), "\\)".to_string()]),
            displaymath_delims: None,
        };

        let callback = CallBack {
            post: Some("https://duckduckgo.com/".to_string()),
            headers: None,
            reply: Some("Here is your reply".to_string()),
        };

        let region = Region {
            top_left_x: Some(0),
            top_left_y: Some(0),
            width: Some(42),
            height: Some(42),
        };

        let mut latex_options = LaTeXOptions::default();
        latex_options
            .ocr(vec![Ocr::Math, Ocr::Text])
            .format_options(format_options.clone())
            .skip_recrop(true)
            .confidence_threshold(0.42)
            .beam_size(6)
            .n_best(4)
            .callback(callback.clone())
            .metadata(MetaData {})
            .include_detected_alphabets(true)
            .auto_rotate_confidence_threshold(0.42)
            .region(region.clone());
        let expected = LaTeXOptions {
            ocr: Some(vec![Ocr::Math, Ocr::Text]),
            format_options: Some(format_options),
            skip_recrop: Some(true),
            confidence_threshold: Some(0.42),
            beam_size: Some(6),
            callback: Some(callback),
            auto_rotate_confidence_threshold: Some(0.42),
            include_detected_alphabets: Some(true),
            metadata: Some(MetaData {}),
            n_best: Some(4),
            region: Some(region),
        };
        assert_eq!(latex_options, expected);
    } //}}}
}
// }}}
