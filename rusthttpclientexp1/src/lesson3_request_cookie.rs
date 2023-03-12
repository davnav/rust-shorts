//working with cookies using reqwest

use reqwest::{Url, header,ClientBuilder, Client};

type MyError = Box< dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(),MyError> {

    let payload = [("freeform","naveen")];

    // let url = Url::parse_with_params("https://httpbin.org/cookies/set",&payload)?;

    // println!("{:?}",url);

    let mut request_headers = header::HeaderMap::new();
    request_headers.insert(
        header::COOKIE,
        header::HeaderValue::from_static("freeform=naveen"),
    );

    let client = ClientBuilder::new()
                .default_headers(request_headers)
                .cookie_store(true)
                .build().unwrap();

    let res = client.get("https://httpbin.org/cookies/set").send().await?;
    let url = res.url().clone();
    // let res = reqwest::get(url).await?;
    // println!("{}",res.status());
    // println!("{:?}",res.headers());
    // let cookies = res.cookies();
    let body = res.text().await?;
    // for i in cookies {
    //     println!("{:?}",i);
    // }
    println!("{:?}",body);
    // let res = reqwest::get(url).await?;
    let res = client.get("https://httpbin.org/cookies").send().await?;

    let body = res.text().await?;
    println!("{:?}",body);


    Ok(())
}