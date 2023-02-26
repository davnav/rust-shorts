use reqwest::{Url};
type MyError = Box<dyn std::error::Error>;

//tokio run time is used for running the code asyncronously
#[tokio::main]

// Below is the point where tokio runtime start executing - can be considered as our main program logic
async fn main() -> Result<(), MyError> {


    let client = reqwest::Client::builder();
    let client = client.build()?;

    let url = "https://httpbin.org/redirect-to";
    // let req = client.get(url).form(&params);
    let url = Url::parse_with_params(url,
                                 &[("url", "https://httpbin.org/cookies"), ("status_code", "302")])?;
    println!("{:?}",url);
    let req = client.get(url);

    
    let res = req.send().await?;
    println!("{:?}",res);
    println!("{}",res.url());
    println!("{}",res.status());

    println!("{:?}",res.text().await);




    Ok(())
}