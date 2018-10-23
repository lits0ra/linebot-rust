#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

extern crate line_messaging_api_rocket;
extern crate rocket;
extern crate serde_json;

extern crate reqwest;
extern crate env_logger;



use rocket::response::Failure;
use rocket::http::Status;

use serde_json::value::Value;

use line_messaging_api_rocket::bot::LineBot;
use line_messaging_api_rocket::messages::{ LineMessage };
use line_messaging_api_rocket::events::{ ReplyableEvent };
use line_messaging_api_rocket::utils;
use line_messaging_api_rocket::rocket_line::models::{ Body, Signature };

#[post("/callback", format="application/json", data = "<body>")]
fn webhook (signature: Signature, body: Body,) -> Result<(), Box<std::error::Error>> { 
    let bot = LineBot::new(
        "XXXXX",
        "XXXXX",
    );

    if bot.check_signature(&body.data, &signature.key){
        if utils::is_replyable(&body.get_data()) {
            let data: &str = &body.get_data();
            let recive_event: Value = serde_json::from_str(data).unwrap();
            let event: ReplyableEvent = utils::to_events(&body.get_data()).unwrap();
            if recive_event["events"][0]["message"]["type"] == "text"{
                let recive_text: &str = &recive_event["events"][0]["message"]["text"].as_str().unwrap();
                let message = LineMessage::create_text("", recive_text);
                event.reply(vec![message], bot);
            }
            else if recive_event["events"][0]["message"]["type"] == "location"{
                let recive_location: &str = &recive_event["events"][0]["message"]["address"].as_str().unwrap().to_string()[21..].to_string();
                let message = LineMessage::create_text("", &(recive_location.to_owned()+"\n付近の電車の発着情報を検索します。"));
                event.reply(vec![message], bot);
                let data: &str = &reqwest::get(&("http://geo.search.olp.yahooapis.jp/OpenLocalPlatform/V1/geoCoder?appid=XXXXXXXX&query=".to_owned()+&recive_location.to_owned()+"&output=json"))?.text()?;
                let recive_data: Value = serde_json::from_str(data).unwrap();
                let coordinates = recive_data["Feature"][0]["Geometry"]["Coordinates"];
                let latitude: &str = serde_json::from_str(coordinates).unwrap();
                let longitude: &str = coordinates[0];
                
            }
        }
    }

    Ok(())
}

fn main() {
    rocket::ignite().mount("/", routes![webhook]).launch();
}