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

#[derive(Debug)]
struct Message {
        // timestamp: i64,
        key : String,
        value: String
        // process_data: i64
}

async fn publish(mut msg: Vec<Message>, mut log: tokio::fs::File)-> Result<(), std::io::Error>{
    let mut buffer: Vec<u8> = Vec::new();
    for x in msg {
        std::io::Write::write_all(&mut buffer, &[x.key.len() as u8])?;
        std::io::Write::write_all(&mut buffer, x.key.as_bytes())?;
        std::io::Write::write_all(&mut buffer, &(x.value.len() as u16).to_le_bytes())?;
        std::io::Write::write_all(&mut buffer, x.value.as_bytes())?;
    };
    let checksum = crc32fast::hash(&buffer);
    std::io::Write::write_all(&mut buffer, &checksum.to_le_bytes())?;
    

    log.write_all(&buffer).await.unwrap();
    return Ok(());
}
    // #TODO Durability

