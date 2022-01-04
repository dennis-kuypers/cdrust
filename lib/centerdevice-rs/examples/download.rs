use centerdevice::{client::download::Download, CenterDevice, ClientBuilder, ClientCredentials, Token};

use std::{env, path::Path};

fn main() {
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
    let document_id = env::var_os("CENTERDEVICE_DOCUMENT_ID")
        .expect("Environment variable 'CENTERDEVICE_DOCUMENT_ID' is not set.")
        .to_string_lossy()
        .to_string();

    let client_credentials = ClientCredentials::new(&client_id, &client_secret);
    let token = Token::new(access_token, refresh_token);
    let client = ClientBuilder::new("centerdevice.de", client_credentials).build_with_token(token);

    let download_dir_path = "/tmp";
    let path = Path::new(download_dir_path);
    let download = Download::new(&document_id, path);

    let result = client.download_file(download).expect("Download failed");

    println!("Result: {:#?}", result);
}
