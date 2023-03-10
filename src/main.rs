use error_chain::error_chain;
use mime::Mime;
use reqwest::header::CONTENT_TYPE;
use std::str::FromStr;

error_chain! {
   foreign_links {
       Reqwest(reqwest::Error);
       Header(reqwest::header::ToStrError);
       Mime(mime::FromStrError);
   }
}

#[tokio::main]
async fn main() -> Result<()> {
    // make a request to the url
    let url = "https://images.unsplash.com/photo-1563805042-7684c019e1cb?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2303&q=80.png";
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    // get the content type
    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .ok_or("No content type")?
        .to_str()?;
    let mime_type: Mime = Mime::from_str(content_type)?;

    println!("Mime: {:?}", mime_type);

    // Another method
    // let header = response.headers();

    // match header.get(CONTENT_TYPE) {
    //     Some(content_type) => {
    //         let mime_type: Mime = Mime::from_str(content_type.to_str()?)?;
    //         println!("Mime: {:?}", mime_type);
    //     }
    //     None => println!("No content type"),
    // }

    Ok(())
}
