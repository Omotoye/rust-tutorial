use super::method::Method;

pub struct Request {
    path: String,
    query_string: Option<String>, // Option is used to represent option of something or nothing
    method: Method,
}
