use mini_redis::{buffer, client};
/// A basic "hello world" style test. A server instance is started in a
/// background task. A client instance is then established and used to intialize
/// the buffer. Set and get commands are sent to the server. The response is
/// then evaluated.
#[tokio::test]
async fn pool_key_value_get_set() {
    let client = client::connect("127.0.0.1:6379").await.unwrap();
    let mut client = buffer(client);

    client.set("hello", "world".into()).await.unwrap();

    let value = client.get("hello").await.unwrap().unwrap();
    assert_eq!(b"world", &value[..])
}
