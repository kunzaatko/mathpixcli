extern crate mathpixapi;

#[cfg(clap)]
use clap::{App, Arg, crate_version};

#[cfg(clap)]
fn main() {
    let args = App::new("MathpixCLI")
        .version(crate_version!())
        .author("Martin Kunz <martinkunz@email.cz")
        .about("Provides a command line interface for the Mathpix OCR API")
        .subcommand(
            App::new("text")
                .about("Text endpoint for for the Mathpix API")
                .arg(
                    Arg::new("formats")
                        .short('f')
                        .long("formats")
                        .about("List of formats from `text`, `data`, `html` and `latex_styled`")
                        .value_name("FORMAT")
                        .multiple(true)
                        .takes_value(true),
                ),
        )
        .get_matches();
}
