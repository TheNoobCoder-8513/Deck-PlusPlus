use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProfileInfo {
    pub name: String,
    pub created: String,
    pub modified: String,
    pub enabled_keys: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Profile {
    pub info: ProfileInfo,  // Profile info
    pub bindings: Bindings, // Holds bindings of ESP Key input and Action name
}

type Bindings = HashMap<u8, String>;

impl Profile {
    #[allow(dead_code)]
    pub fn new(info: ProfileInfo, bindings: Bindings) -> Self {
        Self { info, bindings }
    }

    pub fn empty() -> Self {
        Self {
            info: ProfileInfo {
                name: "".to_string(),
                created: "".to_string(),
                modified: "".to_string(),
                enabled_keys: 0,
            },
            bindings: HashMap::new(),
        }
    }
}
