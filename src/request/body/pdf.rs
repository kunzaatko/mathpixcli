pub use super::shared_objects::{AlphabetsAllowed, MetaData};
use reqwest::Url;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::convert::TryFrom;
use std::path::PathBuf;

const PDF_EXTENSIONS: &[&str] = &["pdf"];

// PDFBody {{{
#[derive(Debug)]
/// This structs contains the possible items that the _text_ endpoint accepts
pub struct PDFBody {
    /// Source of PDF
    pub src: PDFSrc,
    /// Configuration options for the _PDF_ endpoint
    pub options: PDFBodyOptions,
}

impl Serialize for PDFBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PostPDF", 2)?;
        if let PDFSrc::Url(url) = &self.src {
            state.serialize_field("url", &url.to_string())?;
        }
        state.end()
    }
} //}}}

#[derive(Debug, Default, Serialize)]
pub struct PDFBodyOptions {}

/// A checked pdf file path
#[derive(Debug)]
pub struct PDFPath {
    pub pdf_path: PathBuf,
}

impl PDFPath {
    pub fn new(path: PathBuf) -> Result<Self, String> {
        Self::try_from(path)
    }
}

impl TryFrom<PathBuf> for PDFPath {
    type Error = String;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let extension = path
            .extension()
            .ok_or_else(|| "Extension not found of no file passed".to_string())?;
        match extension {
            _ if PDF_EXTENSIONS.contains(&extension.to_str().unwrap().to_lowercase().as_str()) => {
                Ok(PDFPath { pdf_path: path })
            }
            _ => Err("Unsupported filetype. Must be a PDF file.".to_string()),
        }
    }
}

#[derive(Debug)]
pub enum PDFSrc {
    /// HTTP URL where PDF can be downloaded from
    Url(Url),
    /// Path of PDF
    Path(PDFPath),
}

// TESTS {{{
#[cfg(test)]
mod test {
    use super::{PDFBody, PDFBodyOptions, PDFSrc};
    use reqwest::Url;
    use serde_json::json;

    #[test]
    fn serialize_pdfbody() {
        let pdf_body = PDFBody {
            src: PDFSrc::Url(Url::parse("https://www.duckduckgo.com/").unwrap()),
            options: PDFBodyOptions::default(),
        };
        let serilized = serde_json::to_value(pdf_body).unwrap();
        let expected = json!({
            "url" : "https://www.duckduckgo.com/"
        });
        assert_eq!(serilized, expected);
    }
}
// }}}
