use crate::profile::Profile;
use slint::SharedString;
use slint::{ModelRc, VecModel};
use std::{fs, path::Path, rc::Rc};

pub struct Config {
    pub profiles: Vec<Profile>,
}

impl Config {
    pub fn create_from(file_path: &Path) -> Self {
        if let Ok(content) = fs::read_to_string(file_path) {
            match serde_json::from_str(&content) {
                Ok(config) => Self { profiles: config },
                Err(e) => {
                    println!("Error : {e} :: Error while parsing Config.json file");
                    Self { profiles: vec![] }
                }
            }
        } else {
            Self { profiles: vec![] }
        }
    }

    pub fn save_to(&self, file_path: &Path) {
        if let Ok(data) = serde_json::to_string_pretty(&self.profiles) {
            match fs::write(file_path, data) {
                Ok(_) => println!("Saved Configs to {file_path:?}"),
                Err(err) => println!("Error: {err:?} :: Failed to write Configs to {file_path:?}"),
            }
        }
    }

    pub fn to_slint(&self) -> ModelRc<crate::Profile_data> {
        let data: Vec<crate::Profile_data> = self
            .profiles
            .iter()
            .map(|p| {
                // 1. Collect entries into a Vec so we can sort them
                let mut sorted_bindings: Vec<_> = p.bindings.iter().collect();

                // 2. Sort by the key (the trigger ID)
                // Assuming the key is a type that implements Ord (like usize or u8)
                sorted_bindings.sort_by_key(|&(trigger, _)| trigger);

                // 3. Map to SharedString now that they are in order
                let mapping_vec: Vec<SharedString> = sorted_bindings
                    .into_iter()
                    .map(|(_trigger, action_name)| SharedString::from(action_name))
                    .collect();

                let mapping_model = ModelRc::from(Rc::new(VecModel::from(mapping_vec)));

                crate::Profile_data {
                    info: crate::Profile_info {
                        name: p.info.name.clone().into(),
                        cre_date: p.info.created.clone().into(),
                        mod_date: p.info.modified.clone().into(),
                        keys: p.bindings.len() as i32,
                        active: false,
                    },
                    mapping: mapping_model,
                }
            })
            .collect();

        ModelRc::from(Rc::new(VecModel::from(data)))
    }
}
