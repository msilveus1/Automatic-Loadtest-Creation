#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[ path = "services/configuration_service.rs" ] mod configuration_service;

fn main() {
    rocket::ignite()    
        .launch();
}