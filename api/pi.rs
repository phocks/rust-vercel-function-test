use http::StatusCode;
use num_bigint::BigInt;
use serde_json::json;
use std::error::Error;
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
	let pi = calc_pi(10000);
	let info = json!({ "pi": pi });

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

fn calc_pi(iterations: u32) -> std::string::String {
	let mut q = BigInt::from(1);
	let mut r = BigInt::from(0);
	let mut t = BigInt::from(1);
	let mut k = BigInt::from(1);
	let mut n = BigInt::from(3);
	let mut l = BigInt::from(3);
	let mut first = true;

	let mut count = 1;
	let mut pi = String::from("");

	while count < iterations {
		if &q * 4 + &r - &t < &n * &t {
			let str = n.to_string();

			pi = format!("{}{}", pi, str);

			if first {
				pi = format!("{}{}", pi, ".");
				first = false;
			}
			let nr = (&r - &n * &t) * 10;
			n = (&q * 3 + &r) * 10 / &t - &n * 10;
			q *= 10;
			r = nr;
		} else {
			let nr = (&q * 2 + &r) * &l;
			let nn = (&q * &k * 7 + 2 + &r * &l) / (&t * &l);
			q *= &k;
			t *= &l;
			l += 2;
			k += 1;
			n = nn;
			r = nr;
		}

		count += 1;
	}

	return pi;
}
