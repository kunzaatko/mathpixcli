// pub mod batch; {{{
/// Module for constructing the _batch_ endpoint request
/// ```text
/// The Mathpix API supports processing multiple images in a single POST request to
/// a different endpoint: /v3/batch. The request body may contain all the /v3/latex
/// parameters except src and must contain a urls parameter. The request may contain
/// an additonal callback parameter to receive results after all the images in the
/// batch have been processed.
/// ```
pub mod batch; //}}}

// pub mod latex; {{{
/// Module for constructing the _latex_ endpoint request
/// ```text
/// This is an older endpoint that was developed when Mathpix could only read math
/// equations, before we had full text OCR.
///
/// We recommend using v3/text or v3/strokes instead, if you want to handle text and
/// math together.
///
/// There are some benefits to v3/latex when it comes to ignoring everything but the
/// main equation in the image. If you have a solver app that only handles math and not
/// text, you should consider using v3/latex, as it contains special math equation cropping.
///
/// Mathpix supports image recognition for jpg and png images. Images are encoded by base64
/// and sent inside JSON requests.
/// ```
pub mod latex; //}}}

// pub mod pdf; {{{
/// Module for constructing the _pdf_ endpoint request
/// ```text
/// Mathpix supports PDF processing for scientific documents.
///
/// Supported outputs:
///
/// mmd file (Mathpix Markdown spec)
/// docx file (compatible with MS Office, Google Docs, Libre Office)
/// LaTeX zip file (includes images)
///
/// Disclaimer: don't expect good results with fancy formatting as might be encountered in
/// textbooks. The PDF processing feature is designed to work with scientific documents,
/// both single and double columned.
/// The PDF processing as currently exists was designed to work with PDF documents found
/// on ArXiv.
/// ```
pub mod pdf; //}}}

// pub mod strokes; {{{
/// Module for constructing the _strokes_ endpoint request
/// ```text
/// Mathpix supports handwriting recognition for strokes coordinates.
///
/// The v3/strokes endpoint is in beta but provides a service able to transform handwritten
/// strokes into its transcript of text and math.
///
/// This endpoint is very convenient for users that were generating images of handwritten
/// math and text and then using the service v3/text, since with v3/strokes no image generation
/// is required, the request payload is smaller and therefore it results in faster response
/// time.
///
/// The LaTeX of the recognized handwriting is returned inside inline delimiters \( ... \) and
/// block mode delimiters \[ .... \]. Lines are separated with \n newline characters. In some
/// cases (e.g. multiple choice equations) we will try to flatten horizontally aligned content
/// into different lines in order to keep the markup simple.
/// ```
pub mod strokes; //}}}

// pub mod text; {{{
/// Module for constructing the _text_ endpoint request
/// ```text
/// The v3/text endpoint extracts text, and optionally derived data / HTML, from images.
///
/// The text outputs follow mathpix-markdown conventions, including math mode Latex inside
/// inline delimiters \( ... \) and block mode delimiters \[ .... \]. Lines are separated
/// with \n newline characters. In some cases (eg multiple choice equations) we will try
/// flatten horizontally aligned content into different lines in order to keep the markup
/// simple.
///
/// We also provide structured data outputs via the data and html output options. The data
/// output returns a list of extracted formats (such as tsv for tables, or asciimath for
/// equations). The html output provides annotated HTML and can be parsed via HTML / XML
/// parsers.
/// ```
pub mod text; //}}}

/// Object that are shared in multiple endpoints. Now consists of `Src` (and `Base64Image`),
/// `MetaData`, `DataOptions` and `CallBack` types.
mod shared_objects;

use self::shared_objects::{CallBack, DataOptions, MetaData, Src};
use self::{
    batch::BatchBody, latex::LaTeXBody, pdf::PDFBody, strokes::StrokesBody, text::TextBody,
};

use serde::Serialize;

/// A type that abstracts over the possible endpoints and helps create the request body
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Body {
    /// text endpoint variant
    Text(TextBody),
    /// pdf endpoint variant
    PDF(PDFBody),
    /// strokes endpoint variant
    Strokes(StrokesBody),
    /// batch endpoint variant
    Batch(BatchBody),
    /// latex endpoint variant
    LaTeX(LaTeXBody),
}

// TESTS {{{
#[cfg(test)]
mod test {
    use super::Body::Text;
    use super::{
        text::AlphabetsAllowed, text::Base64Image, text::DataOptions, text::Src, text::TextBody,
        text::TextFormats,
    };
    use serde_json::{json, Value::Null};
    use std::convert::TryInto;
    use std::path::PathBuf;

    #[test]
    fn serialize_body() {
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

        let text_body = TextBody {
            src,
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
        let body = Text(text_body);
        let serialized = serde_json::to_value(&body).unwrap();
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
    }
}
// }}}
