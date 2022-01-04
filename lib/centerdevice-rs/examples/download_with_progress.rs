use centerdevice::{client::download::Download, CenterDevice, ClientBuilder, ClientCredentials, Token, WithProgress};

use std::{
    env,
    io::{stdout, Write},
    path::Path,
};

pub struct Progress {
    amount: usize,
    interval: usize,
    interval_counter: usize,
}

impl Progress {
    fn new() -> Self {
        Progress {
            amount: 0,
            interval: 0,
            interval_counter: 0,
        }
    }
}

impl WithProgress for Progress {
    fn setup(&mut self, size: usize) {
        self.interval = size / 10;
    }

    fn progress(&mut self, amount: usize) {
        self.amount += amount;
        if self.amount > self.interval_counter + self.interval {
            self.interval_counter = self.amount;
            print!(".");
            stdout().flush().expect("Failed to write to stdout");
        }
    }

    fn finish(&self) {
        println!(".");
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
    let download = Download::new(&document_id, path).filename(Path::new("centerdevice_download"));

    let mut progress = Progress::new();
    let result = client
        //.download_file(download)
        .download_file_with_progress(download, &mut progress)
        .expect("Download failed");

    println!("Result: {:#?}", result);
}
