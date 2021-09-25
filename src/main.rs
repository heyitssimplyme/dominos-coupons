use std::{env, time::Duration};
use futures::{stream, StreamExt};
use serde::Deserialize;

const CONCURRENT_REQUESTS: usize = 50;

#[derive(Debug, Deserialize)]
#[serde(rename_all="PascalCase")]
struct DominosResponse {
  status: i32,
  code: Option<String>,
  name: Option<String>,
  price: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(2))
    .build()?;

  let range = 1000..10000;

  let store_id = env::var("STORE_ID").expect("no store id environment variable");

  let responses = stream::iter(range.into_iter())
    .map(|i| {
      let client = &client;
      let url = format!("https://order.dominos.ca/power/store/{}/coupon/{}?lang=en", store_id, i);
      async move {
        let resp = client.get(url)
          .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/11.1.2 Safari/605.1.15")
          .header("Referer", "https://order.dominos.ca/assets/build/xdomain/proxy.html")
          .send()
          .await?;
          // let resp = client.get(url).send().await?;
        resp.json::<DominosResponse>().await
      }
    })
    .buffer_unordered(CONCURRENT_REQUESTS);

  responses
    .for_each(|b| async {
      match b {
        Ok(d) => {
          if d.status != -404 {
            let mut price = d.price.unwrap();
            if price.len() == 0 { price = "N/A".to_string(); }
            println!("Code: {} | Price: ${} | Name: {}", d.code.unwrap(), price, d.name.unwrap());
          }
        },
        _ => {},
      }
    })
    .await;

  Ok(())
}
