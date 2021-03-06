use rocket::http::Status;

#[derive(Debug)]
pub struct CustomResponse<T> {
    message: String,
    status_code: Status,
    data: T,
}

impl<T> CustomResponse<T> {
    pub fn send_error(code: Status, message: String) {}
}
