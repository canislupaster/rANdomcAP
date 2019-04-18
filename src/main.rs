extern crate autopilot;
extern crate systray;
extern crate multiinput;
extern crate rand;

use multiinput::*;
use autopilot::*;
use rand::prelude::*;
use std::sync::{atomic::{Ordering, AtomicBool}, Arc};

fn get_icon(on: &bool) -> String {
    match on { true => "on.png", false => "off.png" }.to_owned()
}

fn main() {
    let mut toggle = AtomicBool::new(false);

    let mut tray = systray::Application::new().unwrap();
    tray.set_icon_from_file(&"./resources/on.png".to_owned()).unwrap();

//    tray.add_menu_item(&"Toggle".to_owned(), move |app| {
//        tray_toggle.store(!tray_toggle.load(Ordering::Relaxed), Ordering::Relaxed);
//    }).unwrap();

    tray.add_menu_item(&"Quit".to_owned(), move |app| std::process::exit(0)).ok();

    let mut im = RawInputManager::new().unwrap();
    im.register_devices(DeviceType::Keyboards);

    let mut rng = thread_rng();
    let mut caps = false;

    loop {
        if let Some(ev) = im.get_event() {
            if toggle.load(Ordering::Relaxed) {
                match ev {
                    RawEvent::KeyboardEvent(_, _, State::Pressed) => {
                        let weight = match caps {
                            true => 0.2,
                            false => 0.8
                        };

                        caps = rng.gen_bool(weight);
                        key::toggle(key::Code(key::KeyCode::CapsLock), caps, &[], 0);
                    },
                    _ => ()
                }
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
