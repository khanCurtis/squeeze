use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    compress {
        input: String,
        output: String,
    },

    decompress {
        input: String,
        output: String,
    },
}

fn compress() {

}

fn decompress() {

}

fn main() {
    println!("Hello, world!");
}
