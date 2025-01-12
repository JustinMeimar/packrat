/// cli.rs

use clap::{Parser, Subcommand};
use std::{error::Error, io::{self, stdout}};
use crate::model::store::TaskStore;
use crate::ui::view;
use crossterm::{
    execute,
    terminal::{enable_raw_mode,disable_raw_mode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Show, Hide},
};
///////////////////////////////////////////////////////////

#[derive(Subcommand, Debug)]
pub enum Mode {
    // Start packrat as normal
    Regular,
    // Export the database
    Export {
        #[arg(value_parser = ["CSV", "JSON"], help = "export format.")]
        export_type: String,

        #[arg(help = "Output file.")]
        file: String,
    },
}

#[derive(Debug, Parser)]
#[command(
    name = "packrat",
    about = "Packrat - An interactive habit tracker"
)]
pub struct CLI {
    
    /// sub command
    #[command(subcommand)]
    pub mode: Option<Mode>
}

impl CLI {

    /// Let the Regular mode be set if no subcommand is used
    pub fn parse_with_default() -> Self {
        let mut cli = CLI::parse();
        if cli.mode.is_none() {
            cli.mode = Some(Mode::Regular);
        }
        cli
    }
}

impl Mode {
    
    pub fn run(&self) -> Result<(), Box<dyn Error>> { 
        match self {
            Mode::Regular => {
                // enter raw mode
                enable_raw_mode()?;

                // switch to alternate screen, hide cursor, etc.
                execute!(
                    stdout(),
                    EnterAlternateScreen,
                    Hide
                )?;
                   
                // run the app
                let mut app = view::App::new();    
                app.run();

                // disable raw mode before exit
                disable_raw_mode()?;

                // leave alternate screen, show cursor again
                execute!(
                    stdout(),
                    LeaveAlternateScreen,
                    Show
                )?;
            }
            
            Mode::Export {export_type, file} => {
                match TaskStore::instance().to_csv(file) {
                    Ok(_) => {
                        println!("CSV written to: {}", file);
                    }
                    Err(e) => {
                        eprintln!("Failed to create CSV: {}", e);
                    }
                }
            }
        }
        Ok(())
    }
}

