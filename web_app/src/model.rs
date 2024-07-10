use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use crate::schema::users;
use crate::Insertable;


#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub phone: String,
}


#[derive(Serialize)]
struct TwilioMessage {
    to: String,
    from: String,
    body: String,
}

#[derive(Deserialize)]
pub struct TwilioResponse {
    sid: String,
    status: String,
}

pub async fn send_sms(to: &str, code: &str) -> Result<TwilioResponse, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let account_sid = env::var("TWILIO_ACCOUNT_SID")?;
    let auth_token = env::var("TWILIO_AUTH_TOKEN")?;
    let from = env::var("TWILIO_PHONE_NUMBER")?;

    let client = Client::new();
    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account_sid);

    let message = TwilioMessage {
        to: to.to_string(),
        from,
        body: format!("Your verification code is: {}", code),
    };

    let response = client
        .post(&url)
        .basic_auth(account_sid, Some(auth_token))
        .form(&message)
        .send()
        .await?
        .json::<TwilioResponse>()
        .await?;

    Ok(response)
}

pub async fn verify_code(stored_code: &str, provided_code: &str) -> bool {
    stored_code == provided_code
}

