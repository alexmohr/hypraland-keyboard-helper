use hypraland_keyboard_helper::cli::CommandLineArgs;
use hyprland::data::{Devices, Keyboard};
use hyprland::event_listener::EventListener;
use hyprland::prelude::HyprData;
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

fn next_layout() -> Result<(), Box<dyn Error>> {
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
            Ok(Some(new_keyboard)) => new_keyboard,
            Ok(None) => continue,
            Err(e) => {
                eprintln!(
                    "failed to change layout for keyboard {}, {:?}",
                    &keyboard.name, e
                );
                continue;
            }
        };

        Notification::new()
            .summary(&format!(
                "Switched layout to {}",
                new_keyboard.active_keymap
            ))
            .icon("dialog-information") // Optional: specify an icon
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
    if let Some(map_value) = map {
        let maps = map_value
            .split(";")
            .find(|kb| kb.to_string().starts_with(active_keymap));
        if maps.is_some() {
            let value = maps.unwrap().split("=").collect::<Vec<&str>>();
            if value.len() == 2 {
                return value[1].to_string();
            }
        }
    }
    active_keymap.to_string()
}

fn print_layouts(map: &Option<String>) -> Result<(), Box<dyn Error>> {
    for keyboard in fetch_main_keyboards()?.iter() {
        println!("{}", map_layouts(map, &keyboard.active_keymap));
    }
    Ok(())
}

fn listen_layout_changed(map: &Option<String>) -> Result<(), Box<dyn Error>> {
    let mut listener = EventListener::new(); // creates a new listener
    // add a event handler which will be run when this event happens
    let layouts = map.clone();
    listener.add_layout_changed_handler(move |data| {
        if data.layout_name.to_lowercase().contains("error") {
            if let Err(e) = print_layouts(&layouts) {
                println!("failed to print layouts: {}", e);
            }
        } else {
            println!("{}", map_layouts(&layouts, &data.layout_name));
        }
    });
    Ok(listener.start_listener()?)
}

fn fetch_main_keyboards() -> Result<Vec<Keyboard>, Box<dyn Error>> {
    let devices = Devices::get()?;
    let keyboards = devices
        .keyboards
        .into_iter()
        .filter(|keyboard| keyboard.main)
        .collect();
    Ok(keyboards)
}

fn fetch_keyboard(search: &Keyboard) -> Result<Option<Keyboard>, Box<dyn Error>> {
    let devices = Devices::get()?;
    let keyboard: Option<Keyboard> = devices
        .keyboards
        .into_iter()
        .find(|keyboard| keyboard.name == search.name);
    Ok(keyboard)
}
