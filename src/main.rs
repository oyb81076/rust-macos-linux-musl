use futures::join;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("start");
    let text1 = fetch_message("text1");
    let text2 = fetch_message("text2");
    let (t0, t1) = join!(text1, text2);
    println!("{}, {}", t0, t1);
}

async fn fetch_message(s: &str) -> String {
    println!("fetch message start {}", s);
    sleep(Duration::from_millis(1000)).await;
    println!("fetch message complete {}", s);
    format!("get {}", s)
}
