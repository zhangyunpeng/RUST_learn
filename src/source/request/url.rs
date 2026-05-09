//
// use std::io::{self, Result, ErrorKind};
// use std::error::Error;
// use url;
//
// pub enum Protocol {
//     HTTP,
//     HTTPS,
// }
//
// pub struct Url {
//     pub protocol: Protocol,
//     pub host: String,
//     pub port: u16,
//     pub path: String,
// }
//
// impl Url {
//     pub fn new(url: &str) -> Result<Url> {
//         let parsed_url = match url::Url::parse(url) {
//             Ok(url) => url,
//             Err(e) => {
//                 let err = io::Error::new(ErrorKind::InvalidInput, e.to_string());
//                 return Err(err);
//             }
//         };
//
//         let protocol = match parsed_url.scheme() {
//             "http" => Protocol::HTTP,
//             "https" => Protocol::HTTPS,
//             _ => {
//                 let err = io::Error::new(ErrorKind::InvalidInput,
//                     "The protocol is not supported."
//                 );
//                 return Err(err);
//             }
//         };
//         let host = match parsed_url.domain() {
//             Some(host) => host,
//             None => {
//                 let err = io::Error::new(ErrorKind::InvalidInput,
//                                          "The URL is invalid."
//                 );
//                 return Err(err);
//             }
//         };
//         let port = match parsed_url.port() {
//             Some(port) => port,
//             None => match protocol {
//                 Protocol::HTTP => 80,
//                 Protocol::HTTPS => 443,
//             }
//         };
//
//     }
// }