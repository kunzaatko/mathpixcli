use super::{AlphabetsAllowed, DataOptions, MetaData, Src};

pub struct PostText {
    src: Src,
    metadata: Option<MetaData>,
    // TODO: This should instead be a list of enums that specifies the allowed formats.
    // Formats type should be implemented in super. <01-05-21, kunzaatko> //
    formats: Option<Vec<TextFormats>>,
    data_options: Option<DataOptions>,
    include_detected_alphabets: Option<bool>,
    alphabets_allowed: Option<AlphabetsAllowed>,
    // TODO: Add the num bounded trait (is between 0 and 1) <30-04-21, kunzaatko> //
    confidence_threshold: Option<f32>,
    include_line_data: Option<bool>,
    include_word_data: Option<bool>,
    include_smiles: Option<bool>,
    include_inchi: Option<bool>,
    include_geometry_data: Option<bool>,
    // TODO: Add the num bounded trait (is between 0 and 1) <30-04-21, kunzaatko> //
    auto_rotate_confidence_threchold: Option<f32>,
    rm_spaces: Option<bool>,
    rm_fonts: Option<bool>,
    numbers_default_to_math: Option<bool>,
}

/// Formats possible for the text endpoint
enum TextFormats {
    /// Mathpix markdown formatted text
    Text,
    /// HTML rendered from `text` via mathpix-markdown-it
    Html,
    /// Data extracte from `html` as specified in the `data_options` request parameter
    Data,
    /// Styled Latex, returned only in cases that the whole image can be reduces to a single
    /// equation
    LatexStyled,
}

impl ToString for TextFormats {
    fn to_string(&self) -> String {
        match self {
            TextFormats::Text => "text".to_string(),
            TextFormats::Html => "html".to_string(),
            TextFormats::Data => "data".to_string(),
            TextFormats::LatexStyled => "latex_styled".to_string(),
        }
    }
}
