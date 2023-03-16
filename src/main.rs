extern crate pem;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
// use mongodb::Collection;
// use mongodb::error::Error;
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand;

#[path = "db/util.rs"] mod db;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[macro_use]
extern crate rocket;
extern crate bson;
extern crate mongodb;


#[get("/")]
async fn index() -> &'static str {
    db::main().await;
    // let database = client.database("mongodbVSCodePlaygroundDB");
    // println!("{:?}", database);
    return "pub_key";
}

static string: &str = "Hello World";

#[post("/", data = "<user_input>")]
fn helloPost(user_input: std::string::String) -> String {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key"); 
    let pub_key = RsaPublicKey::from(&priv_key);
   


    // Encrypt
let data = b"hello world";
let enc_data = pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &data[..]).expect("failed to encrypt");
assert_ne!(&data[..], &enc_data[..]);

println!("Encrypted data: {:?}", enc_data);
    return user_input;
}


#[get("/test/<check>/<test>")]
fn hello(check: u32, test: &str) -> String {
    format!("Hello, {} year old named {}!", check, test)
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS).mount("/", routes![index]).mount("/", routes![hello]).mount("/", routes![helloPost])
}



