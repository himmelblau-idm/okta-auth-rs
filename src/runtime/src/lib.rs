//! Native services for source-generated authentication logic.
pub mod value;
pub use value::*;

mod native;

mod http;
pub use http::HttpTransport;
