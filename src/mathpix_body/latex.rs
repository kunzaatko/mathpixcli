use super::{CallBack, MetaData, Region, Src};
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

// TESTS {{{
#[cfg(test)]
mod test {
    use super::{FormatOptions, LaTeXFormats, Ocr, PostLaTeX, Transforms};
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
}
// }}}
