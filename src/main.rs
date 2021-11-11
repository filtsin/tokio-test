use std::thread::current;
use tokio::runtime::Runtime;
use tokio::task::{self, spawn_local, LocalSet};
use tokio::time::{sleep, Duration};

async fn foo() {
    println!("foo(before sleep): {:?}", current().id());
    sleep(Duration::from_secs(1)).await;
    println!("foo(after sleep): {:?}", current().id());
    tokio::spawn(bar()).await;
    bar().await;
}

async fn bar() {
    println!("bar: {:?}", current().id());
}

async fn main_loop() {
    let mut i = 1;
    loop {
        println!("main_loop: {:?}", current().id());

        if i == 1 {
            task::spawn_local(foo());
            i -= 1;
        }

        sleep(Duration::from_millis(100)).await;
    }
}

fn main() {
    let rt = Runtime::new().unwrap();
    let local = LocalSet::new();
    local.block_on(&rt, main_loop());
}
