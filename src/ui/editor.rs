use std::io;
use std::io::{Write, Read};
use std::fs::File;
use tempfile::NamedTempFile;
use std::process::Command;

///////////////////////////////////////////////////////////

pub fn open_editor(bytes: &[u8]) -> io::Result<String> {
    
    // open a temporary file 
    let mut tmp_file = NamedTempFile::new()
        .map_err(|e| format!("Failed to create tmpfile: {}", e))
        .unwrap();
    
    // write the contents into the file 
    tmp_file
        .write_all(bytes)
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
    
    Ok(content_updated)
}

