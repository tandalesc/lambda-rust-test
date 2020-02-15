#[macro_use] extern crate lambda_runtime as lambda;
#[macro_use] extern crate failure;
#[macro_use] extern crate log;

use lambda_runtime::{Context};
use crate::error::ApplicationError::{self, MissingRequestParameter};
use crate::types::{HandlerResponse, RequestEvent};

mod error;
mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    info!("Starting lambda handler");
    lambda!(lambda_handler);
    Ok(())
}

fn lambda_handler(e: RequestEvent, c: Context) -> Result<HandlerResponse, ApplicationError> {
    trace!("Processing request {}", c.aws_request_id);
    if e.first_name == "" {
        let error = MissingRequestParameter{ param: String::from("firstName") };
        return Err(error);
    }
    let res = HandlerResponse { message: format!("Hi, {}!", e.first_name) };
    Ok(res)
}
