use reqwest::Url;
use serde::{Serialize, Serializer};

// PostPDF {{{
#[derive(Debug)]
/// This structs contains the possible items that the _text_ endpoint accepts
pub struct PostPDF {
    /// HTTP URL where PDF can be downloaded from
    pub url: Url,
}

impl Serialize for PostPDF {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.url.to_string().serialize(serializer)
    }
}//}}}
