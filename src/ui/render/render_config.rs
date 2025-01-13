use std::io;
use std::io::{Write, Read};
use std::fs::File;
use std::process::Command;
use std::fmt::Debug;
use tempfile::NamedTempFile;
use crate::log::debug_log;
use crate::ui::editor::open_editor;
use crate::model::convert::Storable;
use crate::model::store::TaskStore;
use crate::ui::view::{Transition, View};
use crate::ui::state::{ConfigViewState, PopUpViewState};
use crate::ui::render::renderable::Renderable;

///////////////////////////////////////////////////////////

impl<T> Renderable for ConfigViewState<T> where T: Storable + Debug {

    fn render(&mut self) -> io::Result<Transition> {
        
        let item_config = self.config_item.to_toml().unwrap();  
        let updated_config = open_editor(&item_config.as_bytes()).unwrap();
        
        let transition = match T::from_toml(updated_config) {
            Ok(t) => {
                if self.config_item.is_legal_update_from(&t) {
                    // make the update
                    TaskStore::instance().put(t);
                    Transition::Pop
                } else {
                    Transition::Push(
                        View::PopUpView(
                            PopUpViewState::new(String::from("Invalid"))
                        )
                    )
                }
            },
            Err(_) => {
                let msg = String::from("Failed to parse TOML");
                Transition::Push(
                    View::PopUpView(
                        PopUpViewState::new(msg)
                    )
                )
            }
        };
        
        Ok(transition)
    }
}

