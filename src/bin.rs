extern crate mathpixapi;

use clap::{crate_authors, crate_version, App, Arg, ArgGroup};

fn main() {
    // Text endpoint{{{
    let text_subcommand = App::new("text")
                .about("Text endpoint for for the Mathpix API")
                .arg(
                    // TextBodyOptions.text_formats {{{
                    Arg::new("TextBodyOptions.text_formats")
                        .short('f')
                        .long("format")
                        // TODO: What is the default <29-05-21, kunzaatko> //
                        .about("List of formats required in the output")
                        .value_name("FORMAT")
                        .possible_values(&["text", "html", "data", "latex_styled", "all"])
                        .multiple(true)
                        .takes_value(true),
                )//}}}
                .arg(
                    // TextBodyOptions.data_options {{{
                    Arg::new("TextBodyOptions.data_options")
                        .long("data_options")
                        // TODO: What is the default <29-05-21, kunzaatko> //
                        .about("List of data options for the outputs")
                        .value_name("OPTION")
                        .possible_values(&["include_svg", "include_table_html", "include_latex", "include_tsv", "include_acsiimath", "include_mathml", "all"])
                        .multiple(true)
                        .takes_value(true),
                )//}}}
                .arg(
                    // TextBodyOptions.include_detected_alphabets {{{
                    Arg::new("TextBodyOptions.include_detected_alphabets")
                        // TODO: What is the default <29-05-21, kunzaatko> //
                        .about("Return detected alphabets")
                        .long("include_detected_alphabets")
                )//}}}
                .arg(
                    // TextBodyOptions.alphabets_allowed {{{
                    Arg::new("TextBodyOptions.alphabets_allowed")
                        // TODO: What is the default <29-05-21, kunzaatko> //
                        .about("Specify which alphabets are allowed for the OCR")
                        .long("alphabets_allowed")
                        .value_name("ALPHABET")
                        .takes_value(true)
                        .multiple(true)
                        .possible_values(&["en","hi","zh","ja","ko","ru","th","all"])
                )//}}}
                .arg(
                    // TextBodyOptions.confidence_threshold {{{
                    Arg::new("TextBodyOptions.confidence_threshold")
                        .about("Specifies threshold for triggering confidence errors")
                        .long("confidence_threshold")
                        .takes_value(true)
                        .value_name("THRESHOLD")
                )//}}}
                .arg(
                    // TextBodyOptions.confidence_rate_threshold {{{
                    Arg::new("TextBodyOptions.confidence_rate_threshold")
                        // NOTE: 0.75 is API default (no need to set as explicit default)
                        .about("Specifies threshold for triggering confidence errors for individual symbols [default: 0.75]")
                        .long("confidence_rate_threshold")
                        .takes_value(true)
                        .value_name("THRESHOLD")
                )//}}}
                .group(
                    //  Included data formats {{{
                    ArgGroup::new("Included data formats")
                        // TODO: What is the default <29-05-21, kunzaatko> //
                        .arg("TextBodyOptions.include_line_data")
                        .arg("TextBodyOptions.include_word_data")
                        .arg("TextBodyOptions.include_smiles")
                        .arg("TextBodyOptions.include_inchi")
                        .arg("TextBodyOptions.include_geometry_data")
                        .multiple(true)
                )//}}}
                .arg(
                    // TextBodyOptions.include_line_data {{{
                    Arg::new("TextBodyOptions.include_line_data")
                        .long("include_line_data")
                        .about("Include the line data objects in the response")
                )//}}}
                .arg(
                    // TextBodyOptions.include_word_data {{{
                    Arg::new("TextBodyOptions.include_word_data")
                        .long("include_word_data")
                        .about("Include the word data objects in the response")
                )//}}}
                .arg(
                    // TextBodyOptions.include_smiles {{{
                    Arg::new("TextBodyOptions.include_smiles")
                        .long("include_smiles")
                        .about("Include the smiles data objects in the response")
                )//}}}
                .arg(
                    // TextBodyOptions.include_inchi {{{
                    Arg::new("TextBodyOptions.include_inchi")
                        .long("include_inchi")
                        .about("Include the InChI data as XML attributes inside smiles element")
                )//}}}
                .arg(
                    // TextBodyOptions.include_geometry_data {{{
                    Arg::new("TextBodyOptions.include_geometry_data")
                        .long("include_geometry_data")
                        .about("Include data extraction for geometry diagrams")
                ); //}}}
                   //}}}

    // LaTeX endpoint {{{
    let latex_subcommand = App::new("latex")
        .about("LaTeX endpoint for for the Mathpix API")
        .arg(
            // LaTeXBodyOptions.format_options {{{
            Arg::new("LaTeXBodyOptions.formats")
                .long("format")
                .short('f')
                // TODO: What is the default <29-05-21, kunzaatko> //
                .about("List of formats required in the output")
                .value_name("FORMAT")
                .possible_values(&[
                    "text",
                    "text_display",
                    "latex_styled",
                    "latex_simplified",
                    "latex_list",
                    "mathml",
                    "asciimath",
                    "wolfram",
                    "all",
                ])
                .multiple(true)
                .required(true),
        ) //}}}
        .arg(
            // LaTeXBodyOptions.ocr {{{
            Arg::new("LaTeXBodyOptions.ocr")
                .long("ocr")
                // TODO: What is the default <29-05-21, kunzaatko> //
                .about("Whether to process only `math` or both `math` and `text`")
                .possible_values(&["text", "math", "all"])
                .value_name("OCR")
                .takes_value(true)
                .multiple(true),
        ) //}}}
        .group(
            // LaTeXBodyOptions.format_options {{{
            ArgGroup::new("LaTeXBodyOptions.format_options")
                .arg("LaTeXBodyOptions.format_options.transforms")
                .arg("LaTeXBodyOptions.format_options.math_delimiters")
                .arg("LaTeXBodyOptions.format_options.displaymath_delims")
                .multiple(true),
        ) //}}}
        .arg(
            // LaTeXBodyOptions.format_options.transforms {{{
            Arg::new("LaTeXBodyOptions.format_options.transforms")
                .long("transforms")
                // TODO: What is the default <29-05-21, kunzaatko> //
                .about("Transformations to be applied to the output")
                .possible_values(&[
                    "rm_spaces",
                    "rm_newlines",
                    "rm_fonts",
                    "rm_style_syms",
                    "rm_text",
                    "long_frac",
                    "all",
                ])
                .value_name("TRANSFORM")
                .multiple(true),
        ) //}}}
        .arg(
            // LaTeXBodyOptions.format_options.math_delimiters {{{
            Arg::new("LaTeXBodyOptions.format_options.math_delimiters")
                .long("math_delims")
                // TODO: What is the default <29-05-21, kunzaatko> //
                .about("delimiters to be used in inline math")
                .number_of_values(2)
                .value_names(&["LDELIM", "RDELIM"])
                .multiple(true),
        ) //}}}
        // TODO: Add aliases <29-05-21, kunzaatko> //
        .arg(
            // LaTeXBodyOptions.format_options.displaymath_delims {{{
            Arg::new("LaTeXBodyOptions.format_options.displaymath_delims")
                .long("displaymath_delims")
                // TODO: What is the default <29-05-21, kunzaatko> //
                .about("delimiters to be used in displayed math")
                .value_names(&["LDELIM", "RDELIM"])
                .number_of_values(2),
        ) //}}}
        .arg(
            // LaTeXBodyOptions.skip_recrop {{{
            Arg::new("LaTeXBodyOptions.skip_recrop")
                .long("skip_recrop")
                .about("skip the recroping before OCR of the image"),
        ) //}}}
        .arg(
            // LaTeXBodyOptions.confidence_threshold {{{
            Arg::new("LaTeXBodyOptions.confidence_threshold")
                .long("confidence_threshold")
                // TODO: What is the default <29-05-21, kunzaatko> //
                .about("threshold for triggering confidence errors")
                .value_name("NUMBER"),
        ) //}}}
        .arg(
            // LaTeXBodyOptions.beam_size {{{
            Arg::new("LaTeXBodyOptions.beam_size")
                .long("beam_size")
                // TODO: What is default <29-05-21, kunzaatko> //
                .about("number of results to consider during recognition")
                .value_name("NUMBER"),
        ) //}}}
        .arg(
            // LaTeXBodyOptions.n_best {{{
            Arg::new("LaTeXBodyOptions.n_best")
                .long("n_best")
                // TODO: What is default <29-05-21, kunzaatko> //
                .about("number of best results to return")
                .value_name("NUMBER"),
        ) //}}}
        // TODO: Special from string for region <29-05-21, kunzaatko> //
        .arg(
            // LaTeXBodyOptions.region {{{
            Arg::new("LaTeXBodyOptions.region")
                .long("region")
                .about("region of the image to process")
                .number_of_values(4)
                .value_names(&["TOP_LEFT_X", "TOP_LEFT_Y", "WIDTH", "HEIGHT"]),
        ) //}}}
        .group(
            // LaTeXBodyOptions.callback {{{
            ArgGroup::new("LaTeXBodyOptions.callback")
                .arg("LaTeXBodyOptions.callback.post")
                .arg("LaTeXBodyOptions.callback.headers")
                .arg("LaTeXBodyOptions.callback.reply"),
        ) //}}}
        .arg(
            // LaTeXBodyOptions.callback.post {{{
            Arg::new("LaTeXBodyOptions.callback.post")
                .long("post")
                .about("URL where to post callback")
                .value_name("URL"),
        ) //}}}
        // TODO: Make custom verification for having even number of arguments <29-05-21, kunzaatko> //
        // TODO: Make parsing of $date and $image and so on for the strings in headers <29-05-21, kunzaatko> //
        // TODO: Use custom delimiters for the headers using KEY : VALUE <29-05-21, kunzaatko> //
        .arg(
            // LaTeXBodyOptions.callback.headers {{{
            Arg::new("LaTeXBodyOptions.callback.headers")
                .long("headers")
                .about("key value pairs of headers to make callback post")
                .multiple(true)
                .value_names(&["KEY", "VALUE"]),
        ) //}}}
        // TODO: What are the possible values for this <29-05-21, kunzaatko> //
        .arg(
            // LaTeXBodyOptions.callback.reply {{{
            Arg::new("LaTeXBodyOptions.callback.reply")
                .long("reply")
                .about("values for `reply` field of callback object")
                .multiple(true)
                .value_name("VALUE"),
        ) //}}}
        .arg(
            // LaTeXBodyOptions.include_detected_alphabets {{{
            Arg::new("LaTeXBodyOptions.include_detected_alphabets")
                .long("include_detected_alphabets")
                .about("Detected alphabets in the response"),
        ); //}}}
           // }}}

    let args = App::new("MathpixCLI")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Provides a command line interface for the Mathpix OCR API")
        .arg(
            // Header.app_id {{{
            Arg::new("Header.app_id")
                .long("id")
                .env("MATHPIX_APP_ID")
                .hide_env_values(true)
                .about("API ID to use for the request header")
                .value_name("ID")
        )//}}}
        .arg(
            // Header.app_key {{{
            Arg::new("Header.app_key")
                .long("key")
                .env("MATHPIX_APP_KEY")
                .hide_env_values(true)
                .about("API key to use for the request header")
                .value_name("KEY")
        )//}}}
        .subcommand(text_subcommand)
        .subcommand(latex_subcommand)
        .get_matches();
}
