#![warn(clippy::pedantic)]
#![warn(clippy::all)]
#![warn(clippy::nursery)]
#![allow(clippy::single_call_fn)]
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
        print_layouts(args.map.as_ref())?;
    }
    if Some(true) == args.listen {
        // use loop, so we reconnect in case connection is lost.
        loop {
            listen_layout_changed(args.map.as_ref())?;
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

        let Some(new_keyboard) = fetch_keyboard(&keyboard) else {
            println!("failed find new layout for keyboard {}", &keyboard.name);
            continue;
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

fn map_layouts(map: Option<&String>, active_keymap: &str) -> String {
    map.and_then(|map_value| {
            map_value
                .split(';')
                .find(|kb| kb.starts_with(active_keymap))
                .and_then(|maps| maps.split_once('=').map(|(_, v)| v.to_string()))
        })
        .unwrap_or_else(|| active_keymap.to_string())
}


fn print_layouts(map: Option<&String>) -> Result<(), HyprError> {
    fetch_main_keyboards()?.iter().for_each(|keyboard| {
        println!("{}", map_layouts(map, &keyboard.active_keymap));
    });
    Ok(())
}

fn listen_layout_changed(layout_map: Option<&String>) -> Result<(), HyprError> {
    let mut listener = EventListener::new();
    let layouts = layout_map.map(String::to_owned);
    listener.add_layout_changed_handler(move |data| {
        if let Some(k) = fetch_keyboard_by_name(&data.keyboard_name)
            .filter(|k| k.main)
            .filter(|k| !k.active_keymap.to_lowercase().contains("error"))
        {
            println!("{}", map_layouts(layouts.as_ref(), &k.active_keymap));
        }
    });

    listener.start_listener()
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
            println!("failed to get keyboard with given name {e}, error=");
            None
        }
    }
}
