use reqwest::Url;
use serde::ser::{Serialize, SerializeStruct, Serializer};

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
        let mut state = serializer.serialize_struct("PostPDF", 1)?;
        state.serialize_field("url", &self.url.to_string())?;
        state.end()
    }
} //}}}

// TESTS {{{
#[cfg(test)]
mod test {
    use super::PostPDF;
    use reqwest::Url;
    use serde_json::json;

    #[test]
    fn serialize_postpdf() {
        let postpdf = PostPDF {
            url: Url::parse("https://www.duckduckgo.com/").unwrap(),
        };
        let serilized = serde_json::to_value(postpdf).unwrap();
        let expected = json!({
            "url" : "https://www.duckduckgo.com/"
        });
        assert_eq!(serilized, expected);
    }
}
// }}}
