
#[derive(Debug, failure::Fail)]
pub enum ApplicationError {
    #[fail(display = "Missing request parameter: {}", param)]
    MissingRequestParameter {
        param: String
    }
}
impl lambda::error::LambdaErrorExt for ApplicationError {
    fn error_type(&self) -> &str { "ApplicationError" }
}
