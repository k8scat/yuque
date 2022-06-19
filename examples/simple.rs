use yuque::yuque::Yuque;

#[tokio::main]
async fn main() {
    let token = option_env!("YUQUE_TOKEN").unwrap_or_default();
    let yuque = Yuque::new(token).unwrap();
    let user = yuque.get_auth_user().await.unwrap();
    println!("{:#?}", user);
}