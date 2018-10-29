extern crate autopilot;
extern crate multiinput;
extern crate rand;

use multiinput::*;
use autopilot::*;
use rand::prelude::*;

fn main() {
    let mut im = RawInputManager::new().unwrap();
    im.register_devices(DeviceType::Keyboards);
    println!("SHITPOSTER DOGE HAS TAKEN CONTROL OF YOUR CAPS LOCK");

    let mut rng = thread_rng();
    let mut caps = false;

    loop {
        if let Some(ev) = im.get_event() {
            match ev {
                RawEvent::KeyboardEvent(_, _, State::Pressed) => {
                    let weight = match caps {
                        true => 0.2, false => 0.8
                    };

                    caps = rng.gen_bool(weight);
                    key::toggle(key::Code(key::KeyCode::CapsLock), caps, &[], 0);

                    match caps {
                        true => println!("CAPPED!"),
                        false => println!("and uncapped!")
                    };
                },
                _ => ()
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
