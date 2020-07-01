// use async_std::task;
// use surf;

// async fn fetch(url: &str) -> Result <String, surf::Exception> { // either string or exception /error 
//     surf::get(url).recv_string().await
// }

// async fn exce() {
//     match fetch("https://www.w3.org/TR/PNG/iso_8859-1.txt").await {
//         Ok(res) => println!("{}", res),
//         Err(e) => println!("Error {}", e),
//     }
// }

// fn main() {
//     task::block_on(exce())
// }

//==========================

// use surf;
// use futures::try_join;
// use async_std::task;

// fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {         // box is used for error handling
//     task::block_on(async {
//         let req1 = surf::get("https://www.google.com/").recv_string();
//         let req2 = surf::get("https://www.google.com/").recv_string();

//         let (str1, str2) = futures::future::try_join(req1, req2).await?;
//         dbg!("{:?}", str2);     // its a debug for printing present in surf
//         dbg!("{:?}", str1);
//         Ok(())
//     })
// }

//========================

// fn main() {
//     match reqwest::get("https://httpbin.org/ip") {
//         Ok(mut res) => {
//             match res.text() {
//                 Ok(text) => println!("response is: {:?}", text),
//                 Err(_) => println!("The error")
//             }
//         }
//         Err(_) => println!("The error")
//     }
// }

//=======================

// use std::collections::HashMap;
// use async_std::task;

// fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     task::block_on(async {

//         let res = reqwest::blocking::get("https://google.com/")?
//         .json::<HashMap<String, String>>()?;
//         println!("{:?}", res);
//         Ok(())
//     })
// }

//============using tokiyo

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}