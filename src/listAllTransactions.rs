#[macro_use]
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Response {
    balance: u32,
}

pub fn call_api_list_transactions(current: i32) -> Result<(), Error> {
    //println!("List Transactions - Current Thread {:?}", current);

    let mut account_info = "11975132627";

    if current % 2 == 0 {
        account_info = "11946281283";
    }else{
        account_info = "11975132627";
    }

    let request_url = format!("https://gdfj1abzu8.execute-api.us-east-1.amazonaws.com/dev/{function}/{account}",
                              function = "get-transactions",
                              account = account_info);
    //println!("List Transactions - Current Thread {:?} - {}", current, request_url);
    let mut response = reqwest::get(&request_url)?;

    //println!("List Transactions - Current Thread {:?} - {:?}",current, response);
    Ok(())
}

fn main() {
    let countdown: i32 = 1;
    call_api_list_transactions(countdown);
    //Ok(())
}