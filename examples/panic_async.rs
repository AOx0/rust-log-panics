use std::time::Duration;

extern crate env_logger;
extern crate log_panics;

#[tokio::main]
async fn main() {
    env_logger::init();
    let (sender, receiver) = flume::unbounded();

    let fut = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(5)).await;
        println!("Panic");
    });

    let panic = log_panics::Config::default()
        .cleanup_ready(receiver)
        .install_panic_hook();

    tokio::spawn(async move {
        if panic.recv_async().await.is_ok() {
            println!("Panic received");
            let _ = fut.await;
            let _ = sender.send(());
        }
    });

    foo();
}

fn foo() {
    panic!();
}
