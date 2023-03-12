use reqwest::Url;


type MyError = Box<dyn std::error::Error>;

#[tokio::main]

async fn main() -> Result<(),MyError>{
    
    let url = Url::parse("http://127.0.0.1:3030")?;
    let res = reqwest::get(url).await?;

    let body = res.text().await?;
    println!("{}",&body);
    Ok(())
}