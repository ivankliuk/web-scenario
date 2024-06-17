use reqwest::blocking::{Client, RequestBuilder};
use std::fs;

mod errors;
use errors::AppError;
mod models;
use models::{HttpMethod, Step};

fn read_scenario_file(path: String) -> Result<Vec<Step>, AppError> {
    let file_contents = fs::read_to_string(path)
        .map_err(AppError::from)
        .and_then(|file_content| {
            serde_json::from_str::<Vec<Step>>(&file_content).map_err(AppError::from)
        });

    file_contents
}

// fn send_request(step: Step) -> () {
//     let body = reqwest::blocking::get("https://www.rust-lang.org")
//         .unwrap()
//         .text();

//     println!("body = {body:?}");
// }

// fn run() -> Result<(), Error> {
//     let client = reqwest::blocking::Client::new();
//     let res = client.post("http://httpbin.org/post")
//         .body("the exact body that is sent")
//         .send()?;
//     Ok(())
// }

fn build_request(client: &Client, step: Step) -> RequestBuilder {
    match step {
        Step {
            method: HttpMethod::GET,
            url,
            ..
        } => client.get(url),
        Step {
            method: HttpMethod::POST,
            url,
            body: Some(body),
        } => client.post(url).body(body),
        _ => panic!("Unsupported HTTP method!"),
    }
}

fn main() {
    let steps = match read_scenario_file(String::from("scenario.json")) {
        Ok(steps) => steps,
        Err(e) => panic!("Error occurred during reading of the file {:?}", e),
    };
    let client = reqwest::blocking::Client::new();
    let requests = steps.into_iter().map(|step| build_request(&client, step));
    for request in requests.into_iter() {
        match request.send() {
            Ok(response) => {
                let response1 = &response.text().unwrap()[..50];
                println!("{:?}", response1)
            }
            Err(_) => {
                println!("Something wrong with the request");
                // return  <- uncomment it for short circuiting.
            }
        }
    }
    // let http_result = steps.map_err(AppError::from);
}
