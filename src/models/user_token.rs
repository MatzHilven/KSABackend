use crate::models::user::LoginInfoDTO;
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub user: String,
    pub login_session: String,
}

impl UserToken {
    pub fn generate_token(login: &LoginInfoDTO) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            user: login.username.clone(),
            login_session: login.login_session.clone(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(&KEY),
        ).unwrap()
    }
}