pub use xmlrpc::client::Client;
pub use xmlrpc::server::{run_xmlrpc_server, HandlesXmlrpcRequests};
pub use xmlrpc::common::{Request, Response, Value};

mod client;
mod server;
mod parser;
mod common;
mod macros;


