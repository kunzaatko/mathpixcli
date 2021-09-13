use base64::encode;
use mime::{Mime, IMAGE_JPEG, IMAGE_PNG};
use serde::{Serialize, Serializer};
use std::convert::TryFrom;
use std::path::PathBuf;
use thiserror::Error;

const JPEG_EXTENSIONS: &[&str] = &["jpg", "jpeg", "jpe", "jif", "jfif", "jfi"];
const PNG_EXTENSIONS: &[&str] = &["png"];

#[derive(Debug, PartialEq)]
pub struct Base64Image {
    img_path: PathBuf,
    img_mime: Mime,
}

#[derive(Error, Debug)]
pub enum Base64ImageError {
    #[error("InvalidExtension: {0}")]
    InvalidExtension(String),
    #[error("UnsupportedFileType: {0}")]
    UnsupportedFileType(String),
}

impl TryFrom<PathBuf> for Base64Image {
    //{{{
    type Error = Base64ImageError;
    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let extension = path.extension().ok_or_else(|| {
            Self::Error::InvalidExtension(format!("File {:?} has an invalid extension.", path))
        })?;
        let img_mime = match extension {
            // FIX: error handling <22-05-21, kunzaatko> //
            _ if JPEG_EXTENSIONS.contains(&extension.to_str().unwrap().to_lowercase().as_str()) => {
                IMAGE_JPEG
            }
            _ if PNG_EXTENSIONS.contains(&extension.to_str().unwrap().to_lowercase().as_str()) => {
                IMAGE_PNG
            }
            _ => {
                return Err(Self::Error::UnsupportedFileType(format!(
                    "File {:?} has an unsupported filetype. jpg and png images are supported.",
                    path
                )))
            }
        };
        Ok(Base64Image {
            img_path: path,
            img_mime,
        })
    }
} //}}}

impl ToString for Base64Image {
    //{{{
    fn to_string(&self) -> String {
        let mut string = "data:".to_string();
        string.push_str(&self.img_mime.to_string());
        string.push_str(";base64,");
        // PERF: Why does this clone need to be here? <22-05-21, kunzaatko> //
        string.push_str(&encode(&std::fs::read(self.img_path.clone()).unwrap()));
        return string;
    }
} //}}}

impl Serialize for Base64Image {
    //{{{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
} //}}}

// TESTS {{{
#[cfg(test)]
mod base64image_tests {
    use super::Base64ImageError;
    use super::*;
    use mime::{IMAGE_JPEG, IMAGE_PNG};
    use regex::Regex;
    use serde_json::json;
    use std::convert::TryInto;
    use std::fmt::Display;
    use std::path::PathBuf;

    #[test]
    fn base64image_from_pathbuf() {
        //{{{
        // JPG
        let base64image: Base64Image =
            PathBuf::from("./test/assets/test_encode_base64.jpg".to_string())
                .try_into()
                .unwrap();
        let acctual = Base64Image {
            img_path: "./test/assets/test_encode_base64.jpg".into(),
            img_mime: IMAGE_JPEG,
        };
        assert_eq!(base64image, acctual);

        // PNG
        let base64image: Base64Image =
            PathBuf::from("./test/assets/test_encode_base64.png".to_string())
                .try_into()
                .unwrap();
        let acctual = Base64Image {
            img_path: "./test/assets/test_encode_base64.png".into(),
            img_mime: IMAGE_PNG,
        };
        assert_eq!(base64image, acctual);

        // Uppercase extension
        let base64image: Base64Image =
            PathBuf::from("./test/assets/test_encode_base64.JPG".to_string())
                .try_into()
                .unwrap();
        let acctual = Base64Image {
            img_path: "./test/assets/test_encode_base64.JPG".into(),
            img_mime: IMAGE_JPEG,
        };
        assert_eq!(base64image, acctual);

        // UnsupportedFileType
        let base64image: Result<Base64Image, Base64ImageError> =
            PathBuf::from("./test/assets/test_encode_base64.txt".to_string()).try_into();
        assert!(match base64image {
            Err(Base64ImageError::UnsupportedFileType(_)) => true,
            _ => false,
        });

        let error_re = Regex::new(r"UnsupportedFileType: .*").unwrap();
        assert!(error_re.is_match(&format!("{}", base64image.unwrap_err())));

        // InvalidExtension
        let path_str = "./test/assets/test_encode_base64".to_string();
        let path = PathBuf::from(&path_str);
        let base64image: Result<Base64Image, Base64ImageError> = path.try_into();
        assert!(base64image.is_err());

        let error_re = Regex::new(r"InvalidExtension: .*").unwrap();
        assert!(error_re.is_match(&format!("{}", base64image.unwrap_err())))
    } //}}}

    #[test]
    fn base64image_to_string() {
        //{{{
        let base64image = Base64Image {
            img_path: "./test/assets/test_encode_base64.jpg".into(),
            img_mime: IMAGE_JPEG,
        };
        let string = "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAMCAgICAgMCAgIDAwMDBAYEBAQEBAgGBgUGCQgKCgkICQkKDA8MCgsOCwkJDRENDg8QEBEQCgwSExIQEw8QEBD/wAALCAACAAIBAREA/8QAFAABAAAAAAAAAAAAAAAAAAAACP/EABwQAAEFAQEBAAAAAAAAAAAAAAIBAwQFBgcIAP/aAAgBAQAAPwBfeevPXAt7wLmm63XD+f6PSaPH01tcXFtmYUydZTpEJp1+TIfdbJx55xwzM3DJSIiUlVVVV+//2Q==".to_string();
        assert_eq!(base64image.to_string(), string);
    } //}}}

    #[test]
    fn serialize_base64image() {
        //{{{
        let base64image = Base64Image {
            img_path: "./test/assets/test_encode_base64.jpg".into(),
            img_mime: IMAGE_JPEG,
        };
        let serialized = serde_json::to_value(&base64image).unwrap();
        let acctual = json!("data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAMCAgICAgMCAgIDAwMDBAYEBAQEBAgGBgUGCQgKCgkICQkKDA8MCgsOCwkJDRENDg8QEBEQCgwSExIQEw8QEBD/wAALCAACAAIBAREA/8QAFAABAAAAAAAAAAAAAAAAAAAACP/EABwQAAEFAQEBAAAAAAAAAAAAAAIBAwQFBgcIAP/aAAgBAQAAPwBfeevPXAt7wLmm63XD+f6PSaPH01tcXFtmYUydZTpEJp1+TIfdbJx55xwzM3DJSIiUlVVVV+//2Q==");
        assert_eq!(serialized, acctual);
    } //}}}
}
//}}}
