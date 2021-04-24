#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[ path = "services/configuration_service.rs"] mod configuration_service;


use rocket::http::RawStr;
// use serde::Deserialize;
// use rocket_contrib::json::Json;
use rocket::response::content;





#[get("/test/config/<config_id>")]
fn get_test_config(config_id: &RawStr){

}

#[get("/test/config/template")]
fn get_test_config_template() -> content::Json<&'static str> {  
    content::Json("{ 'config_id' :  '<config_id>', 'test_name' : '<test_name>', 'data_type' : '<data_type>'}")
}


fn main() {
    configuration_service::get_configuration_by_id(8);


    rocket::ignite()
    .mount("/api/v1", routes![get_test_config,get_test_config_template])
    .launch();   
}