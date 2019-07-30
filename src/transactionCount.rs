#[macro_use]
use reqwest::Error;
use rand::Rng;
use reqwest::Client;

#[derive(Deserialize, Debug)]
struct Response {
    total: u32,
}

pub fn call_api(current: i32) -> Result<(), Error> {
    println!("Transaction Count - Current Thread {:?}", current);

    let request_url = format!("https://gdfj1abzu8.execute-api.us-east-1.amazonaws.com/dev/{function}",
                              function = "transaction-count");
    println!("Transaction Count - Current Thread {:?} - {}", current, request_url);
    let mut response = reqwest::get(&request_url)?;

    let balances: Response = response.json()?;
    println!("Transaction Count - Current Thread {:?} - {:?}",current, balances);
    Ok(())
}

fn main() {
    let countdown: i32 = 1;
    call_api(countdown);
}