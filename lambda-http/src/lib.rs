#![warn(missing_docs)]
//#![deny(warnings)]
//! Enriches the `lambda_runtime` crate with [http](https://github.com/hyperium/http)
//! types targeting ALB and API Gateway proxy events.
//!
//! Though ALB and API Gateway proxy events are separate Lambda triggers, they both share
//! similar shapes that contextually map to an http request handler. From a application perspective
//! the differences shouldn't matter. This crate
//! abstracts over both using standard [http](https://github.com/hyperium/http) types allowing
//! you to focus more on your application while giving you to the flexibility to
//! transparently use whichever http trigger suits your application's needs best.
//!
//! # Examples
//!
//! ```rust,no_run
//! use lambda_http::{lambda, IntoResponse, Request, RequestExt};
//! use lambda_runtime::{Context, error::HandlerError};
//!
//! fn main() {
//!     lambda!(hello)
//! }
//!
//! fn hello(
//!     request: Request,
//!     _ctx: Context
//! ) -> Result<impl IntoResponse, HandlerError> {
//!     Ok(format!(
//!         "hello {}",
//!         request
//!             .query_string_parameters()
//!             .get("name")
//!             .unwrap_or_else(|| "stranger")
//!     ))
//! }
//! ```
//!
//! You can also provide a closure directly to the `lambda!` macro
//!
//! ```rust,no_run
//! use lambda_http::{lambda, Request, RequestExt};
//!
//! fn main() {
//!   lambda!(
//!     |request: Request, context| Ok(
//!       format!(
//!         "hello {}",
//!         request.query_string_parameters()
//!           .get("name")
//!           .unwrap_or_else(|| "stranger")
//!       )
//!     )
//!   )
//! }
//! ```

// only externed because maplit doesn't seem to play well with 2018 edition imports
#[cfg(test)]
#[macro_use]
extern crate maplit;

use std::{fmt, future::Future};

pub use http::{self, Response};
use lambda::{self as lambda, LambdaCtx as Context, Handler as UpstreamHandler};
use tokio::runtime::Runtime as TokioRuntime;

use futures::future::FutureExt;

mod body;
mod ext;
pub mod request;
mod response;
mod strmap;

pub use crate::{body::Body, ext::RequestExt, response::IntoResponse, strmap::StrMap};
pub use crate::{request::LambdaRequest, response::LambdaResponse};

// type HandlerError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Type alias for `http::Request`s with a fixed `lambda_http::Body` body
pub type Request = http::Request<Body>;

// /// Functions serving as ALB and API Gateway handlers must conform to this type.
// pub trait Handler<R> {
//     /// Errors returned by this handler.
//     type Err;
//     /// The future response value of this handler.
//     type Fut: Future<Output = Result<R, Self::Err>>;
//     /// Run the handler.
//     fn run(&mut self, event: Request) -> Self::Fut;
// }

// impl<F, R, Err, Fut> Handler<R> for F
// where
//     F: FnMut(Request) -> Fut,
//     Fut: Future<Output = Result<R, Err>> + Send,
//     Err: Into<Box<dyn std::error::Error + Send + Sync + 'static>> + fmt::Debug,
// {
//     type Err = Err;
//     type Fut = Fut;
//     fn run(&mut self, event: Request) -> Self::Fut {
//         (*self)(event)
//     }
// }

// impl<R> UpstreamHandler<Request, R> for Handler
// {
//     type Err = Self::Err;
//     type Fut = Self::Fut;
//     async fn call(&mut self, event: A) -> Result<R, Self::Err> {
//         todo!()
//     }
// }

// /// Creates a new `lambda_runtime::Runtime` and begins polling for ALB and API Gateway events
// ///
// /// # Arguments
// ///
// /// * `f` A type that conforms to the `Handler` interface.
// ///
// /// # Panics
// /// The function panics if the Lambda environment variables are not set.
// pub fn start<R>(f: impl Handler<R>, runtime: Option<TokioRuntime>)
// where
//     R: IntoResponse,
// {
//     // handler requires a mutable ref
//     let mut func = f;
//     runtime.unwrap().block_on(
//         lambda::run(
//             lambda::handler_fn(
//                 |req: LambdaRequest<'_>| {
//                     let is_alb = req.request_context.is_alb();
//                     func.run(req.into())
//                         .map(|result| result.map(|resp| LambdaResponse::from_response(is_alb, resp.into_response())))
//                 },
//             )
//         )
//     );
// }

// /// A macro for starting new handler's poll for API Gateway and ALB events
// #[macro_export]
// macro_rules! lambda {
//     ($handler:expr) => {
//         $crate::start($handler, None)
//     };
//     ($handler:expr, $runtime:expr) => {
//         $crate::start($handler, Some($runtime))
//     };
//     ($handler:ident) => {
//         $crate::start($handler, None)
//     };
//     ($handler:ident, $runtime:expr) => {
//         $crate::start($handler, Some($runtime))
//     };
// }
