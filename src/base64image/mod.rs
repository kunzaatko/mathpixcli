use base64::encode;
use mime::{Mime, IMAGE_JPEG, IMAGE_PNG};
use serde::{Serialize, Serializer};
use std::path::PathBuf;

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

#[cfg(test)]
mod test {
    use super::Base64Image;
    use mime::IMAGE_JPEG;
    use serde_json::json;

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
