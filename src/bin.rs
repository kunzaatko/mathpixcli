extern crate mathpixapi;

use clap::{crate_authors, crate_version, App, Arg, ArgGroup};

fn main() {
    // Text endpoint{{{
    let text_subcommand = App::new("text")
                .about("Text endpoint for for the Mathpix API")
                .arg(
                    Arg::new("Text Formats")
                        .short('f')
                        .long("format")
                        // TODO: What is the default <29-05-21, kunzaatko> //
                        .about("List of formats required in the output")
                        .value_name("FORMAT")
                        .possible_values(&["text", "html", "data", "latex_styled", "all"])
                        .multiple(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("Data Options")
                        .long("data_options")
                        // TODO: What is the default <29-05-21, kunzaatko> //
                        .about("List of data options for the outputs")
                        .value_name("OPTION")
                        .possible_values(&["include_svg", "include_table_html", "include_latex", "include_tsv", "include_acsiimath", "include_mathml", "all"])
                        .multiple(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("Include Detected Alphabets")
                        // TODO: What is the default <29-05-21, kunzaatko> //
                        .about("Return detected alphabets")
                        .long("include_detected_alphabets")
                )
                .arg(
                    Arg::new("Alphabets allowed")
                        // TODO: What is the default <29-05-21, kunzaatko> //
                        .about("Specify which alphabets are allowed for the OCR")
                        .long("alphabets_allowed")
                        .value_name("ALPHABET")
                        .takes_value(true)
                        .multiple(true)
                        .possible_values(&["en","hi","zh","ja","ko","ru","th","all"])
                )

                // Confidence threshholds
                .arg(
                    Arg::new("Confidence Threshold")
                        .about("Specifies threshold for triggering confidence errors")
                        .long("confidence_threshold")
                        .takes_value(true)
                        .value_name("THRESHOLD")
                )
                .arg(
                    Arg::new("Confidence Rate Threshold")
                        // NOTE: 0.75 is API default (no need to set as explicit default)
                        .about("Specifies threshold for triggering confidence errors for individual symbols [default: 0.75]")
                        .long("confidence_rate_threshold")
                        .takes_value(true)
                        .value_name("THRESHOLD")
                )

                // Included data formats
                .group(ArgGroup::new("Included data formats")
                        // TODO: What is the default <29-05-21, kunzaatko> //
                        .arg("include_line_data")
                        .arg("include_word_data")
                        .arg("include_smiles")
                        .arg("include_inchi")
                        .arg("include_geometry_data")
                        .multiple(true)
                )
                .arg(
                    Arg::new("include_line_data")
                        .long("include_line_data")
                        .about("Include the line data objects in the response")
                )
                .arg(
                    Arg::new("include_word_data")
                        .long("include_word_data")
                        .about("Include the word data objects in the response")
                )
                .arg(
                    Arg::new("include_smiles")
                        .long("include_smiles")
                        .about("Include the smiles data objects in the response")
                )
                .arg(
                    Arg::new("include_inchi")
                        .long("include_inchi")
                        .about("Include the InChI data as XML attributes inside smiles element")
                )
                .arg(
                    Arg::new("include_geometry_data")
                        .long("include_geometry_data")
                        .about("Include data extraction for geometry diagrams")
                ); //}}}

    // LaTeX endpoint {{{
    let latex_subcommand = App::new("latex")
        .about("LaTeX endpoint for for the Mathpix API")
        .arg(
            Arg::new("LaTeX formats")
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
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("OCR")
                .long("ocr")
                // TODO: What is the default <29-05-21, kunzaatko> //
                .about("Whether to process only `math` or both `math` and `text`")
                .possible_values(&["text", "math", "all"])
                .value_name("OCR")
                .takes_value(true)
                .multiple(true),
        )
        .group(
            ArgGroup::new("Format Options")
                .arg("Transforms")
                .arg("Math delimiters")
                .arg("Displayed math delimiters")
                .multiple(true),
        )
        .arg(
            Arg::new("Transforms")
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
        )
        .arg(
            Arg::new("Math delimiters")
                .long("math_delims")
                // TODO: What is the default <29-05-21, kunzaatko> //
                .about("`begin`,`end` delimiter to be used in inline math")
                .number_of_values(2)
                .value_names(&["LDELIM", "RDELIM"])
                .multiple(true),
        )
        .arg(
            Arg::new("Displayed math delimiters")
                .long("displaymath_delims")
                // TODO: What is the default <29-05-21, kunzaatko> //
                .about("`begin`,`end` delimiter to be used in displayed math")
                .value_names(&["LDELIM", "RDELIM"])
                .number_of_values(2),
        )
        .arg(
            Arg::new("Skip recrop")
                .long("skip_recrop")
                .about("Skip the recroping before OCR for the image"),
        )
        .arg(
            Arg::new("Confidence threshold")
                .long("confidence_threshold")
                // TODO: What is the default <29-05-21, kunzaatko> //
                .about("Threshold for triggering confidence errors")
                .value_name("NUMBER"),
        )
        .arg(
            Arg::new("Beam size")
                .long("beam_size")
                // TODO: What is default <29-05-21, kunzaatko> //
                .about("Number of results to consider during recognition")
                .value_name("NUMBER"),
        )
        .arg(
            Arg::new("N best")
                .long("n_best")
                // TODO: What is default <29-05-21, kunzaatko> //
                .about("Number of best results to return")
                .value_name("NUMBER"),
        )
        // TODO: Special from string for region <29-05-21, kunzaatko> //
        .arg(
            Arg::new("Region")
                .long("region")
                .about("Region of the image to process")
                .number_of_values(4)
                .value_names(&["TOP_LEFT_X", "TOP_LEFT_Y", "WIDTH", "HEIGHT"]),
        )
        .group(
            ArgGroup::new("Callback")
                .arg("Post")
                .arg("Headers")
                .arg("Reply")
        )
        .arg(
            Arg::new("Post")
                .long("post")
                .about("URL where to post callback")
                .value_name("URL")
        )
        // TODO: Make custom verification for having even number of arguments <29-05-21, kunzaatko> //
        // TODO: Make parsing of $date and $image and so on for the strings in headers <29-05-21, kunzaatko> //
        // TODO: Use custom delimiters for the headers using KEY : VALUE <29-05-21, kunzaatko> //
        .arg(
            Arg::new("Headers")
                .long("headers")
                .about("Key value pairs of headers to make callback post")
                .multiple(true)
                .value_names(&["KEY", "VALUE"])
        )
        // TODO: What are the possible values for this <29-05-21, kunzaatko> //
        .arg(
            Arg::new("Reply")
                .long("reply")
                .about("Sets of values of `reply` field for callback object")
                .multiple(true)
                .value_name("VALUE")
        )
    ;

    // }}}

    let args = App::new("MathpixCLI")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Provides a command line interface for the Mathpix OCR API")
        .subcommand(text_subcommand)
        .subcommand(latex_subcommand)
        .get_matches();
}
