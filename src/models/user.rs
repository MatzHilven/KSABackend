use crate::{
    config::db::Connection,
    constants,
    models::{login_history::LoginHistory, user_token::UserToken},
    schema::users::{self, dsl::*},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub login_session: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct LoginInfoDTO {
    pub username: String,
    pub login_session: String,
}

impl User {
    pub fn signup(user: UserDTO, connection: &mut Connection) -> Result<String, String> {
        if Self::find_user_by_username(&user.username, connection).is_err() {
            let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
            let user = UserDTO {
                password: hashed_pwd,
                ..user
            };
            diesel::insert_into(users)
                .values(&user)
                .execute(connection).expect("Error while inserting user.");
            Ok(constants::MESSAGE_SIGNUP_SUCCESS.to_string())
        } else {
            Err(format!("User '{}' is already registered", &user.username))
        }
    }

    pub fn login(login: LoginDTO, connection: &mut Connection) -> Option<LoginInfoDTO> {
        if let Ok(user_to_verify) = users
            .filter(username.eq(&login.username_or_email))
            .or_filter(email.eq(&login.username_or_email))
            .get_result::<User>(connection)
        {
            if !user_to_verify.password.is_empty()
                && verify(&login.password, &user_to_verify.password).unwrap()
            {
                if let Some(login_history) = LoginHistory::create(&user_to_verify.username, connection) {
                    if LoginHistory::save_login_history(login_history, connection).is_err() {
                        return None;
                    }
                    let login_session_str = User::generate_login_session();
                    if User::update_login_session_to_db(
                        &user_to_verify.username,
                        &login_session_str,
                        connection,
                    ) {
                        return Some(LoginInfoDTO {
                            username: user_to_verify.username,
                            login_session: login_session_str,
                        });
                    }
                }
            } else {
                return Some(LoginInfoDTO {
                    username: user_to_verify.username,
                    login_session: String::new(),
                });
            }
        }

        None
    }

    pub fn logout(user_id: i32, connection: &mut Connection) {
        if let Ok(user) = users.find(user_id).get_result::<User>(connection) {
            Self::update_login_session_to_db(&user.username, "", connection);
        }
    }

    pub fn is_valid_login_session(user_token: &UserToken, connection: &mut Connection) -> bool {
        users
            .filter(username.eq(&user_token.user))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(connection)
            .is_ok()
    }

    pub fn find_user_by_username(un: &str, connection: &mut Connection) -> QueryResult<User> {
        users.filter(username.eq(un)).get_result::<User>(connection)
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().simple().to_string()
    }

    pub fn update_login_session_to_db(un: &str, login_session_str: &str, connection: &mut Connection, ) -> bool {
        if let Ok(user) = User::find_user_by_username(un, connection) {
            diesel::update(users.find(user.id))
                .set(login_session.eq(login_session_str.to_string()))
                .execute(connection)
                .is_ok()
        } else {
            false
        }
    }
}