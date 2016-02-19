extern crate hyper;
extern crate serde_json;


use std::io::Read;
use serde_json::Value;


enum PubnubError {
    HttpError,
    IoError,
    FormatError
}


impl std::convert::From<hyper::error::Error> for PubnubError {
    fn from(_: hyper::error::Error) -> Self {
        PubnubError::HttpError
    }
}


impl std::convert::From<std::io::Error> for PubnubError {
    fn from(_: std::io::Error) -> Self {
        PubnubError::IoError
    }
}


impl std::convert::From<serde_json::error::Error> for PubnubError {
    fn from(_: serde_json::error::Error) -> Self {
        PubnubError::FormatError
    }
}


impl std::fmt::Debug for PubnubError {
    fn fmt(&self, f: &mut std::fmt::Formatter)-> std::fmt::Result {
        let x = match *self { 
            PubnubError::HttpError => "HTTP error", 
            PubnubError::IoError => "I/O error", 
            PubnubError::FormatError => "Format error", 
        };
        write!(f, "{}", x)
    }
}


fn pubnub_time() -> Result<u64, PubnubError> {
    let client = hyper::Client::new();
    let mut res = try!(client.get("http://pubsub.pubnub.com/time/0").send());
    let mut body = String::new();
    try!(res.read_to_string(&mut body));
    let json_response:Value = try!(serde_json::from_str(&body[..]));
    let a = try!(json_response.as_array().ok_or(PubnubError::FormatError));
    if a.len() != 1 { return Err(PubnubError::FormatError)};
    a[0].as_u64().ok_or(PubnubError::FormatError)
}


fn pubnub_publish(pubkey: &str, keysub: &str, channel: &str, message: &str) -> Result<bool, PubnubError> {
    let client = hyper::Client::new();
    let url = "http://pubsub.pubnub.com/publish/".to_string() + pubkey + "/" + keysub + "/0/" + channel + "/0/" + message;
    let mut res = try!(client.get(&url).send());
    let mut body = String::new();
    try!(res.read_to_string(&mut body));
    let json_response:Value = try!(serde_json::from_str(&body[..]));
    println!("publish response: {:?}", json_response);
    let a = try!(json_response.as_array().ok_or(PubnubError::FormatError));
    if a.len() != 3 { return Err(PubnubError::FormatError)};
    match a[0].as_u64() {
        Some(v) => Ok(v == 1),
        None => Err(PubnubError::FormatError)
    }
}


fn pubnub_subscribe(keysub: &str, channel: &str, timetoken: &str) -> Result<(Vec<Value>, String), PubnubError> {
    let client = hyper::Client::new();
    let url = "http://pubsub.pubnub.com/subscribe/".to_string() + keysub + "/" + channel + "/0/" + timetoken;
    let mut res = try!(client.get(&url).send());
    let mut body = String::new();
    try!(res.read_to_string(&mut body));
    let json_response:Value = try!(serde_json::from_str(&body[..]));
    println!("subscribe response: {:?}", json_response);
    let mut result: Vec<Value> = Vec::new();
    let a = try!(json_response.as_array().ok_or(PubnubError::FormatError));
    if a.len() < 2 { return Err(PubnubError::FormatError) };
    for x in try!(a[0].as_array().ok_or(PubnubError::FormatError)) {
	result.push(x.clone());
    }
    let token = try!(a[1].as_string().ok_or(PubnubError::FormatError));
    Ok((result, token.to_string()))
}


fn main() {
    println!("pubnub_time: {:?}", pubnub_time());
    
    println!("pubnub_publish: {:?}", pubnub_publish("demo", "demo", "hello_world", "\"Hello world from Rust!\""));
    
    let (msg, token) = match pubnub_subscribe("demo", "hello_world", "0") {
        Ok(x) => x,
        Err(_) => (Vec::<Value>::new(), "lalal".to_string())
    };
    println!("pubnub_subscribe: msg={:?}, token={}", msg, token);

    let (msg, token) = match pubnub_subscribe("demo", "hello_world", &token) {
        Ok(x) => x,
        Err(_) => (Vec::<Value>::new(), "lalal".to_string())
    };
    println!("pubnub_subscribe: msg={:?}, token={}", msg, token);
}
