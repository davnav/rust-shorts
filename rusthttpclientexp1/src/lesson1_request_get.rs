// First program to demonstrate http request in Rust
// Experiments on how to build a http request in Rust programming language using reqwest

// This is the custom type of error used for error propagation
// This case we have only standard error 
type MyError = Box<dyn std::error::Error>;

//tokio run time is used for running the code asyncronously
#[tokio::main]

// Below is the point where tokio runtime start executing - can be considered as our main program logic
async fn main() -> Result<(), MyError> {

    // Spawning a tokio thread to execure the async get request using reqwest
    let handle = tokio::spawn(async {
        let body = reqwest::get("https://httpbin.org/get")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("{}", body);
    });

    // get request to httpbin to get headers
    // As this is an asynchronours call, we need to wait for the response to get it finished using 'await' 
    let res = reqwest::get("https://httpbin.org/headers").await?;

    // printing the headers
    println!("{:?}", res.headers());

    handle.await;
    Ok(())
}
