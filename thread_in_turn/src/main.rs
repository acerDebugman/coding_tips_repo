use tokio::sync::mpsc;

//协程无顺序执行打印
#[tokio::main]
async fn main() {
    let (tx,rx) = flume::unbounded::<i32>();

    let mut tasks = vec![];
    //producer
    let tx1 = tx.clone();
    let jd1 = tokio::spawn(async move {
        for i in 0..10 {
            let _ = tx1.send_async(i).await; 
        }
    });
    tasks.push(jd1);
    let jd2 = tokio::spawn(async move {
        for i in 10..20 {
            let _ = tx.send_async(i).await; 
        }
    });
    tasks.push(jd2);

    for i in 0..2 {
        let rx = rx.clone();
        let jd = tokio::spawn(async move {
            loop {
                if let Ok(msg) = rx.recv_async().await {
                    println!("consumer: {}, msg: {}", i, msg);
                } else {
                    // println!("recv error");
                    break;
                }
            }
        });
        tasks.push(jd);
    }

    
    for jd in tasks {
        let _ = jd.await;
    } 
    //concumser
}