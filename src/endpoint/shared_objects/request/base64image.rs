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
    ExtensionError(String),
    FileTypeError(String),
}

impl std::fmt::Display for Base64ImageError {
    //{{{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Base64ImageError::ExtensionError(context) => write!(f, "ExtensionError: {}", context),
            Base64ImageError::FileTypeError(context) => write!(f, "FileTypeError: {}", context),
        }
    }
} //}}}

impl TryFrom<PathBuf> for Base64Image {
    //{{{
    type Error = Base64ImageError;
    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let extension = path.extension().ok_or(Self::Error::ExtensionError(format!(
            "File {:?} has an invalid extension.",
            path
        )))?;
        let img_mime = match extension {
            // FIX: error handling <22-05-21, kunzaatko> //
            _ if JPEG_EXTENSIONS.contains(&extension.to_str().unwrap().to_lowercase().as_str()) => {
                IMAGE_JPEG
            }
            _ if PNG_EXTENSIONS.contains(&extension.to_str().unwrap().to_lowercase().as_str()) => {
                IMAGE_PNG
            }
            _ => {
                return Err(Self::Error::FileTypeError(format!(
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
        string
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
    use super::*;
    use mime::IMAGE_JPEG;
    use serde_json::json;
    use std::convert::TryInto;
    use std::path::PathBuf;

    #[test]
    fn base64image_from_pathbuf() {
        //{{{
        let base64image: Base64Image =
            PathBuf::from("./test/assets/test_encode_base64.jpg".to_string())
                .try_into()
                .unwrap();
        let acctual = Base64Image {
            img_path: "./test/assets/test_encode_base64.jpg".into(),
            img_mime: IMAGE_JPEG,
        };
        assert_eq!(base64image, acctual);
    } //}}}

    #[test]
    fn base64image_from_pathbuf_no_extension() {
        // {{{
        let base64image: Result<Base64Image, Base64ImageError> =
            PathBuf::from("./test/assets/test_encode_base64".to_string()).try_into();
        assert!(match base64image {
            Err(Base64ImageError::ExtensionError(_)) => true,
            _ => false,
        })
    } //}}}

    #[test]
    fn base64image_from_pathbuf_invalid_filetype() {
        // {{{
        let base64image: Result<Base64Image, Base64ImageError> =
            PathBuf::from("./test/assets/test_encode_base64.txt".to_string()).try_into();
        assert!(match base64image {
            Err(Base64ImageError::FileTypeError(_)) => true,
            _ => false,
        })
    } // }}}

    #[test]
    fn base64_from_pathbuf_upercase_extension() {
        //{{{
        let base64image: Base64Image =
            PathBuf::from("./test/assets/test_encode_base64.JPG".to_string())
                .try_into()
                .unwrap();
        let acctual = Base64Image {
            img_path: "./test/assets/test_encode_base64.JPG".into(),
            img_mime: IMAGE_JPEG,
        };
        assert_eq!(base64image, acctual);
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
