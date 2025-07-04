use std::{
    fs,
    io::{Read, stdin},
};

use json2nix::{Json2NixConfig, json2nix};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, bin_name = "json2nix", long_about = None)]
struct Args {
    #[arg(index = 1, help = "The input JSON file. Use '-' to read from stdin until EOF.")]
    input: String,

    #[arg(short, long, help = "The output file. If not specified, the result is printed to stdout.")]
    output: Option<String>,

    #[arg(short, long, default_value_t = 2, help = "The number of spaces for indentation.")]
    indentation: usize,

    #[arg(long, default_value_t = 0, help = "The number spaces to indent the whole output with.")]
    initial_indentation: usize,

    #[arg(short, long, default_value_t = false, help = "Compact the keys in sets if they contain only one value.")]
    compact_set_keys: bool,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let json_result = match args.input.as_str() {
        "-" => {
            let mut input = String::new();
            stdin().read_to_string(&mut input).map_err(|err| err.to_string()).map(|_| input)
        }
        file_name => fs::read_to_string(file_name).map_err(|err| err.to_string()),
    };

    let json = json_result.map_err(|err| format!("Could read the input from '{}' because of: {}", args.input, err))?;

    let config = Json2NixConfig::new(args.initial_indentation, args.indentation, args.compact_set_keys);
    let nix = json2nix(&json, &config).map_err(|err| format!("Could not convert the input to Nix: {}", err))?;

    match args.output {
        Some(file_name) => {
            fs::write(&file_name, nix).map_err(|err| format!("Could not write the generated Nix expression to '{}': {}", file_name, err))?;
        }
        None => {
            println!("{}", nix)
        }
    };

    Ok(())
}
