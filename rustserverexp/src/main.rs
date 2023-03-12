
use warp::Filter;

type MyError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(),MyError> {
    let routes = warp::any().map( || "helloworld ");
    warp::serve(routes).run(([127,0,0,1],3030)).await;

    Ok(())
}
