use fastly::http::StatusCode;
use fastly::{Error, Request, Response};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // let mut req = Request::from_client();
    //
    // request lands on compute service 1
    // add header chain-status and send to compute service 2
    // compute service 2 routes request back to compute service 1
    // if chain-status header exists with a value of 1 then do complicated routing logic
    //

    // baby steps
    // if chain-status header doesnt exist create it and send it httpbin to be reflected back in response

    const HBIN: &str = "OHTTP";

	// let c_status = req.get_header_str("chain-status")
	// 							.unwrap_or("")
	// 							.to_string();
    
    for name in req.get_header_names() {
        println!("saw header: {:?}", name);
    }

    // req.set_header("host", "httpbin.org");
    
    // if c_status == "" {
        // let request = Request::get(
        // let bin_resp = Request::get("https://httpbin.org/headers")
        // .with_pass(true)
        // .send(HBIN)?;

        return Ok(req
            .with_pass(true)
            .send(HBIN)?);

        // Ok(bin_resp)

}
