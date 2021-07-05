mod options;
use super::super::MathpixEndpoint;
pub use super::shared_objects::{AlphabetsAllowed, Base64Image, DataOptions, ImageSrc, MetaData};
pub use options::{TextBodyOptions, TextFormats};
use serde::Serialize;

// TextBody {{{
#[derive(Serialize, Debug)]
/// This structs contains the possible items that the _text_ endpoint accepts
pub struct TextBody {
    /// Image data, or public URL where image is located
    pub src: ImageSrc,
    /// Configuration options for the _text_ endpoint
    #[serde(flatten)]
    pub options: TextBodyOptions,
} //}}}

/* impl MathpixEndpoint for TextBody {
    type Src = ImageSrc;
    // TODO: Error handling <05-07-21, kunzaatko> //
    type Error = String;

} */

// TESTS {{{
#[cfg(test)]
mod test {}
//}}}
