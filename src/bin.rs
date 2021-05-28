extern crate mathpixapi;

use clap::{crate_authors, crate_version, App, Arg, ArgGroup};

fn main() {
    let args = App::new("MathpixCLI")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Provides a command line interface for the Mathpix OCR API")
        .subcommand(
            App::new("text")
                .about("Text endpoint for for the Mathpix API")
                .arg(
                    Arg::new("Text Formats")
                        .short('f')
                        .long("formats")
                        .about("List of formats required in the output")
                        .value_name("FORMAT")
                        .possible_values(&["text", "html", "data", "latex_styled"])
                        .multiple(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("Data Options")
                        .long("data_options")
                        .about("List of data options for the outputs")
                        .value_name("OPTION")
                        .possible_values(&["include_svg", "include_table_html", "include_latex", "include_tsv", "include_acsiimath", "include_mathml"])
                        .multiple(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("Include Detected Alphabets")
                        .about("Return detected alphabets")
                        .long("include_detected_alphabets")
                )
                .arg(
                    Arg::new("Alphabets allowed")
                        .about("Specify which alphabets are allowed for the OCR")
                        .long("alphabets_allowed")
                        .value_name("ALPHABET")
                        .takes_value(true)
                        .multiple(true)
                        .possible_values(&["en","hi","zh","ja","ko","ru","th"])
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
                )
            )
        .get_matches();
}
