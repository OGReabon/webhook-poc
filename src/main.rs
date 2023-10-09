#![feature(proc_macro_hygiene, decl_macro)]

mod models;
mod schema;
mod services;

#[macro_use]
extern crate rocket;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use crate::services::user_service::get_user;
use rocket::serde::json::Json;
use serde::Deserialize;
use std::collections::HashSet;
use std::sync::Mutex;

struct Subscribers {
    clients: Mutex<HashSet<String>>,
}

#[derive(Deserialize)]
struct Subscription {
    callback_url: String,
}

#[get("/")]
fn index() -> &'static str {
    "This is a health check"
}

#[post("/subscribe", format = "json", data = "<subscription>")]
fn subscribe(
    subscription: Json<Subscription>,
    subscribers: &rocket::State<Subscribers>,
) -> &'static str {
    let mut clients = subscribers.clients.lock().unwrap();
    clients.insert(subscription.callback_url.clone());
    "Subscribed successfully"
}

#[post("/notify", format = "json", data = "<message>")]
fn notify(
    message: Json<serde_json::Value>,
    subscribers: &rocket::State<Subscribers>,
) -> &'static str {
    let clients: std::sync::MutexGuard<'_, HashSet<String>> = subscribers.clients.lock().unwrap();
    for client in clients.iter() {
        let _ = reqwest::blocking::Client::new()
            .post(client)
            .json(&*message)
            .send();
    }
    "Notification sent!"
}

#[get("/user/<id>")]
fn fetch_user(id: i32) -> Result<String, String> {
    match get_user(id) {
        Ok(user) => Ok(user.name),
        Err(err) => Err(format!("Failed to fetch user: {}", err)),
    }
}

// #[post("/user", format = "json", data = "<user>")]
// fn create_user(user: i32) -> String {
//     format!("Hello, {}!", user.name);
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Subscribers {
            clients: Mutex::new(HashSet::new()),
        })
        .mount("/", routes![index, subscribe, notify, fetch_user])
}
