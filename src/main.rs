use std::error::Error;
use hypraland_keyboard_helper::cli::CommandLineArgs;
use hyprland::data::{Devices, Keyboard};
use hyprland::prelude::HyprData;
use notify_rust::{Hint, Notification};

fn main() -> Result<(), Box<dyn Error>> {
    let args = CommandLineArgs::new();
    if Some(true) == args.switch_layout {
        next_layout()?;
    }
    if Some(true) == args.print_layouts {
        print_layouts(&args.map)?;
    }

    Ok(())
}

fn next_layout() -> Result<(), Box<dyn Error>> {
    for keyboard in fetch_main_keyboards()? {
        if let Err(e) = hyprland::ctl::switch_xkb_layout::call(&keyboard.name, hyprland::ctl::switch_xkb_layout::SwitchXKBLayoutCmdTypes::Next) {
            println!("failed to change layout for keyboard {}, {:?}", &keyboard.name, e);
            continue;
        }
        let new_keyboard = match fetch_keyboard(&keyboard) {
            Ok(new_keyboard) => {
                if new_keyboard.is_some() {
                    new_keyboard.unwrap()
                } else {
                    continue;
                }
            }
            Err(e) => {
                eprintln!("failed to change layout for keyboard {}, {:?}", &keyboard.name, e);
                continue
            }
        };

        Notification::new()
            .summary(&format!("Switched layout to {}", new_keyboard.active_keymap))
            .icon("dialog-information") // Optional: specify an icon
            // replace old notification with new one
            .hint(Hint::Custom(String::from("x-canonical-private-synchronous"), String::from("anything")))
            .show()
            .unwrap();
    }
    Ok(())
}

fn print_layouts(map_layout: &Option<String>) -> Result<(), Box<dyn Error>> {
    for keyboard in fetch_main_keyboards()?.iter() {
        if let Some(map_value) = map_layout {
            let maps = map_value.split(";").find(|kb| kb.to_string().starts_with(&keyboard.active_keymap));
            if maps.is_some() {
                let value = maps.unwrap().split("=").collect::<Vec<&str>>();
                if value.len() == 2 {
                    println!("{}", value[1]);
                    continue;
                }
            }
        }
        println!("{}", keyboard.active_keymap);
    }
    Ok(())
}

fn fetch_main_keyboards() -> Result<Vec<Keyboard>, Box<dyn Error>> {
    let devices = Devices::get()?;
    let keyboards = devices
        .keyboards
        .into_iter()
        .filter(|keyboard | keyboard.main)
        .collect();
    Ok(keyboards)
}

fn fetch_keyboard(search: &Keyboard) -> Result<Option<Keyboard>, Box<dyn std::error::Error>> {
    let devices = Devices::get()?;
    let keyboard: Option<Keyboard> = devices
        .keyboards
        .into_iter()
        .filter(|keyboard| keyboard.name == search.name)
        .next();
    Ok(keyboard)
}
