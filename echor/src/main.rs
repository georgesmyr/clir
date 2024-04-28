use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "Text to be echoed.")]
    text: Vec<String>,

    #[arg(short = 'n', help = "Add new line at the end of the text")]
    no_newline: bool,

    #[arg(short = 'o', help = "Filename to output the text")]
    output_file: Option<String>,

    #[arg(
        short = 'a',
        long = "append",
        help = "Append text to the output instead of overwriting."
    )]
    append: bool,
}

fn main() -> io::Result<()> {
    // Parse arguments
    let args = Args::parse();

    let mut output_text = args.text.join(" ");
    if !args.no_newline {
        output_text = output_text + "\n";
    }

    if let Some(file_name) = args.output_file {
        let mut file_mode = if args.append {
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(file_name)?
        } else {
            File::create(file_name)?
        };

        writeln!(file_mode, "{}", output_text)?;
    } else {
        print!("{}", output_text);
    }

    Ok(())
}
