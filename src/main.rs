#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod views;

fn main() {
    rocket::ignite().mount("/", routes![views::index, views::post]).launch();
}
