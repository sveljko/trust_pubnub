extern crate hyper;
extern crate serde_json;

use hyper::Client;
use std::io::Read;
use serde_json::Value;


fn pubnub_time() -> u64 {
	let client = Client::new();
	let mut res = client.get("http://pubsub.pubnub.com/time/0").send().unwrap();
	let mut body = String::new();
	res.read_to_string(&mut body).unwrap();
	let json_response:Value = serde_json::from_str(&body[..]).unwrap();
	return json_response.as_array().unwrap()[0].as_u64().unwrap();
}


fn pubnub_publish(pubkey: &str, keysub: &str, channel: &str, message: &str) -> bool {
	let client = Client::new();
	let url = "http://pubsub.pubnub.com/publish/".to_string() + pubkey + "/" + keysub + "/0/" + channel + "/0/" + message;
	let mut res = client.get(&url).send().unwrap();
	let mut body = String::new();
	res.read_to_string(&mut body).unwrap();
	let json_response:Value = serde_json::from_str(&body[..]).unwrap();
	println!("publish response: {:?}", json_response);
	return json_response.as_array().unwrap()[0].as_i64().unwrap() == 1;
}


fn pubnub_subscribe(keysub: &str, channel: &str, timetoken: &str) -> (Vec<Value>, String) {
	let client = Client::new();
	let url = "http://pubsub.pubnub.com/subscribe/".to_string() + keysub + "/" + channel + "/0/" + timetoken;
	let mut res = client.get(&url).send().unwrap();
	let mut body = String::new();
	res.read_to_string(&mut body).unwrap();
	let json_response:Value = serde_json::from_str(&body[..]).unwrap();
	println!("subscribe response: {:?}", json_response);
	let mut result: Vec<Value> = Vec::new();
	for x in json_response.as_array().unwrap()[0].as_array().unwrap() {
		result.push(x.clone());
	}
	return (result,
		json_response.as_array().unwrap()[1].as_string().unwrap().to_string());
}


fn main() {
	println!("pubnub_time: {}", pubnub_time());

	println!("pubnub_publish: {}", pubnub_publish("demo", "demo", "hello_world", "\"Hello world from Rust!\""));

	let (msg, token) = pubnub_subscribe("demo", "hello_world", "0");
	println!("publish_subscribe: msg={:?}, token={}", msg, token);
	let (msg, token) = pubnub_subscribe("demo", "hello_world", &token);
	println!("publish_subscribe: msg={:?}, token={}", msg, token);
}
