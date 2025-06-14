use byteorder::{ByteOrder, LittleEndian};

use tokio::fs::OpenOptions;
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main(){
    // #TODO Durability https://calvin.loncaric.us/articles/CreateFile.html
     let log_insert = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open("foo")
        .await.unwrap();

    let msg = Message{
        key: "12312".to_string(),
        value: "value1231".to_string()
    };

    publish(vec![msg], log_insert).await;
}
