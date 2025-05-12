use futures::{StreamExt, pin_mut};
use tokio::{fs::File, io::AsyncWriteExt};

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn write_stream_to_file(
    stream: impl futures::Stream<Item = reqwest::Result<bytes::Bytes>>,
    file: &mut File,
) -> Result<(), Error> {
    futures::pin_mut!(stream);

    while let Some(item) = stream.next().await {
        match item {
            Ok(ref bytes) => {
                file.write_all(&bytes).await?;
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

pub async fn write_stream_to_filename(
    stream: impl futures::Stream<Item = reqwest::Result<bytes::Bytes>>,
    path: &str,
) -> Result<(), Error> {
    let mut file = File::create(path).await?;

    write_stream_to_file(stream, &mut file).await?;

    file.flush().await?;
    file.shutdown().await?;

    Ok(())
}

pub async fn write_stream_to_vec_u8(
    stream: impl futures::Stream<Item = reqwest::Result<bytes::Bytes>>,
) -> Result<Vec<u8>, Error> {
    pin_mut!(stream);

    let mut bytes = Vec::<u8>::new();
    while let Some(item) = stream.next().await {
        match item {
            Ok(ref new_bytes) => {
                let thing: Vec<u8> = new_bytes.iter().map(|b| *b).collect::<Vec<u8>>();
                bytes.extend(thing);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }
    Ok(bytes)
}
