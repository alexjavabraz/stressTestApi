#[macro_use]
use reqwest::Error;
use rand::Rng;
use reqwest::Client;

#[derive(Deserialize)]
struct ApiResponse {
    Message: String,
    ErrorCode: u32,
    LiqdcId: f64,
}

pub fn call_api_debit_operation(i: i32) -> Result<(), Error> {
    let request_url = " http://bp-api-hml.brasilplural.com/api/BPBank/SendTransferPost";
    let mut value = format!("4,{}", i);

    let mut name = "Joao Darc S. Santos";
    let mut AgencyDestination = format!("{:04}", i);
    let mut AccountDestination = format!("{:07}", 42);

    if((i % 2) == 0){
        name = "Joao Braz";
    }

    let parsed = json!(
                            {
                                "User": "APITEST",
                                "Password": "APITEST",
                                "BankDestination": "237",
                                "AgencyDestination": AgencyDestination,
                                "AccountDestination": AccountDestination,
                                "AccountDigit": "1",
                                "CPFCNPJ": "29488051043",
                                "PersonType": "F",
                                "Name": name,
                                "Observation": "Test ".to_owned() + &i.to_string(),
                                "AccountType": "1",
                                "Value": value
                            }
    );

    let gist_body    = json!(&parsed);
    let mut response = Client::new().post(request_url)
        .json(&gist_body)
        .send()?.json::<ApiResponse>()?;

    if response.Message != "" {
        println!("Message Response {:?} - ErrorCode {:?} - LiqdcId - {:?}", response.Message, response.ErrorCode, response.LiqdcId);
        println!("Message Request {:?} - {:?} ", i, gist_body);
    }

    //println!("Create Debit Transfer - Current Thread {:?} ", i);
    Ok(())
}