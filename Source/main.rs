use std::{
    path::Path,
    sync::{Arc, RwLock, mpsc},
    thread,
};

use crate::action::Map;
use comms::DeckTrigger;

use crate::{config::Config, profile::Profile};
mod action;
mod comms;
mod config;
mod profile;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = App::new()?;
    let config = Arc::new(RwLock::new(Config::create_from(Path::new(
        "Data/Profiles.json",
    ))));

    let profile = Arc::new(RwLock::new(Profile::empty()));

    {
        let cfg = config.read().unwrap();
        if !cfg.profiles.is_empty() {
            ui.set_data(cfg.to_slint());
            if let Ok(mut active_guard) = profile.write() {
                *active_guard = cfg.profiles[0].clone();
            }
        }
    }

    let map = Arc::new(Map::default());

    // Callbacks setup
    let config_for_switch = Arc::clone(&config);
    let active_for_switch = Arc::clone(&profile);

    // Swapping the active profile
    ui.on_profile_changed(move |index| {
        let idx = index as usize;
        let cfg = config_for_switch.read().unwrap();

        if let Some(new_prof) = cfg.profiles.get(idx) {
            if let Ok(mut active_guard) = active_for_switch.write() {
                *active_guard = new_prof.clone();
                println!("Switched hardware to profile: {}", new_prof.info.name);
            }
        }
    });

    let ui_weak_mod = ui.as_weak();
    let config_for_mod = Arc::clone(&config);
    let active_for_mod = Arc::clone(&profile);

    // Swapping the button functionality
    ui.on_modify_map(move |prof_idx_str, btn_idx_str, new_value| {
        let p_idx = prof_idx_str.parse::<usize>().unwrap_or(0);
        let trigger = btn_idx_str.parse::<usize>().unwrap_or(0);

        if let Ok(mut cfg) = config_for_mod.write() {
            {
                if let Some(profile) = cfg.profiles.get_mut(p_idx) {
                    profile
                        .bindings
                        .insert(trigger as u8, new_value.to_string());
                    if let Ok(mut active_guard) = active_for_mod.write() {
                        if active_guard.info.name == profile.info.name {
                            *active_guard = profile.clone();
                        }
                    }
                }
            }
            cfg.save_to(Path::new("Data/Profiles.json"));
        }
        if let Some(ui) = ui_weak_mod.upgrade() {
            if let Ok(cfg) = config_for_mod.read() {
                ui.set_data(cfg.to_slint());
            }
        }
    });

    // Creating Esp communication channel
    let (tx, rx) = mpsc::channel::<DeckTrigger>();
    comms::listen(String::from("/dev/ttyUSB0"), 115200, tx);

    let ui_events_handle = ui.as_weak(); // UI handle for event handling
    let map_events_handle = Arc::clone(&map);
    let profile_events_handle = Arc::clone(&profile);

    // Starting Decktrigger handling thread
    thread::spawn(move || {
        for event in rx {
            match profile_events_handle.read() {
                Ok(profile) => {
                    if let Some(action_name) = profile.bindings.get(&event) {
                        if let Some(action) = map_events_handle.map.get(action_name) {
                            action.execute();
                        } else {
                            println!("Error :: Action is not mapped to any DeckAction");
                        }
                    } else {
                        println!(
                            "Error :: Current Profile has no Action mapped for Trigger Esp: {event}"
                        );
                    }
                }
                Err(_) => {}
            }

            let ui_handle = ui_events_handle.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_active(event as i32);
                }
            });
        }
    });

    ui.run()
}
