use blockfrost::{load, BlockFrostApi, BlockFrostSettings, Address};
use rocket::serde::{Deserialize, Serialize, json::{Json, json, Value}};
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[get("/<path..>")]
async fn index(path: PathBuf) -> Option<NamedFile> {
    println!(env!("CARGO_MANIFEST_DIR"));
    let mut path = 
    Path::new(&format!("{}/frontend/build", env!("CARGO_MANIFEST_DIR"))).join(path);
  
    if path.is_dir() {
        path.push("index.html");
    }

    NamedFile::open(path).await.ok()
}

#[macro_use] extern crate rocket;
#[get("/address/<address>")]
async fn address_info(address: String) -> Value {
    let address_info = get_address_info(address).await;

    if let Err(_err) = address_info {
        println!("{:?}", _err);
        json!("Error")
    } else {
        json!(address_info.unwrap())   
    }
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Metadata {
    name: String,
    description: String,
    amount: u8
}

#[post("/example", format="json", data = "<metadata>")]
async fn example(metadata: Json<Metadata>) -> Value {
    json!({
        "name": metadata.name,
        "uppcase_name": metadata.name.to_uppercase()
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, address_info, example])
}

fn build_api() -> blockfrost::Result<BlockFrostApi> {
    let configurations = load::configurations_from_env()?;

    let project_id = configurations["project_id"].as_str().unwrap();
    let cardano_network:String = String::from(configurations["cardano_network"].as_str().unwrap());

    let settings = BlockFrostSettings {
        network_address: cardano_network,
        query_parameters: Default::default(),
        retry_settings: Default::default()
    };

    let api = BlockFrostApi::new(project_id, settings);
    Ok(api)
}

async fn get_address_info(address: String) -> blockfrost::Result<Address> {
    let api = build_api()?;
    let address_info = api.addresses(&address).await;

    Ok(address_info.unwrap())
}