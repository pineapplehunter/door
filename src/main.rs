#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

use std::error::Error;

const GPIO_PWM: u8 = 23;
const GPIO_REED_SWITCH: u8 = 24;

mod hardware;

use crate::hardware::doorlock::DoorLock;
use libreauth::oath::TOTPBuilder;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::{get, post, routes, uri, FromForm, State};
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let keys = load_keys()?;

    println!("{:?}", keys);

    let door_lock = DoorLock::new(GPIO_PWM, GPIO_REED_SWITCH);

    rocket::ignite()
        .manage(DoorState {
            door: Arc::new(Mutex::new(door_lock)),
            keys: Arc::new(keys),
        })
        .mount("/", routes![index, page_post])
        .attach(Template::fairing())
        .launch();

    Ok(())
}

struct DoorState {
    door: Arc<Mutex<DoorLock>>,
    keys: Arc<Vec<String>>,
}

#[derive(FromForm)]
struct Pincode {
    number: String,
}

#[get("/?<msg>")]
fn index(msg: Option<String>) -> Template {
    let mut context = HashMap::new();
    if let Some(e) = msg {
        context.insert("error".to_string(), e);
    }
    Template::render("index", context)
}

#[post("/", data = "<pincode>")]
fn page_post(pincode: Form<Pincode>, state: State<DoorState>) -> Redirect {
    if is_valid(&pincode.number, &*state.keys) {
        let doorlock = state.door.clone();
        thread::spawn(move || {
            if let Ok(mut door) = doorlock.try_lock() {
                door.open().unwrap();
                thread::sleep(Duration::from_secs(5));

                while door.is_open().unwrap() {
                    thread::sleep(Duration::from_secs(1));
                    dbg!("door open!");
                }
                dbg!("door closed!");
                thread::sleep(Duration::from_secs(3));
                door.close().unwrap();
            }
        });
        Redirect::to(uri!(index: msg = "OK"))
    } else {
        Redirect::to(uri!(index: msg = "コードが間違っています"))
    }
}

fn is_valid(number: &str, keys: &[String]) -> bool {
    if number.len() != 6 {
        return false;
    }
    for key in keys {
        let code = TOTPBuilder::new()
            .base32_key(&key)
            .finalize()
            .unwrap()
            .generate();
        if *number == code {
            return true;
        }
    }
    false
}
fn load_keys() -> Result<Vec<String>, std::io::Error> {
    let mut data = String::new();
    File::open("keys.txt")?.read_to_string(&mut data)?;
    Ok(data
        .split('\n')
        .filter(|x| x.len() == 32)
        .map(|x| x.to_owned())
        .collect())
}
