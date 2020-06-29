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

use surf;
use futures::try_join;
use async_std::task;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {         // box is used for error handling
    task::block_on(async {
        let req1 = surf::get("https://www.google.com/").recv_string();
        let req2 = surf::get("https://www.google.com/").recv_string();

        let (str1, str2) = futures::future::try_join(req1, req2).await?;
        dbg!("{:?}", str2);     // its a debug for printing present in surf
        dbg!("{:?}", str1);
        Ok(())
    })
}