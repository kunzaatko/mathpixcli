use base64::encode;
use mime::{Mime, IMAGE_JPEG, IMAGE_PNG};
use serde::{Serialize, Serializer};
use std::path::PathBuf;

// TODO: This should implement a custom serialize that convets the image into a base64 string <22-05-21, kunzaatko> //
#[derive(Debug)]
pub struct Base64Image {
    img_path: PathBuf,
    img_type: Mime,
}

impl Serialize for Base64Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!();
    }
}
