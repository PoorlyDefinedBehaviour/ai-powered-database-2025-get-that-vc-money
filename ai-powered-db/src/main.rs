// use byteorder::{ByteOrder, LittleEndian};
use async_compression::tokio::write::ZstdEncoder;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt};

#[tokio::main]
async fn main(){
    // #TODO Durability https://calvin.loncaric.us/articles/CreateFile.html
    let dir = OpenOptions::new().read(true).open("./").await.unwrap();

     let log_insert = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open("./foo")
        .await.unwrap();

    dir.sync_all().await.unwrap();        

    let msg = Message{
        key: "12312".to_string(),
        value: "value1231".to_string()
    };
// #TODO lidar com erro
    publish(vec![msg], log_insert).await.unwrap(); 
}

#[derive(Debug)]
struct Message {
        key : String,
        value: String
}

async fn publish(msg: Vec<Message>, mut log: tokio::fs::File)-> Result<(), std::io::Error>{
    let buffer = Vec::new();
    let mut encd = ZstdEncoder::new(buffer);

    for x in msg {
        encd.write_all(&[x.key.len() as u8]).await?;
        encd.write_all(x.key.as_bytes()).await?;
        encd.write_all(&(x.value.len() as u16).to_le_bytes()).await?;
        encd.write_all( x.value.as_bytes()).await?;
    };
    
    encd.flush().await?;
    let compressed_buffer = encd.into_inner();
    let checksum = crc32fast::hash(&compressed_buffer);
    log.write_all(&checksum.to_le_bytes()).await?;
    log.write_all(&(compressed_buffer.len() as u16).to_le_bytes()).await?;
    log.write_all(&compressed_buffer).await?;    

    log.sync_all().await?;

    Ok(())
}
    // #TODO Durability

