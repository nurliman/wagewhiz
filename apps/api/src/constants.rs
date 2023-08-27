pub const ACCESS_TOKEN_COOKIE_NAME: &str = "access_token";
pub const REFRESH_TOKEN_COOKIE_NAME: &str = "refresh_token";
pub const ACCESS_TOKEN_MAX_AGE: time::Duration = time::Duration::minutes(15);
pub const REFRESH_TOKEN_MAX_AGE: time::Duration = time::Duration::hours(2);
