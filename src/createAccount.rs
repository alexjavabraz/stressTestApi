#[macro_use]
use reqwest::Error;
use rand::Rng;
use reqwest::Client;


#[derive(Deserialize, Debug)]
struct User {
    phoneNumber: String,
    cpf: String,
    name: String,
    deviceId: String
}

pub fn call_api(current: i32) -> Result<(), Error> {
    println!("Create Account - Current Thread {:?}", current);

    //let mut transfer = Transfer::new(current.to_string());

    let adeviceid: String = generate_telephone();
    let acpf: String = generate_telephone();
    let aphone: String = generate_telephone();
    let aname: String = current.to_string();

    let user = User{phoneNumber: aphone, cpf: acpf, name: aname, deviceId: adeviceid};
    
    let request_url = "https://gdfj1abzu8.execute-api.us-east-1.amazonaws.com/dev/create-account";
    //println!("Create Account - Current Thread {:?} - {}", current, request_url);
    
    let parsed = json!(
    {
        "name": user.name,
        "cpf": user.cpf,
        "phoneNumber": user.phoneNumber,
        "deviceId": user.deviceId
    });

    let gist_body = json!(&parsed);

    //println!("Create Account - Current Thread {:?} - {}", current, gist_body);


    let mut response = Client::new().post(request_url)
        .json(&gist_body)
        .send()?;


    //println!("Create Account - Current Thread {:?} - {:?}",current, response);
    Ok(())
}

fn main() {
    let countdown: i32 = 1;
    call_api(countdown);
    //Ok(())
}

fn generate_telephone() -> String {

    const CHARSET: &[u8] = b"0123456789";
    const PASSWORD_LEN: usize = 10;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            // This is safe because `idx` is in range of `CHARSET`
            char::from(unsafe { *CHARSET.get_unchecked(idx) })
        })
        .collect();

    println!("{:?}", password);

    password
}