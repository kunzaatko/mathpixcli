use super::{Src, FormatOptions, Region, CallBack, MetaData};

pub struct PostLaTeX {
    src: Src,
    formats: Vec<String>,
    ocr: Option<Vec<String>>,
    format_options: Option<FormatOptions>,
    skip_recrop: Option<bool>,
    // TODO: bounded 0-1. <01-05-21, kunzaatko> //
    confidence_threshold: Option<f32>,
    // TODO: this should be bounded. It is from 1-5. <01-05-21, kunzaatko> //
    beam_size: Option<u8>,
    // TODO: this should be bounded. It is from 1-beam_size. <01-05-21, kunzaatko> //
    n_best: Option<u8>,
    region: Option<Region>,
    callback: Option<CallBack>,
    metadata: Option<MetaData>,
    include_detected_alphabets: Option<bool>,
    // TODO: bounded 0-1. <01-05-21, kunzaatko> //
    auto_rotate_confidence_threshold: Option<f32>
}
