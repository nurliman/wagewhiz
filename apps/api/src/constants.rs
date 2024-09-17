pub const ACCESS_TOKEN_COOKIE_NAME: &str = "access_token";
pub const REFRESH_TOKEN_COOKIE_NAME: &str = "refresh_token";
pub const ACCESS_TOKEN_MAX_AGE: time::Duration = time::Duration::hours(1);
pub const REFRESH_TOKEN_MAX_AGE: time::Duration = time::Duration::weeks(4);

pub const DEFAULT_PAGE: u64 = 1;
pub const DEFAULT_PAGE_SIZE: u64 = 12;
pub const DEFAULT_MAX_PAGE_SIZE: u64 = 50;
