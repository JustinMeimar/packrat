use std::io;
use std::io::{Write, Read};
use std::fs::File;
use tempfile::NamedTempFile;
use crate::model::store::TaskStore;
use crate::ui::view::Transition;
use crate::ui::state::EntryViewState;
use std::process::Command;
use crate::ui::render::renderable::Renderable;

///////////////////////////////////////////////////////////

impl Renderable for EntryViewState {
    
    // A hacky, happy-path implementation for now
    fn render(&mut self) -> io::Result<Transition> {
        
        // get the contents of selected task entry
        let content = &self.task_entry.content;
        
        // open a temporary file 
        let mut tmp_file = NamedTempFile::new()
            .map_err(|e| format!("Failed to create tmpfile: {}", e))
            .unwrap();
        
        // write the contents into the file 
        tmp_file
            .write_all(&content)
            .map_err(|e| format!("Failed to write to temp file: {}", e));

        // open the editor
        let status = Command::new("nvim")
            .arg(tmp_file.path())
            .status()
            .expect("Failed to open editor");

        if !status.success() {
            eprintln!("Neovim exited with an error."); 
        }

        // read the contents back
        let mut content_updated = String::new();
        File::open(&tmp_file)?
            .read_to_string(&mut content_updated)?;

        // synchronize the updates
        self.task_entry.content = content_updated.into_bytes();  
        TaskStore::instance().put(self.task_entry.clone());
        
        Ok(Transition::Pop)
    }
}

