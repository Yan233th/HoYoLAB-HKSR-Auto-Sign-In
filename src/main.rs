use reqwest::blocking::Client;

mod tools;

fn main() {
    let tokens = tools::get_tokens();
    let client = Client::new();
    let headers = tools::generate_headers();
    for token in tokens {
        tools::do_sign(&client, headers.clone(), token);
    }
}
