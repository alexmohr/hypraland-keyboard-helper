use hypraland_keyboard_helper::cli::CommandLineArgs;
use hyprland::data::{Devices, Keyboard};
use hyprland::event_listener::EventListener;
use hyprland::prelude::HyprData;
use hyprland::shared::HyprError;
use notify_rust::{Hint, Notification};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CommandLineArgs::new();
    if Some(true) == args.switch_layout {
        next_layout()?;
    }
    if Some(true) == args.print_layouts {
        print_layouts(&args.map)?;
    }
    if Some(true) == args.listen {
        // use loop, so we reconnect in case connection is lost.
        loop {
            listen_layout_changed(&args.map)?;
            sleep(Duration::from_millis(500));
        }
    }

    Ok(())
}

fn next_layout() -> Result<(), HyprError> {
    for keyboard in fetch_main_keyboards()? {
        if let Err(e) = hyprland::ctl::switch_xkb_layout::call(
            &keyboard.name,
            hyprland::ctl::switch_xkb_layout::SwitchXKBLayoutCmdTypes::Next,
        ) {
            println!(
                "failed to change layout for keyboard {}, {:?}",
                &keyboard.name, e
            );
            continue;
        }
        let new_keyboard = match fetch_keyboard(&keyboard) {
            Some(new_keyboard) => new_keyboard,
            None => {
                println!("failed find new layout for keyboard {}", &keyboard.name);
                continue;
            }
        };

        Notification::new()
            .summary(&format!(
                "Switched layout to {}",
                new_keyboard.active_keymap
            ))
            .icon("dialog-information")
            // replace old notification with new one
            .hint(Hint::Custom(
                String::from("x-canonical-private-synchronous"),
                String::from("anything"),
            ))
            .show()
            .unwrap();
    }
    Ok(())
}

fn map_layouts(map: &Option<String>, active_keymap: &str) -> String {
    map.as_deref()
        .and_then(|map_value| {
            map_value
                .split(';')
                .find(|kb| kb.starts_with(active_keymap))
                .and_then(|maps| maps.split_once('=').map(|(_, v)| v.to_string()))
        })
        .unwrap_or_else(|| active_keymap.to_string())
}


fn print_layouts(map: &Option<String>) -> Result<(), HyprError> {
    for keyboard in fetch_main_keyboards()?.iter() {
        println!("{}", map_layouts(map, &keyboard.active_keymap));
    }
    Ok(())
}

fn listen_layout_changed(map: &Option<String>) -> Result<(), HyprError> {
    let mut listener = EventListener::new();
    let layouts = map.clone();
    listener.add_layout_changed_handler(move |data| {
        let keyboard = match fetch_keyboard_by_name(&data.keyboard_name) {
            Some(keyboard) => keyboard,
            None => {
                println!("Can't find keyboard with given name {}", data.keyboard_name);
                return;
            }
        };

        if !keyboard.main || keyboard.active_keymap.to_lowercase().contains("error") {
            return;
        }

        println!("keyboard={}, keymap={}", keyboard.name, keyboard.active_keymap);

        println!("{}", map_layouts(&layouts, &keyboard.active_keymap));
    });

    Ok(listener.start_listener()?)
}

fn fetch_main_keyboards() -> Result<Vec<Keyboard>, HyprError> {
    let devices = Devices::get()?;
    let keyboards = devices
        .keyboards
        .into_iter()
        .filter(|keyboard| keyboard.main)
        .collect();
    Ok(keyboards)
}

fn fetch_keyboard(search: &Keyboard) -> Option<Keyboard> {
    fetch_keyboard_by_name(&search.name)
}

fn fetch_keyboard_by_name(search: &str) -> Option<Keyboard> {
    let devices = Devices::get();
    match devices {
        Ok(devices) => devices
            .keyboards
            .into_iter()
            .find(|keyboard| keyboard.name == search),
        Err(e) => {
            println!("failed to get keyboard with given name {}, error=", e);
            None
        }
    }
}
