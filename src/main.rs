use clap ::{Parser, Subcommand};

use std::fs::File;
use std::io::{BufReader,BufWriter, copy, Result};


use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;


#[derive(Parser)]
#[command(name = "Encryptor")]
#[command(about = "A Simple file compressor and decompressor", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command : Commands,
}

#[derive(Subcommand)]
enum Commands{
    Compress  {
        input : String,
        output : String,
    },
    Decrypt {
        input: String,
        output : String,
    },
}

fn compress_file(input_path: &str, output_path: &str) -> Result<()>{
    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;

    let mut reader = BufReader::new(input_file);
    let writer = BufWriter::new(output_file);

    let mut encoder  = GzEncoder::new(writer,Compression::default());

    copy(&mut reader, &mut encoder)?;

    Ok(())
}

fn decompress_file(input_path: &str, output_path: &str)->Result<()>{
    let input_file = File::open(input_path)?;
    let mut decoder = GzDecoder::new(BufReader::new(input_file));

    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    copy(&mut decoder, &mut writer)?;
    
    Ok(())
}

fn main() {
    let cli = Cli ::parse();

    match &cli.command {
        Commands :: Compress {input, output}=>{
            println!("compressing {} to {}", input, output);
            if let Err(e) = compress_file(input, output){
                eprintln!("compression failed {}",e);
                
            } else {
                println!("compression success");
            }
        }
        Commands::Decrypt { input, output } => {
            println!("Decrypting {} to {}", input, output);
            if let Err(e) = decompress_file(input, output) {
                eprintln!(" Decompression failed: {}", e);
            } else {
                println!(" Decompression successful.");
            }
        }
        
    }
}
