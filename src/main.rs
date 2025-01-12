use packrat::model::store::TaskStore;
use packrat::ui::view;
use std::{error::Error, io::stdout};

/// Used to manage 
use crossterm::{
    execute,
    terminal::{enable_raw_mode,disable_raw_mode,
               EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Show, Hide},
};

///////////////////////////////////////////////////////////

fn main() -> Result<(), Box<dyn Error>> {
    
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
        
    TaskStore::instance().dump();
    Ok(())
}

