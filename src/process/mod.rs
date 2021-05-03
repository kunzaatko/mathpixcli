use url::Url;

mod batch;
mod latex;
mod pdf;
mod strokes;
mod text;

use self::batch::BatchRequest;
use self::latex::LaTeXRequest;
use self::pdf::PDFRequest;
use self::strokes::StrokesRequest;
use self::text::TextRequest;

// TODO: Add docs <30-04-21, kunzaatko> //
pub enum Process {
    Text(TextRequest),
    PDF(PDFRequest),
    Strokes(StrokesRequest),
    Batch(BatchRequest),
    LaTeX(LaTeXRequest),
}

struct Src {}
struct MetaData {}
struct DataOptions {}
struct FormatOptions {}
struct AlphabetsAllowed {}
struct Region {}
struct CallBack {}
