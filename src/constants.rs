// Messages
pub const MESSAGE_OK: &str = "ok";
pub const MESSAGE_CAN_NOT_FETCH_DATA: &str = "Can not fetch data";
pub const MESSAGE_CAN_NOT_INSERT_DATA: &str = "Can not insert data";
pub const MESSAGE_CAN_NOT_UPDATE_DATA: &str = "Can not update data";
pub const MESSAGE_CAN_NOT_DELETE_DATA: &str = "Can not delete data";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";
// pub const MESSAGE_SIGNUP_FAILED: &str = "Error while signing up, please try again";
pub const MESSAGE_LOGIN_SUCCESS: &str = "Login successfully";
pub const MESSAGE_LOGIN_FAILED: &str = "Wrong username or password, please try again";
pub const MESSAGE_USER_NOT_FOUND: &str = "User not found, please signup";
pub const MESSAGE_LOGOUT_SUCCESS: &str = "Logout successfully";
pub const MESSAGE_PROCESS_TOKEN_ERROR: &str = "Error while processing token";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";
pub const MESSAGE_FORBIDDEN: &str = "Forbidden";

// Bad request messages
pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";

// Headers
pub const AUTHORIZATION: &str = "Authorization";

// Misc
pub const EMPTY: &str = "";

// Ignore routes
pub const IGNORE_ROUTES: [(crate::http::Method, &str); 5] = [
    (crate::http::Method::GET, "/api/status"),
    (crate::http::Method::POST, "/api/auth/signup"),
    (crate::http::Method::POST, "/api/auth/login"),
    (crate::http::Method::GET, "/api/event"),
    (crate::http::Method::GET, "/api/activity"),
];