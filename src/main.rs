/// main.rs

use std::{error::Error, io::stdout};
use packrat::cli::{CLI, Mode};
use clap::Parser;

///////////////////////////////////////////////////////////

fn main() -> Result<(), Box<dyn Error>> {
   
    let cli = CLI::parse_with_default();   
    
    cli.mode.unwrap().run()?; 

    Ok(())
}

