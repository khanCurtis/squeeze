/* LOGO IDEA
                    ___ ____    
  ___ __ _ _  _ ___| __|_  /___ 
 (_-</ _` | || / -_) _| / // -_)
 /__/\__, |\_,_\___|___/___\___|
        |_|                     
*/

/*TODO
create function to compress (files and directories)
create fnuction to decompress
add progress bar (indicatif(?))

*/

use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compress {
        input: String,
        output: String,
    },

    Decompress {
        input: String,
        output: String,
    },
}

fn compress(input_path: &str, output_path: &str) -> io::Result<()> {
    let input = File::open(input_path)?;
    let reader = BufReader::new(input);
    let output = File::create(output_path)?;
    let writer = BufWriter::new(output);
    let mut encoder = GzEncoder::new(writer, Compression::default());
    let mut buffer = [0u8; 1024];

    let mut reader = reader;
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        encoder.write_all(&buffer[..count])?;
    }

    encoder.finish()?;
    Ok(())
}

fn decompress(input_path: &str, output_path: &str) -> io::Result<()> {
    let input = File::open(input_path)?;
    let output = File::create(output_path)?;
    let mut writer = BufWriter::new(output);
    let mut decoder = GzDecoder::new(input);
    let mut buffer = [0u8; 1024];

    loop {
        let count = decoder.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        writer.write_all(&buffer[..count])?;
    }

    writer.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Compress { input, output } => compress(&input, &output),
        Commands::Decompress { input, output } => decompress(&input, &output),
    }
}
