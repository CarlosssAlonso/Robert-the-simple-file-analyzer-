use clap::Parser;
use std::fs;
use std::io::ErrorKind;
use colored::*;
// cargo install --path . --force

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    // si pongo --file long si pongo -f short
    #[arg(value_name = "FILE", num_args = 1)]
    file: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let args = Args::parse();
    let contenido = match fs::read_to_string(&args.file)
    {
        Ok(contenido) => contenido,

        Err(e) if e.kind() == ErrorKind::IsADirectory => {
            eprintln!("{}", format!("{} es un directorio, no un archivo", args.file).red());
            return Ok(());
        } 
        Err(e) => {
            eprintln!("{}", format!("Error al leer el archivo: {}", e).red());
            return Ok(());
        }
    };
    let lineas = contenido.lines().count();
    let palabras = contenido.split_whitespace().count();
    let bytes = contenido.len();
    let _tamano = bytes as f64 / 1024.0;
    let _tamano_mb = bytes as f64 / 1024.0 / 1024.0;
    match lineas {
        0 => println!("{}", format!("No hay lineas").red()),
        _ => println!("{}", format!("{} lineas", lineas).green()),
    }
    match palabras {
        0 => println!("{}", format!("No hay palabras").red()),
         _ => println!("{}", format!("{} palabras", palabras).green()),
    }
    match _tamano_mb {
        0.0 => println!("{}", format!("No hay mb").red()),
        _ => println!("{}", format!("{:.4} mb", _tamano_mb).green()),
    }
    Ok(())
}