#[macro_use]
use reqwest::Error;
use rand::Rng;
use reqwest::Client;

pub fn callApi(current: i32) -> Result<(), Error> {
    //println!("Create Transfer - Current Thread {:?}", current);

    let mut from = "11975132627";

    let mut account_info = "11946281283";

    if current % 2 == 0 {
        account_info = "11983836246";
    }else{
        account_info = "11946281283";
    }
    
    let request_url = "https://gdfj1abzu8.execute-api.us-east-1.amazonaws.com/dev/create-transaction";
    //println!("Create Transfer - Current Thread {:?} - {}", current, request_url);
    
    let parsed = json!(
{
    "from": from,
    "to": account_info,
    "amount": 1,
    "summary": "Stress Test",
    "deviceId": "B23745234ASD134134",
    "method": "qrcode",
    "orderId": current.to_string(),
    "promo": true,
    "product": [
        {
            "summary": current.to_string(),
            "quantity": 1,
            "amount": 1,
            "ean": "7890011001200"
        }
    ]
}


    );

    let gist_body = json!(&parsed);

    let mut response = Client::new().post(request_url)
        .json(&gist_body)
        .send()?;


    println!("Create Transfer - Current Thread {:?} - {:?}",current, response.status());
    Ok(())
}

fn main() {
    let countdown: i32 = 1;
    callApi(countdown);
}