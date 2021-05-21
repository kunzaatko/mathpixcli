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

struct Src {
    src: String,
}
// TODO: Ask mathpix what are the possibilities for MetaData <14-05-21, kunzaatko> //
struct MetaData {}
// DataOptions {{{
struct DataOptions {
    /// Include math SVG in `html` and `data` formats
    include_svg: bool,
    /// Include HTML for `html` and `data` outputs (tables only)
    include_table_html: bool,
    /// Include math mode latex in `data` and `html`
    include_latex: bool,
    /// Include tab separated values (TSV) in `data` and `html` outputs (tables only)
    include_tsv: bool,
    /// Include asciimath in `data` and `html` outputs
    include_asciimath: bool,
    /// Include mathml in `data` and `html` outputs
    include_mathml: bool,
}
//}}}
// FormatOptions {{{
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
struct Region {
    top_left_x: Option<u32>,
    top_left_y: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
}
//}}}
// CallBack {{{
struct CallBack {
    /// URL to which to make POST callback
    post: Option<String>,
    /// Key value pairs of headers to make POST
    headers: Option<String>,
    /// Sets values of `reply` field of callbakc response object (see callback response object)
    reply: Option<String>,
}
//}}}
