use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};
use evdev::{AttributeSet, EventType, InputEvent};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

static VIRTUAL_KEYBOARD: Lazy<Mutex<VirtualDevice>> = Lazy::new(|| {
    let mut keys = AttributeSet::<evdev::Key>::new();

    // Enable keys in the standard keyboard range
    for i in 0..255 {
        keys.insert(evdev::Key::new(i));
    }

    // Using the non-deprecated builder() pattern as requested by the compiler
    let device = VirtualDeviceBuilder::new()
        .expect("Failed to create uinput builder")
        .name("Deck++ Virtual Keyboard")
        .with_keys(&keys)
        .unwrap()
        .build()
        .expect("Failed to build virtual device. Ensure sudo permissions.");

    Mutex::new(device)
});

#[allow(dead_code)]
pub enum Action {
    KeyAction(evdev::Key),
    Screenshot,
    Record,
    OpenUrl(String), // New
    RunCommand(String),
    None,
}

impl Action {
    pub fn execute(&self) {
        match self {
            Action::KeyAction(k) => {
                if let Ok(mut device) = VIRTUAL_KEYBOARD.lock() {
                    // Use .0 to get the raw u16 for the InputEvent constructor
                    let down = InputEvent::new(EventType::KEY, k.code(), 1);
                    let up = InputEvent::new(EventType::KEY, k.code(), 0);
                    let sync = InputEvent::new(EventType::SYNCHRONIZATION, 0, 0);

                    if let Err(e) = device.emit(&[down, sync, up, sync]) {
                        eprintln!("Evdev Error: {}", e);
                    }
                }
            }
            Action::Screenshot => {
                if let Ok(mut device) = VIRTUAL_KEYBOARD.lock() {
                    // Use .0 to get the raw u16 for the InputEvent constructor
                    let down = InputEvent::new(EventType::KEY, evdev::Key::KEY_PRINT.code(), 1);
                    let up = InputEvent::new(EventType::KEY, evdev::Key::KEY_PRINT.code(), 0);
                    let sync = InputEvent::new(EventType::SYNCHRONIZATION, 0, 0);

                    if let Err(e) = device.emit(&[down, sync, up, sync]) {
                        eprintln!("Evdev Error: {}", e);
                    }

                    let down = InputEvent::new(EventType::KEY, evdev::Key::KEY_ENTER.code(), 1);
                    let up = InputEvent::new(EventType::KEY, evdev::Key::KEY_ENTER.code(), 0);
                    let sync = InputEvent::new(EventType::SYNCHRONIZATION, 0, 0);

                    if let Err(e) = device.emit(&[down, sync, up, sync]) {
                        eprintln!("Evdev Error: {}", e);
                    }
                }
            }
            Action::Record => {}
            Action::OpenUrl(url) => {
                let _ = std::process::Command::new("xdg-open").arg(url).spawn();
            }
            Action::RunCommand(cmd) => {
                let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            }
            Action::None => {
                println!("No action to perform");
            }
        }
    }
}

pub struct Map {
    pub map: HashMap<String, Action>,
}

impl Map {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn default() -> Self {
        let mut map = HashMap::new();

        // Fully qualified evdev::Key constants to ensure the compiler finds them
        let keys = [
            evdev::Key::KEY_0,
            evdev::Key::KEY_1,
            evdev::Key::KEY_2,
            evdev::Key::KEY_3,
            evdev::Key::KEY_4,
            evdev::Key::KEY_5,
            evdev::Key::KEY_6,
            evdev::Key::KEY_7,
            evdev::Key::KEY_8,
            evdev::Key::KEY_9,
        ];

        for (i, &k) in keys.iter().enumerate() {
            map.insert(format!("Num {i}"), Action::KeyAction(k));
        }

        {
            let keys = [
                (evdev::Key::KEY_A, 'A'),
                (evdev::Key::KEY_B, 'B'),
                (evdev::Key::KEY_C, 'C'),
                (evdev::Key::KEY_D, 'D'),
                (evdev::Key::KEY_E, 'E'),
                (evdev::Key::KEY_F, 'F'),
                (evdev::Key::KEY_G, 'G'),
                (evdev::Key::KEY_H, 'H'),
                (evdev::Key::KEY_I, 'I'),
                (evdev::Key::KEY_J, 'J'),
                (evdev::Key::KEY_K, 'K'),
                (evdev::Key::KEY_L, 'L'),
                (evdev::Key::KEY_M, 'M'),
                (evdev::Key::KEY_N, 'N'),
                (evdev::Key::KEY_O, 'O'),
                (evdev::Key::KEY_P, 'P'),
                (evdev::Key::KEY_Q, 'Q'),
                (evdev::Key::KEY_R, 'R'),
                (evdev::Key::KEY_S, 'S'),
                (evdev::Key::KEY_T, 'T'),
                (evdev::Key::KEY_U, 'U'),
                (evdev::Key::KEY_V, 'V'),
                (evdev::Key::KEY_W, 'W'),
                (evdev::Key::KEY_X, 'X'),
                (evdev::Key::KEY_Y, 'Y'),
                (evdev::Key::KEY_Z, 'Z'),
            ];

            for (key, letter) in keys {
                map.insert(format!("Letter {letter}"), Action::KeyAction(key));
            }
        }

        map.insert(String::from("Screenshot"), Action::Screenshot);
        map.insert(String::from("Record Screen"), Action::Record);
        map.insert(
            String::from("Enter"),
            Action::KeyAction(evdev::Key::KEY_ENTER),
        );
        map.insert(
            String::from("Volume +"),
            Action::KeyAction(evdev::Key::KEY_VOLUMEUP),
        );
        map.insert(
            String::from("Volume -"),
            Action::KeyAction(evdev::Key::KEY_VOLUMEDOWN),
        );
        map.insert(
            String::from("Volume Mute"),
            Action::KeyAction(evdev::Key::KEY_MUTE),
        );
        map.insert(
            String::from("Mic Mute"),
            Action::KeyAction(evdev::Key::KEY_MICMUTE),
        );
        map.insert(
            String::from("Up Arrow"),
            Action::KeyAction(evdev::Key::KEY_UP),
        );
        map.insert(
            String::from("Down Arrow"),
            Action::KeyAction(evdev::Key::KEY_DOWN),
        );
        map.insert(
            String::from("Left Arrow"),
            Action::KeyAction(evdev::Key::KEY_LEFT),
        );
        map.insert(
            String::from("Right Arrow"),
            Action::KeyAction(evdev::Key::KEY_RIGHT),
        );
        map.insert(
            String::from("Tab Key"),
            Action::KeyAction(evdev::Key::KEY_TAB),
        );
        map.insert(
            String::from("Back Key"),
            Action::KeyAction(evdev::Key::KEY_BACKSPACE),
        );
        map.insert(
            String::from("Camera"),
            Action::KeyAction(evdev::Key::KEY_CAMERA),
        );
        // "Github", "Leetcode", "Youtube", "Reddit", "Slint", "BMW"
        map.insert(
            String::from("Github"),
            Action::OpenUrl("https://github.com/".to_string()),
        );
        map.insert(
            String::from("Leetcode"),
            Action::OpenUrl("https://leetcode.com/".to_string()),
        );
        map.insert(
            String::from("Youtube"),
            Action::OpenUrl("https://youtube.com/".to_string()),
        );
        map.insert(
            String::from("Reddit"),
            Action::OpenUrl("https://reddit.com/".to_string()),
        );
        map.insert(
            String::from("Slint"),
            Action::OpenUrl("https://slint.dev/".to_string()),
        );
        map.insert(
            String::from("BMW"),
            Action::OpenUrl("https://www.bmw.in/en/index.html".to_string()),
        );
        map.insert(String::from("Unset"), Action::None);

        Self { map }
    }
}
