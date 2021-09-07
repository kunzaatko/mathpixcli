pub use super::super::shared_objects::request::{
    AlphabetsAllowed, Base64Image, ConfidenceThreshold, DataOptions, ImageSrc, MetaData,
};
use super::error::{
    BadOptionError, ConfidenceThresholdError, LogicalFallacyError, TextOptionsError,
    UnreasonableOptionsError,
};
use rayon::prelude::*;
use serde::{ser::SerializeSeq, Serialize, Serializer};
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Serialize, Debug, PartialEq)]
pub struct TextOptions {
    // {{{
    /// > Key value object
    pub metadata: Option<MetaData>,
    #[serde(serialize_with = "ser_set")]
    /// > List of formats, one of `text`, `data`, `html`, `latex_styled`, see [Format Descriptions](https://docs.mathpix.com/?shell#format-descriptions)
    pub formats: Option<HashSet<TextFormats>>,
    /// > See [DataOptions](https://docs.mathpix.com/?shell#dataoptions-object) section, specifies outputs for `data` and `html` return fields
    pub data_options: Option<DataOptions>,
    /// > Return detected alphabets
    pub include_detected_alphabets: Option<bool>,
    /// > See [AlphabetsAllowed](https://docs.mathpix.com/?shell#alphabetsallowed-object) section, use this to specify which alphabets you don't want in the output
    pub alphabets_allowed: Option<AlphabetsAllowed>,
    /// > Specifies threshold for triggering confidence errors
    pub confidence_threshold: Option<ConfidenceThreshold>,
    /// > Specifies threshold for triggering confidence errors, default `0.75` (symbol level threshold)
    pub confidence_rate_threshold: Option<ConfidenceThreshold>,
    /// > Specifies whether to return information segmented line by line, see [LineData](https://docs.mathpix.com/?shell#linedata-object) object section for details
    pub include_line_data: Option<bool>,
    /// > Specifies whether to return information segmented word by word, see [WordData](https://docs.mathpix.com/?shell#worddata-object) object section for details
    pub include_word_data: Option<bool>,
    /// > Enable experimental chemistry diagram OCR, via RDKIT normalized SMILES with `isomericSmiles=False`, included in `text` output format, via MMD SMILES syntax `<smiles>...</smiles>`
    pub include_smiles: Option<bool>,
    /// > Include InChI data as XML attributes inside `<smiles>` elements, for examples `<smiles inchi="..." inchikey="...">...</smiles>`; only applies when `include_smiles` is true
    pub include_inchi: Option<bool>,
    /// > Enable data extraction for geometry diagrams (currently only supports triangle diagrams); see [GeometryData](https://docs.mathpix.com/?shell#geometry-objects)
    pub include_geometry_data: Option<bool>,
    /// > Specifies threshold for auto rotating image to correct orientation; by default it is set to `0.99`, can be disabled with a value of `1` (see [Auto rotation](https://docs.mathpix.com/?shell#auto-rotation) section for details)
    pub auto_rotate_confidence_threshold: Option<ConfidenceThreshold>,
    /// > Determines whether extra white space is removed from equations in `latex_styled` and `text` formats. Default is `true`.
    pub rm_spaces: Option<bool>,
    /// > Determines whether font commands such as `\mathbf` and `\mathrm` are removed from equations in `latex_styled` and `text` formats. Default is `false`.
    pub rm_fonts: Option<bool>,
    /// > Specifies whether numbers are always math, e.g., `Answer: \( 17 \)` instead of `Answer: 17`. Default is `false`.
    pub numbers_default_to_math: Option<bool>,
} // }}}

pub fn ser_set<S>(set: &Option<HashSet<TextFormats>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(set) = set {
        let vec: Vec<&TextFormats> = set.iter().collect();
        let mut seq = s.serialize_seq(Some(vec.len()))?;
        for format in vec {
            seq.serialize_element(format)?;
        }
        seq.end()
    } else {
        s.serialize_none()
    }
}

impl Default for TextOptions {
    // {{{
    fn default() -> Self {
        TextOptions {
            metadata: None,
            formats: None,
            data_options: None,
            include_detected_alphabets: None,
            alphabets_allowed: None,
            confidence_threshold: None,
            confidence_rate_threshold: None,
            include_line_data: None,
            include_word_data: None,
            include_smiles: None,
            include_inchi: None,
            include_geometry_data: None,
            auto_rotate_confidence_threshold: None,
            rm_spaces: None,
            rm_fonts: None,
            numbers_default_to_math: None,
        }
    }
} // }}}

impl TextOptions {
    //{{{
    // TODO: When implemented in rust <05-09-21, kunzaatko> //
    // type Error = TextOptionsError;

    pub fn set_metadata(&mut self) -> &mut Self {
        //  Option<MetaData>
        todo!()
    }

    /// Add formats to the options of the request
    /// * possible inputs are "text", "data", "html" and "latex_styled"
    ///
    /// # Examples
    /// ```
    /// use maplit::hashset;
    /// use mathpixapi::endpoint::text::{TextOptions, TextFormats};
    /// let mut options = TextOptions::default();
    /// options.add_formats_from_strings(&["text", "data"]).unwrap().add_formats_from_strings(&["html"]).unwrap();
    /// let mut expected = TextOptions::default();
    /// expected.formats = Some(hashset![TextFormats::Text, TextFormats::Data,
    /// TextFormats::Html]);
    /// assert_eq!(options, expected);
    /// ```

    pub fn add_formats_from_strings<S, I: IntoIterator<Item = S>>(
        &mut self,
        formats: I,
    ) -> Result<&mut Self, TextOptionsError>
    where
        S: AsRef<str>,
    {
        //{{{
        if self.formats == None {
            self.formats = Some(HashSet::new());
        }
        if let Some(self_formats) = &mut self.formats {
            for format in formats.into_iter() {
                match format.as_ref() {
                    "text" => self_formats.insert(TextFormats::Text),
                    "data" => self_formats.insert(TextFormats::Data),
                    "html" => self_formats.insert(TextFormats::Html),
                    "latex_styled" => self_formats.insert(TextFormats::LaTeXStyled),
                    format => return Err(BadOptionError::TextFormat(format.into()).into()),
                };
            }
            if self_formats.is_empty() {
                self.formats = None;
            }
        }
        Ok(self)
    } //}}}

    pub fn add_formats<I: IntoIterator<Item = TextFormats>>(&mut self, formats: I) -> &mut Self {
        //{{{
        if let Some(self_formats) = &mut self.formats {
            for format in formats.into_iter() {
                self_formats.insert(format);
            }
        } else {
            self.formats = Some(HashSet::new());
            self.add_formats(formats);
        }
        if let Some(self_formats) = &self.formats {
            if self_formats.is_empty() {
                self.formats = None;
            }
        }
        self
    } //}}}

    pub fn add_format(&mut self, format: TextFormats) -> &mut Self {
        //{{{
        if let Some(formats) = &mut (self.formats) {
            formats.insert(format);
        } else {
            let mut set = HashSet::new();
            set.insert(format);
            self.formats = Some(set);
        }
        self
    } //}}}

    pub fn add_data_options_from_strings<S: AsRef<str>>(
        &mut self,
        data_options: &[S],
    ) -> Result<&mut Self, TextOptionsError> {
        //{{{
        if self.data_options == None && !data_options.is_empty() {
            self.data_options = Some(DataOptions::default());
        }
        if let Some(self_data_options) = &mut self.data_options {
            for data_option in data_options {
                match data_option.as_ref() {
                    "include_asciimath" => self_data_options.include_asciimath = Some(true),
                    "noinclude_asciimath" | "!include_asciimath" => {
                        self_data_options.include_asciimath = Some(false)
                    }
                    "include_latex" => self_data_options.include_latex = Some(true),
                    "noinclude_latex" | "!include_latex" => {
                        self_data_options.include_latex = Some(false)
                    }
                    "include_mathml" => self_data_options.include_mathml = Some(true),
                    "noinclude_mathml" | "!include_mathml" => {
                        self_data_options.include_mathml = Some(false)
                    }
                    "include_svg" => self_data_options.include_svg = Some(true),
                    "noinclude_svg" | "!include_svg" => self_data_options.include_svg = Some(false),
                    "include_table_html" => self_data_options.include_table_html = Some(true),
                    "noinclude_table_html" | "!include_table_html" => {
                        self_data_options.include_table_html = Some(false)
                    }
                    "include_tsv" => self_data_options.include_tsv = Some(true),
                    "noinclude_tsv" | "!include_tsv" => self_data_options.include_tsv = Some(false),
                    data_option => {
                        return Err(BadOptionError::DataOption(data_option.into()).into())
                    }
                }
            }
        }
        Ok(self)
    } //}}}

    pub fn include_detected_alphabets(&mut self, val: bool) -> &mut Self {
        //{{{
        self.include_detected_alphabets = Some(val);
        self
    } //}}}

    pub fn set_alphabets_allowed<S: AsRef<str> + Eq>(
        &mut self,
        alphabets: &[S],
    ) -> Result<&mut Self, TextOptionsError> {
        // {{{
        if self.alphabets_allowed == None && !alphabets.is_empty() {
            self.alphabets_allowed = Some(AlphabetsAllowed::default());
        }

        // NOTE: check for logical fallacies (ex. "noen" and "en" are both in the vector of options) <25-07-21, kunzaatko> //
        let mut err =
            TextOptionsError::LogicalFallacy(LogicalFallacyError::AlphabetsAllowedLogicalFallacy {
                true_alphabet: String::new(),
                false_alphabet: String::new(),
            });
        let alphabets_as_ref: Vec<&str> = alphabets.iter().map(|x| x.as_ref()).collect();
        let mut has_logical_fallacy = |alpha: &str| -> bool {
            //{{{
            let mut noalpha = "no".to_string();
            noalpha.push_str(alpha);
            let mut excl_alpha = "!".to_string();
            excl_alpha.push_str(alpha);

            let fallacy = alphabets_as_ref.contains(&alpha)
                && (alphabets_as_ref.contains(&excl_alpha.as_ref())
                    || alphabets_as_ref.contains(&noalpha.as_ref()));
            let mut true_alphabet = String::new();
            let mut false_alphabet = String::new();
            if fallacy {
                true_alphabet = alpha.to_string();
                false_alphabet = if alphabets_as_ref.contains(&excl_alpha.as_ref()) {
                    excl_alpha
                } else {
                    noalpha
                };
            }
            err = LogicalFallacyError::AlphabetsAllowedLogicalFallacy {
                true_alphabet,
                false_alphabet,
            }
            .into();
            fallacy
        }; //}}}

        for alphabet in &alphabets_as_ref {
            // NOTE: This works because all of the alphabets have 2 characters <25-07-21, kunzaatko> //
            if alphabet.chars().count() == 2 && has_logical_fallacy(alphabet) {
                return Err(err);
            }
        }

        if let Some(self_alphabets_allowed) = &mut self.alphabets_allowed {
            for alphabet in alphabets {
                match alphabet.as_ref() {
                    "en" => self_alphabets_allowed.en = Some(true),
                    "noen" | "!en" => self_alphabets_allowed.en = Some(false),
                    "hi" => self_alphabets_allowed.hi = Some(true),
                    "nohi" | "!hi" => self_alphabets_allowed.hi = Some(false),
                    "zh" => self_alphabets_allowed.zh = Some(true),
                    "nozh" | "!zh" => self_alphabets_allowed.zh = Some(false),
                    "ja" => self_alphabets_allowed.ja = Some(true),
                    "noja" | "!ja" => self_alphabets_allowed.ja = Some(false),
                    "ko" => self_alphabets_allowed.ko = Some(true),
                    "noko" | "!ko" => self_alphabets_allowed.ko = Some(false),
                    "ru" => self_alphabets_allowed.ru = Some(true),
                    "noru" | "!ru" => self_alphabets_allowed.ru = Some(false),
                    "th" => self_alphabets_allowed.th = Some(true),
                    "noth" | "!th" => self_alphabets_allowed.th = Some(false),
                    "all" => {
                        *self_alphabets_allowed = AlphabetsAllowed {
                            en: Some(true),
                            hi: Some(true),
                            zh: Some(true),
                            ja: Some(true),
                            ko: Some(true),
                            ru: Some(true),
                            th: Some(true),
                        };
                    }
                    alphabet => {
                        return Err(BadOptionError::AlphabetAllowed(alphabet.to_string()).into());
                    }
                }
            }
        }

        if let Some(self_alphabets_allowed) = &self.alphabets_allowed {
            if self_alphabets_allowed.all_false() {
                return Err(UnreasonableOptionsError::NoAlphabetsAllowed.into());
                // TODO:  <25-07-21, kunzaatko> //
            }
        }
        Ok(self)
    } //}}}

    pub fn confidence_threshold(&mut self, val: f32) -> Result<&mut Self, TextOptionsError> {
        //{{{
        let confidence_threshold_result = val.try_into();
        if let Ok(confidence_threshold) = confidence_threshold_result {
            self.confidence_threshold = Some(confidence_threshold);
        } else {
            confidence_threshold_result.map_err::<BadOptionError, _>(|e| e.into())?;
        }
        Ok(self)
    } //}}}

    pub fn confidence_rate_threshold<F: Into<f32>>(
        &mut self,
        val: F,
    ) -> Result<&mut Self, TextOptionsError> {
        //{{{
        let confidence_rate_threshold_result = val.into().try_into();
        if let Ok(confidence_rate_threshold) = confidence_rate_threshold_result {
            self.confidence_rate_threshold = Some(confidence_rate_threshold);
        } else {
            confidence_rate_threshold_result.map_err::<BadOptionError, _>(|e| e.into())?;
        }
        Ok(self)
    } //}}}

    pub fn include_line_data(&mut self, val: bool) -> &mut Self {
        self.include_line_data = Some(val);
        self
    }

    pub fn include_word_data(&mut self, val: bool) -> &mut Self {
        self.include_word_data = Some(val);
        self
    }

    pub fn include_smiles(&mut self, val: bool) -> &mut Self {
        self.include_smiles = Some(val);
        self
    }

    pub fn include_inchi(&mut self, val: bool) -> &mut Self {
        self.include_inchi = Some(val);
        self
    }
    pub fn include_geometry_data(&mut self, val: bool) -> &mut Self {
        self.include_geometry_data = Some(val);
        self
    }

    pub fn auto_rotate_confidence_threshold<F: Into<f32>>(
        &mut self,
        val: F,
    ) -> Result<&mut Self, TextOptionsError> {
        //{{{
        let auto_rotate_confidence_threshold_result = val.into().try_into();
        if let Ok(auto_rotate_confidence_threshold) = auto_rotate_confidence_threshold_result {
            self.auto_rotate_confidence_threshold = Some(auto_rotate_confidence_threshold);
        } else {
            auto_rotate_confidence_threshold_result.map_err::<BadOptionError, _>(|e| e.into())?;
        }
        Ok(self)
    } //}}}

    pub fn rm_spaces(&mut self, val: bool) -> &mut Self {
        self.rm_spaces = Some(val);
        self
    }

    pub fn rm_fonts(&mut self, val: bool) -> &mut Self {
        self.rm_fonts = Some(val);
        self
    }

    pub fn numbers_default_to_math(&mut self, val: bool) -> &mut Self {
        self.numbers_default_to_math = Some(val);
        self
    }
} //}}}

/// Format specifications possible for the _text_ endpoint
#[derive(Debug, Serialize, Eq, Clone, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TextFormats {
    //{{{
    /// > Mathpix markdown formatted text
    Text,
    /// > HTML rendered from `text` via mathpix-markdown-it
    Html,
    /// > Data extracted from `html` as specified in the `data_options` request parameter
    Data,
    /**
    > Styled LaTeX, returned only in cases that the whole image can be reduces to a single
    > equation
    */
    #[serde(rename = "latex_styled")]
    LaTeXStyled,
}

impl ToString for TextFormats {
    fn to_string(&self) -> String {
        match self {
            TextFormats::Text => "text".to_string(),
            TextFormats::Html => "html".to_string(),
            TextFormats::Data => "data".to_string(),
            TextFormats::LaTeXStyled => "latex_styled".to_string(),
        }
    }
}
//}}}

// TESTS {{{
#[cfg(test)]
mod text_options_tests {
    use super::super::super::shared_objects::request::Base64Image;
    use super::super::{AlphabetsAllowed, DataOptions, ImageSrc, Text, TextFormats, TextOptions};
    use super::TextOptionsError;
    use serde_json::{json, Value::Null};
    use std::convert::TryInto;
    use std::path::PathBuf;

    use maplit::hashset;

    #[test]
    fn alphabets_allow() {
        //{{{
        let mut alphabets = AlphabetsAllowed::default();
        alphabets
            .allow(vec!["en".to_string(), "ru".to_string()])
            .unwrap();
        let expected = AlphabetsAllowed {
            en: Some(true),
            ru: Some(true),
            hi: None,
            ja: None,
            ko: None,
            th: None,
            zh: None,
        };
        assert_eq!(alphabets, expected)
    } //}}}

    #[test]
    fn alphabets_disallow() {
        //{{{
        let mut alphabets = AlphabetsAllowed::default();
        alphabets
            .disallow(vec!["en".to_string(), "ru".to_string()])
            .unwrap();
        let expected = AlphabetsAllowed {
            en: Some(false),
            ru: Some(false),
            hi: None,
            ja: None,
            ko: None,
            th: None,
            zh: None,
        };
        assert_eq!(alphabets, expected);
    } //}}}

    #[test]
    fn alphabets_disallow_and_allow() {
        //{{{
        let mut alphabets = AlphabetsAllowed::default();
        alphabets
            .disallow(vec!["en".to_string(), "ru".to_string()])
            .unwrap();
        alphabets
            .allow(vec!["en".to_string(), "hi".to_string()])
            .unwrap();
        let expected = AlphabetsAllowed {
            en: Some(true),
            ru: Some(false),
            hi: Some(true),
            ja: None,
            ko: None,
            th: None,
            zh: None,
        };
        assert_eq!(alphabets, expected)
    } //}}}

    #[test]
    fn serialize_alphabets() {
        //{{{
        let mut alphabets = AlphabetsAllowed::default();
        alphabets
            .disallow(vec!["en".to_string(), "ru".to_string()])
            .unwrap();
        alphabets
            .allow(vec!["en".to_string(), "hi".to_string()])
            .unwrap();
        let serialized = serde_json::to_value(&alphabets).unwrap();
        let expected = json!({
                "en": true,
                "ru": false,
                "hi": true,
                "ja": Null,
                "ko": Null,
                "th": Null,
                "zh": Null,
        });
        assert_eq!(expected, serialized);
    } //}}}

    #[test]
    fn formats_add_empty() {
        //{{{
        let mut text_opts = TextOptions::default();

        let empty_formats_strs: [&str; 0] = [];
        text_opts
            .add_formats_from_strings(empty_formats_strs)
            .unwrap();

        assert_eq!(text_opts.formats, Option::None);

        let empty_formats: [TextFormats; 0] = [];
        text_opts.add_formats(empty_formats);

        assert_eq!(text_opts.formats, Option::None);

        let serialized = serde_json::to_value(&text_opts).unwrap();

        // NOTE: Testing serialization when TextOptions == None <07-09-21, kunzaatko> //
        assert_eq!(
            serialized,
            serde_json::to_value(TextOptions::default()).unwrap()
        );
    } // }}}

    #[test]
    fn formats_from_strings() {
        // {{{
        let mut options = TextOptions::default();
        options
            .add_formats_from_strings(["text", "data", "html", "latex_styled"])
            .unwrap();
        assert_eq!(
            options.formats,
            Some(
                hashset! {TextFormats::Text, TextFormats::Data, TextFormats::Html, TextFormats::LaTeXStyled}
            )
        );

        assert!(options.add_formats_from_strings(["error"]).is_err())
    } // }}}

    #[test]
    fn add_format() {
        // {{{
        let mut options = TextOptions::default();
        options
            .add_formats_from_strings(["text", "data", "html", "latex_styled"])
            .unwrap();
        assert_eq!(
            options.formats,
            Some(
                hashset! {TextFormats::Text, TextFormats::Data, TextFormats::Html, TextFormats::LaTeXStyled}
            )
        );
    } // }}}

    #[test]
    fn serialize_text_formats() {
        //{{{
        let text_formats: Vec<TextFormats> = vec![
            TextFormats::Text,
            TextFormats::Html,
            TextFormats::Data,
            TextFormats::LaTeXStyled,
        ];
        let serialized = serde_json::to_value(&text_formats).unwrap();
        let expected = json!(["text", "html", "data", "latex_styled"]);
        assert_eq!(serialized, expected);
    } //}}}

    #[test]
    fn to_string_text_formats() {
        //{{{
        let text_formats: Vec<TextFormats> = vec![
            TextFormats::Text,
            TextFormats::Html,
            TextFormats::Data,
            TextFormats::LaTeXStyled,
        ];
        let text_formats_text: Vec<String> =
            text_formats.iter().clone().map(|x| x.to_string()).collect();
        let expected = vec![
            "text".to_string(),
            "html".to_string(),
            "data".to_string(),
            "latex_styled".to_string(),
        ];
        assert_eq!(text_formats_text, expected);
    } //}}}

    #[test]
    fn serialize_text() {
        //{{{
        let image: Base64Image = PathBuf::from("./test/assets/test_encode_base64.jpg".to_string())
            .try_into()
            .unwrap();
        let mut alphabets_allowed = AlphabetsAllowed::default();
        alphabets_allowed
            .allow(vec!["ru".to_string(), "en".to_string()])
            .unwrap();
        let data_options = DataOptions {
            include_asciimath: Some(true),
            include_latex: Some(false),
            include_mathml: None,
            include_svg: None,
            include_table_html: None,
            include_tsv: None,
        };
        let src = ImageSrc::Image(image);

        let text_opts = TextOptions {
            metadata: None,
            formats: Some(hashset! {TextFormats::Text, TextFormats::Data}),
            alphabets_allowed: Some(alphabets_allowed),
            auto_rotate_confidence_threshold: Some(1.0.try_into().unwrap()),
            confidence_threshold: Some(1.0.try_into().unwrap()),
            confidence_rate_threshold: Some(1.0.try_into().unwrap()),
            data_options: Some(data_options),
            include_detected_alphabets: Some(true),
            include_geometry_data: Some(false),
            include_inchi: Some(true),
            include_line_data: Some(false),
            include_smiles: Some(true),
            include_word_data: Some(false),
            rm_fonts: Some(true),
            rm_spaces: Some(false),
            numbers_default_to_math: None,
        };

        let text = Text {
            src: Some(src),
            options: text_opts,
        };
        let serialized = serde_json::to_value(&text).unwrap();
        // NOTE: Because there is no way tell the order of the TextFormats in the serialized array
        // (because it is a set) we have to have two different serializations that are right <07-09-21, kunzaatko> //
        let expected_1 = json!({
            "src" : "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAMCAgICAgMCAgIDAwMDBAYEBAQEBAgGBgUGCQgKCgkICQkKDA8MCgsOCwkJDRENDg8QEBEQCgwSExIQEw8QEBD/wAALCAACAAIBAREA/8QAFAABAAAAAAAAAAAAAAAAAAAACP/EABwQAAEFAQEBAAAAAAAAAAAAAAIBAwQFBgcIAP/aAAgBAQAAPwBfeevPXAt7wLmm63XD+f6PSaPH01tcXFtmYUydZTpEJp1+TIfdbJx55xwzM3DJSIiUlVVVV+//2Q==",
            "metadata": Null,
            "formats": ["text", "data"],
            "alphabets_allowed": {
                "en" : true,
                "ru" : true,
                "hi" : Null,
                "zh" : Null,
                "ja" : Null,
                "ko" : Null,
                "th" : Null,
            },
            "auto_rotate_confidence_threshold": 1.,
            "confidence_threshold": 1.,
            "confidence_rate_threshold": 1.,
            "data_options": {
                "include_asciimath": true,
                "include_latex": false,
                "include_mathml": Null,
                "include_svg": Null,
                "include_table_html": Null,
                "include_tsv": Null,
            },
            "include_detected_alphabets": true,
            "include_geometry_data": false,
            "include_inchi": true,
            "include_line_data": false,
            "include_smiles": true,
            "include_word_data": false,
            "rm_fonts": true,
            "rm_spaces": false,
            "numbers_default_to_math": Null,
        });

        let expected_2 = json!({
            "src" : "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wBDAAMCAgICAgMCAgIDAwMDBAYEBAQEBAgGBgUGCQgKCgkICQkKDA8MCgsOCwkJDRENDg8QEBEQCgwSExIQEw8QEBD/wAALCAACAAIBAREA/8QAFAABAAAAAAAAAAAAAAAAAAAACP/EABwQAAEFAQEBAAAAAAAAAAAAAAIBAwQFBgcIAP/aAAgBAQAAPwBfeevPXAt7wLmm63XD+f6PSaPH01tcXFtmYUydZTpEJp1+TIfdbJx55xwzM3DJSIiUlVVVV+//2Q==",
            "metadata": Null,
            "formats": ["data","text"],
            "alphabets_allowed": {
                "en" : true,
                "ru" : true,
                "hi" : Null,
                "zh" : Null,
                "ja" : Null,
                "ko" : Null,
                "th" : Null,
            },
            "auto_rotate_confidence_threshold": 1.,
            "confidence_threshold": 1.,
            "confidence_rate_threshold": 1.,
            "data_options": {
                "include_asciimath": true,
                "include_latex": false,
                "include_mathml": Null,
                "include_svg": Null,
                "include_table_html": Null,
                "include_tsv": Null,
            },
            "include_detected_alphabets": true,
            "include_geometry_data": false,
            "include_inchi": true,
            "include_line_data": false,
            "include_smiles": true,
            "include_word_data": false,
            "rm_fonts": true,
            "rm_spaces": false,
            "numbers_default_to_math": Null,
        });
        assert!([expected_1, expected_2].iter().any(|r| *r == serialized));
    } //}}}

    #[test]
    fn builder_formats_text_options() {
        //{{{
        let mut text_body_options = TextOptions::default();
        text_body_options.add_format(TextFormats::Data);
        text_body_options.add_formats([TextFormats::LaTeXStyled, TextFormats::Html]);
        let mut expected = TextOptions::default();
        expected.formats = Some(hashset![
            TextFormats::Data,
            TextFormats::LaTeXStyled,
            TextFormats::Html,
        ]);
        assert_eq!(text_body_options, expected);
    } //}}}
}
//}}}
