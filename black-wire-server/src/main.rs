#[macro_use] 
extern crate rocket;
use black_wire_server::definitions::*;
use black_wire_server::utils::*;
use rocket::serde::json::Json;
use dotenvy::dotenv;
use std::env;

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/new", format = "json", data ="<device>")]
async fn new_device(device:Json<Device>) -> &'static str {
    let key = "DATABASE_URL";
    dotenv().ok();
    match env::var(key){
        Ok(val) => {
            let database_connection = Database{
                url: val,
            };
            
            println!("{}", database_connection.url);
            
             
                match database_connection.add_device(&device).await {
                    Ok(_) => {
                        println!("{:#?}", device);
                    },
                    Err(err) => {
                        println!("{err}")
                    }
                };
            
            "Device added!"


           
        },
        Err(err) => {
            println!("{err}");
            "Error"
        }
    }


    


}

#[post("/app")]
fn open_app() -> &'static str {
    "App opened!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index, new_device, open_app])
}
