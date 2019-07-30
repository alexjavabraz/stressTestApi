#[macro_use]
use reqwest::Error;
use rand::Rng;
use reqwest::Client;

pub fn do_login(current: i32) -> Result<(), Error> {
    println!("Login - Current Thread {:?}", current);

   
    let request_url = "https://gdfj1abzu8.execute-api.us-east-1.amazonaws.com/dev/login";
    println!("Login - Current Thread {:?} - {}", current, request_url);
    
    let parsed = json!({
        "login": "11946281283",
        "password": "04958763996"
    });

    let gist_body = json!(&parsed);

    let mut response = Client::new().post(request_url)
        .json(&gist_body)
        .send()?;


    println!("Login - Current Thread {:?} - {:?}",current, response);
    Ok(())
}

fn main() {
    let countdown: i32 = 1;
    do_login(countdown);
}