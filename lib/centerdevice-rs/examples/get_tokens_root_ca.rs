use centerdevice::{
    client::auth::{Code, CodeProvider},
    errors::Result,
    Certificate, ClientBuilder, ClientCredentials,
};

use reqwest::IntoUrl;
use std::{
    env,
    io::{self, Write},
};

struct MyCodeProvider {}

impl CodeProvider for MyCodeProvider {
    fn get_code<T: IntoUrl>(&self, auth_url: T) -> Result<Code> {
        let auth_url = auth_url.into_url().expect("Failed to parse auth url");

        println!(
            "Please authenticate at the following URL, wait for the redirect, enter the code into the terminal, and \
             then press return ..."
        );
        println!("\n\t{}\n", auth_url);
        print!("Authentication code: ");
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let code = input.trim();

        let code = Code::new(code.to_string());

        Ok(code)
    }
}

fn main() {
    let client_id = env::var_os("CENTERDEVICE_CLIENT_ID")
        .expect("Environment variable 'CENTERDEVICE_CLIENT_ID' is not set.")
        .to_string_lossy()
        .to_string();
    let client_secret = env::var_os("CENTERDEVICE_CLIENT_SECRET")
        .expect("Environment variable 'CENTERDEVICE_CLIENT_SECRET' is not set.")
        .to_string_lossy()
        .to_string();
    let redirect_uri = env::var_os("CENTERDEVICE_REDIRECT_URI")
        .expect("Environment variable 'CENTERDEVICE_REDIRECT_URI' is not set.")
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
    let code_provider = MyCodeProvider {};

    let client = ClientBuilder::new(&base_url, client_credentials)
        .add_root_certificate(certificate)
        .build()
        .authorize_with_code_flow(&redirect_uri, &code_provider)
        .expect("API call failed.");

    let result = client.token();

    println!("Result: '{:#?}'", result);
}
