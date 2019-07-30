#[macro_use]
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Response {
    balance: u32,
}

pub fn call_api_balance(current: i32) -> Result<(), Error> {
    //println!("getBalance - Current Thread {:?}", current);

    let mut account_info = "11983836246";

    if current % 2 == 0 {
        account_info = "11946281283";
    }else{
        account_info = "11983836246";
    }

    let request_url = format!("https://gdfj1abzu8.execute-api.us-east-1.amazonaws.com/dev/{function}/{account}",
                              function = "get-balance",
                              account = account_info);
    //println!("getBalance - Current Thread {:?} - {}", current, request_url);
    let mut response = reqwest::get(&request_url)?;

    let balances: Response = response.json()?;
    //println!("getBalance - Current Thread {:?} - {:?}",current, balances);
    Ok(())
}

fn main() {
    let countdown: i32 = 50;
    call_api_balance(countdown);
    //Ok(())
}