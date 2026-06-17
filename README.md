# Deck++ 

**Deck++** is an open-source, DIY macro-pad ecosystem designed for speed, customization, and ultimate modern, but minimal feel.  

Built to be modular and highly responsive, this project is perfect for developers, streamers, power users, and mechanical keyboard enthusiasts alike looking to implement something like this themselves.  
Or, even if you want to know **how keyboard work under the hood**.

<img width="992" height="682" alt="image" src="https://github.com/user-attachments/assets/b0b59a0d-52e6-4a62-ac61-89b789eb7d24" />

By combining the versatile hardware capabilities of the ESP32 with the blazing-fast, memory-safe performance of a Rust host application, Deck++ bridges the gap between custom hardware keys and desktop automation.

---  

## Story
**Problem:** Existing macro-pads were very costly for me, and the cheap ones didn't necessarily offered good Linux support, or either they don't even support customization on Liux. But I needed one at that time.
**Solution:**  
So, I built a custom macro-pad myselves in my college hackathon.  
Built,
+ Matrix-based 4X3 custom macro-pad from scratch, using ESP32.
+ Macro-pad customization software.  
**Good News:** I **Won** first prize the first Hackathon of my life. Wohoo!!!

## Tech Stack
*   **Hardware / Firmware:** ESP32
*   **Host Software:** Rust, Slint
*   **Communication:** Serial communication

## Key Features
*   **Rust-Powered Performance:** Minimal CPU footprint and near-zero latency execution for macros.
*   **Dynamic Remapping:** Easily rebind keys, or trigger complex system scripts.
*   **Highly Responsive:** Built for instantaneous, zero-lag input execution.
    *   **Near-Zero Latency:** Leverages Rust's raw speed and efficient memory management to process key matrices instantly.
    *   **High-speed Polling:** Built to capture every single keystroke without dropped inputs, even during intense usage or rapid macro firing.
*   **Modular Architecture:**
    +  Hardware Communication < ─ ─ > Configuration < ─ ─ > Action Pool < ─ ─ > UI Layer < - - > OS
*   **Hackable & Open Source:** Designed from the ground up to be extended with custom hardware layouts or new host integrations.

# Building / Running (Linux)  
+ Get Rust from https://rust-lang.org/tools/install/
+ Install prerequiisite by installing `libudev-dev` from ditribution package manager
+ Clone this repository  
+ Run `cargo run` (debug build) or `cargo run --release` (release build) from the project's root directory
+ The executable "Deck" would be generated at Project's `target/debug/` or `/target/release` accordingly

# Features
+ Minimal, pretty User interface for customization.
+ Quick Profile change. Just a single-click selects new profile and activates it globally.
+ Customization to support all keys from standard QWERTY keyboard, so if you got a broken keypad, Just assign the Profile set to working keys when required.
+ Active assign is supported.
  + Modify the existing macro-bindings by unlocking the Profile first.
  + Press Lock icon on Toolbar to unlock and enable modification.
+ Keys real-time testing is provided within the software.
  + Press The Green Test icon on toolbar, and your Pressed keys would be highghted to test binding, or any malfunction with hardware.
  

# Loose ends
+ Project is far from perfect.
+ Starting Deck++ has a precondition of Communication port being active.
  + Modify `comms::listen(String::from("/dev/ttyUSB0"), 115200, tx);` in Project's `/Source/main.rs:80:` to customize the communication port.
+ UI Data is loaded on the fly from Project's `/Data/Profiles.json`.
  + Always run the Standalone Executable from a directory consisting subdirecory "Data" with file "Profiles.json".
+ Right now, **Adding new Profiles directly from UI is not supported**.
  + Add new Profiles as you feel manually by modifying Project's `/Data/Profiles.json`.
 
**Beware: Only Project's `/Source/Comms.rs` is written later on with the help of LLM. All other code, even this `README.md` is entirely human written.**
