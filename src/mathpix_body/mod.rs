/// Module for constructing the _batch_ endpoint request
/// ```text
/// The Mathpix API supports processing multiple images in a single POST request to
/// a different endpoint: /v3/batch. The request body may contain all the /v3/latex
/// parameters except src and must contain a urls parameter. The request may contain
/// an additonal callback parameter to receive results after all the images in the
/// batch have been processed.
/// ```
pub mod batch;

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
pub mod latex;

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
pub mod pdf;

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
pub mod strokes;

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
pub mod text;

/// Object that are shared in multiple endpoints. Now consists of `Src` (and `Base64Image`),
/// `MetaData`, `DataOptions` and `CallBack` types.
mod shared_objects;

use self::batch::PostBatch;
use self::latex::PostLaTeX;
use self::pdf::PostPDF;
use self::shared_objects::{CallBack, DataOptions, MetaData, Src};
use self::strokes::PostStrokes;
use self::text::PostText;

/// A type that abstracts over the possible endpoints and helps create the request body
pub enum MathpixPost {
    /// text endpoint variant
    Text(PostText),
    /// pdf endpoint variant
    PDF(PostPDF),
    /// strokes endpoint variant
    Strokes(PostStrokes),
    /// batch endpoint variant
    Batch(PostBatch),
    /// latex endpoint variant
    LaTeX(PostLaTeX),
}
