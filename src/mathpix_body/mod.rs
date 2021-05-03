use url::Url;

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

struct Src {}
struct MetaData {}
struct DataOptions {}
struct FormatOptions {}
struct AlphabetsAllowed {}
struct Region {}
struct CallBack {}
