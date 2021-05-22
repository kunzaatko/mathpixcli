use base64::encode;
use mime::{Mime, IMAGE_JPEG, IMAGE_PNG};
use serde::{Serialize, Serializer};
use std::path::PathBuf;

// TODO: This should implement a custom serialize that convets the image into a base64 string <22-05-21, kunzaatko> //
#[derive(Debug)]
pub struct Base64Image {
    img_path: PathBuf,
    img_mime: Mime,
}

impl ToString for Base64Image {
    fn to_string(&self) -> String {
        let mut string = "data:".to_string();
        string.push_str(&self.img_mime.to_string());
        string.push_str(";base64,");
        // PERF: Why does this clone need to be here? <22-05-21, kunzaatko> //
        string.push_str(&encode(&std::fs::read(self.img_path.clone()).unwrap()));
        return string;
    }
}

impl Serialize for Base64Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
