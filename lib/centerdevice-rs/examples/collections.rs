use centerdevice::{client::collections::CollectionsQuery, CenterDevice, ClientBuilder, ClientCredentials, Token};

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

    let client_credentials = ClientCredentials::new(&client_id, &client_secret);
    let token = Token::new(access_token, refresh_token);

    let client = ClientBuilder::new("centerdevice.de", client_credentials).build_with_token(token);
    let query = CollectionsQuery::new().include_public();

    let collections = client.search_collections(query).expect("Search failed.");

    println!("Collections: {:#?}", collections);
}
