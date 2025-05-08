use rdev::{Event, EventType, Key, listen};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

// This will track the currently held modifier keys
pub fn use_key() {
    let modifiers = Arc::new(Mutex::new(HashSet::new()));
    let modifiers_clone = modifiers.clone();

    let callback = move |event: Event| {
        let mut mods = modifiers_clone.lock().unwrap();

        match event.event_type {
            EventType::KeyPress(key) => match key {
                Key::Alt
                | Key::ShiftLeft
                | Key::ShiftRight
                | Key::ControlLeft
                | Key::ControlRight
                | Key::MetaLeft
                | Key::MetaRight => {
                    mods.insert(key);
                }
                Key::Space => {
                    if mods.contains(&Key::Alt) {
                        println!("🪄 Alt + Space detected!");
                    }
                }
                _ => {}
            },
            EventType::KeyRelease(key) => {
                mods.remove(&key);
            }
            _ => {}
        }
    };

    if let Err(e) = listen(callback) {
        eprintln!("Error: {:?}", e);
    }
}
