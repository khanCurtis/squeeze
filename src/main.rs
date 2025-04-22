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
