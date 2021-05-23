pub mod batch;
pub mod latex;
pub mod pdf;
mod shared_objects;
pub mod strokes;
pub mod text;

use self::batch::PostBatch;
use self::latex::PostLaTeX;
use self::pdf::PostPDF;
use self::shared_objects::{CallBack, DataOptions, MetaData, Src};
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
