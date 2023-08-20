use std::net::SocketAddr;

use mini_redis_rpc::gen::volo_gen::redis::{RedisServiceClientBuilder, SetReq};

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:6379".parse().unwrap();

    let client = RedisServiceClientBuilder::new("redis")
        .address(addr)
        .build();

    client
        .set(SetReq {
            key: "foo".into(),
            value: "bar".into(),
            expires: None,
        })
        .await
        .unwrap();
    assert_eq!(
        client.get("foo".into()).await.unwrap().value,
        Some("bar".into())
    );
    assert_eq!(client.del(vec!["foo".into()]).await.unwrap(), 1);
    assert!(client.get("foo".into()).await.unwrap().value.is_none());

    println!("set/get/del ok");

    client
        .set(SetReq {
            key: "foo".into(),
            value: "bar".into(),
            expires: Some(10),
        })
        .await
        .unwrap();
    assert_eq!(
        client.get("foo".into()).await.unwrap().value,
        Some("bar".into())
    );
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    assert!(client.get("foo".into()).await.unwrap().value.is_none());
    assert_eq!(client.del(vec!["foo".into()]).await.unwrap(), 0);

    println!("set/get/del with expires ok");

    client.ping().await.unwrap();

    println!("ping ok");

    assert_eq!(client.publish("foo".into(), "bar".into()).await.unwrap(), 0);

    println!("publish ok");

    tokio_scoped::scope(|scope| {
        scope.spawn(async {
            let sub = client.subscribe(vec!["foo".into()]).await.unwrap();
            assert_eq!(sub, vec!["bar"]);
        });
        scope.spawn(async {
            let sub = client
                .subscribe(vec!["foo".into(), "foo1".into()])
                .await
                .unwrap();
            assert_eq!(sub, vec!["bar", "bar1"]);
        });
        scope.spawn(async {
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            assert_eq!(client.publish("foo".into(), "bar".into()).await.unwrap(), 2);
            assert_eq!(
                client.publish("foo1".into(), "bar1".into()).await.unwrap(),
                1
            );
        });
    });
    println!("subscribe/publish ok");
}
