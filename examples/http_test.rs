use tokio::runtime;

fn main() {
    let rt = runtime::Builder::new_current_thread().build().unwrap();

    rt.block_on(async move {
        let response = reqwest::get("http://127.0.0.1:8080/").await.unwrap();

        let text = response.text().await.unwrap();

        println!("{}", text);
    });
}
