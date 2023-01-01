use const_format::concatcp;

const BASE_URL: &str = "https://api.vndb.org/kana";

pub const POST_ULIST_URL: &str = concatcp!(BASE_URL, "/ulist");
pub const POST_TRAIT_URL: &str = concatcp!(BASE_URL, "/trait");
pub const POST_TAG_URL: &str = concatcp!(BASE_URL, "/tag");
pub const POST_STAFF_URL: &str = concatcp!(BASE_URL, "/staff");
pub const POST_CHARACTER_URL: &str = concatcp!(BASE_URL, "/character");
pub const POST_PRODUCER_URL: &str = concatcp!(BASE_URL, "/producer");
pub const POST_RELEASE_URL: &str = concatcp!(BASE_URL, "/release");
pub const POST_VN_URL: &str = concatcp!(BASE_URL, "/vn");

pub const GET_STATS_URL: &str = concatcp!(BASE_URL, "/stats");
pub const GET_USER_URL: &str = concatcp!(BASE_URL, "/user");
pub const GET_AUTHINFO_URL: &str = concatcp!(BASE_URL, "/authinfo");
pub const GET_ULIST_URL: &str = concatcp!(BASE_URL, "/ulist_labels");
