#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Subscribers {
            clients: Mutex::new(HashSet::new()),
        })
        .mount("/", routes![index, subscribe, notify])
}
