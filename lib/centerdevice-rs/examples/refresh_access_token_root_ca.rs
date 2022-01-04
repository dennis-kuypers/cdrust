use centerdevice::{CenterDevice, Certificate, ClientBuilder, ClientCredentials, Token};

use std::env;

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
    let base_url = env::var_os("CENTERDEVICE_BASE_URL")
        .expect("Environment variable 'CENTERDEVICE_BASE_URL' is not set.")
        .to_string_lossy()
        .to_string();
    let root_ca_file = env::var_os("CENTERDEVICE_ROOT_CA_FILE")
        .expect("Environment variable 'CENTERDEVICE_ROOT_CA_FILE' is not set.")
        .to_string_lossy()
        .to_string();

    let pem = std::fs::read(root_ca_file).expect("Failed to read Root CA pem file");
    let certificate = Certificate::from_pem(&pem).expect("Failed to parse Root CA");

    let client_credentials = ClientCredentials::new(&client_id, &client_secret);
    let token = Token::new(access_token, refresh_token);

    let client = ClientBuilder::new(&base_url, client_credentials)
        .add_root_certificate(certificate)
        .build_with_token(token);

    let token = client.refresh_access_token().expect("Search failed.");

    println!("Refreshed Access Token: '{:#?}'", token);
}
