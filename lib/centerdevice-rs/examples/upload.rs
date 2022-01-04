use centerdevice::{client::upload::Upload, CenterDevice, ClientBuilder, ClientCredentials, Token};

use mime_guess;
use std::{env, path::Path};

fn main() {
    env_logger::init();

    let client_id = env::var_os("CENTERDEVICE_CLIENT_ID")
        .expect("Environment variable 'CENTERDEVICE_CLIENT_ID' is not set.")
        .to_string_lossy()
        .to_string();
    let client_secret = env::var_os("CENTERDEVICE_CLIENT_SECRET")
        .expect("Environment variable 'CENTERDEVICE_CLIENT_SECRET' is not set.")
        .to_string_lossy()
        .to_string();
    let access_token = env::var_os("CENTERDEVICE_ACCESS_TOKEN")
        .expect("Environment variable 'CENTERDEVICE_ACCESS_TOKEN' is not set.")
        .to_string_lossy()
        .to_string();
    let refresh_token = env::var_os("CENTERDEVICE_REFRESH_TOKEN")
        .expect("Environment variable 'CENTERDEVICE_REFRESH_TOKEN' is not set.")
        .to_string_lossy()
        .to_string();

    let client_credentials = ClientCredentials::new(&client_id, &client_secret);
    let token = Token::new(access_token, refresh_token);
    let client = ClientBuilder::new("centerdevice.de", client_credentials).build_with_token(token);

    let file_path = "examples/upload.rs";
    let path = Path::new(file_path);
    let mime_type = mime_guess::get_mime_type(file_path);
    let upload = Upload::new(path, mime_type)
        .expect("Failed to create Upload for path")
        .title("Rust upload example")
        .author("Lukas Pustina")
        .tags(&["rust"])
        .collections(&["5ca2bc87-8314-4037-8d39-5cd31dc379d4"]);

    let result = client.upload_file(upload).expect("Upload failed");

    println!("Result: {:#?}", result);
}
