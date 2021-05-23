mod batch;
mod latex;
mod pdf;
mod strokes;
mod text;
mod shared_objects;

use self::batch::PostBatch;
use self::latex::PostLaTeX;
use self::pdf::PostPDF;
use self::strokes::PostStrokes;
use self::text::PostText;
use self::shared_objects::{Src, MetaData, DataOptions, CallBack};

// TODO: Add docs <30-04-21, kunzaatko> //
pub enum MathpixPost {
    Text(PostText),
    PDF(PostPDF),
    Strokes(PostStrokes),
    Batch(PostBatch),
    LaTeX(PostLaTeX),
}
