#![crate_name = "nickel"]
#![crate_type = "rlib"]
#![feature(slicing_syntax, plugin, core, collections, io, path, hash)]

//!Nickel is supposed to be a simple and lightweight foundation for web applications written in Rust. Its API is inspired by the popular express framework for JavaScript.
//!
//!Some of the features are:
//!
//!* Easy handlers: A handler is just a function that takes a `Request` and `ResponseWriter`
//!* Variables in routes. Just write `my/route/:someid`
//!* Easy parameter access: `request.params.get(&"someid")`
//!* simple wildcard routes: `/some/*/route`
//!* double wildcard routes: `/a/**/route`
//!* middleware

extern crate time;
extern crate http;
extern crate "rustc-serialize" as serialize;
extern crate regex;
#[plugin] #[no_link] extern crate regex_macros;
extern crate typemap;
extern crate plugin;
extern crate url;
extern crate mustache;
extern crate groupable;
#[macro_use]
extern crate log;

pub use nickel::Nickel;
pub use request::Request;
pub use response::Response;
pub use middleware::{Action, Continue, Halt, Middleware, ErrorHandler, MiddlewareResult};
pub use static_files_handler::StaticFilesHandler;
pub use favicon_handler::FaviconHandler;
pub use default_error_handler::DefaultErrorHandler;
pub use json_body_parser::JsonBody;
pub use query_string::QueryString;
pub use router::{Router, Route, RouteResult, HttpRouter};
pub use nickel_error::{ NickelError, NickelErrorKind, ErrorWithStatusCode, UserDefinedError, Other };
pub use mimes::get_media_type;
pub use middleware_handler::ResponseFinalizer;

pub mod router;
mod server;
mod nickel;
mod request;
mod response;
mod middleware;
mod middleware_handler;
mod favicon_handler;
mod static_files_handler;
mod json_body_parser;
pub mod mimes;
mod query_string;
mod urlencoded;
mod nickel_error;
mod default_error_handler;
