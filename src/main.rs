use tokio::sync::mpsc;


#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(400);


    let sender = async move {
        for i in 0..1_000_000 {
            println!("Sending: {}", i);
            tx.send(i).await.unwrap();
        }
    };

    let receiver = async move {
        while let Some(val) = rx.recv().await {
            println!("Receiving {:?}", val);
        }
    };

    tokio::join!(receiver, sender);
}
