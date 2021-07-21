mod error;
mod options;
mod response;

pub use super::shared_objects::request::{CallBack, ImageSrc, MetaData};
use error::LaTeXError;
pub use options::{LaTeXFormats, LaTeXOptions};
use response::LaTeXResponse;
use serde::Serialize;

// LaTeX {{{
#[derive(Serialize, Debug)]
/// This structs contains the possible items that the _latex_ endpoint accepts
pub struct LaTeX {
    /// > Image data, or public URL where image is located
    pub src: ImageSrc,
    /// > String postprocessing formats (see [Formatting](https://docs.mathpix.com/?shell#formatting-2) section)
    pub formats: Vec<LaTeXFormats>,
    /// Configuration options for the _latex_ endpoint
    #[serde(flatten)]
    pub options: LaTeXOptions,
} //}}}
