use http::StatusCode;
use serde_json::json;
use std::error::Error;
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
	let info = json!({
		"name": "Joshua Byrd",
		"age": 39,
		"phones": [
			"0438 519545",
			"NONE"
		]
	});

	println!("first phone number: {}", info["phones"][0]);

	// Convert to a string of JSON and print it out
	println!("{}", info.to_string());

	let response = Response::builder()
		.status(StatusCode::OK)
		.header("Content-Type", "application/json")
		.body(info.to_string())
		.expect("Internal Server Error");

	Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
	Ok(lambda!(handler))
}
