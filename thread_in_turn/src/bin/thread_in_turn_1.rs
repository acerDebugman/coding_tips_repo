use futures::StreamExt;
use tokio::sync::mpsc;

/*
线程轮流打印的几种方式，这种是使用 消息队列 的方式；
但是不论哪种方式，都是在一个线程执行完后，需要通知下一个线程;
使用 消息队列 的方式，也需要在一个线程执行完后，通知下一个线程！
通知的方式也可以使用消息队列的方式！!

线程轮流打印的关键是 顺序，一个线程执行完后，再执行下一个线程！！所以需要通知方式!
通知方式一般也是：共享变量，消息队列，锁notify通知
 */

/*
队列确实是解耦非常有用的东西：灵活运用队列，可以有效的做线程解耦！并且在很多场景都有用！！
队列主要方式：
按生产者消费者分：mpmc,mpsc,spsc
按消息是否复制分：消息广播，消息单次消费
如果消息单次消费的队列支持peek()/take() 的语义，也是广播，但是不如直接的广播队好用;
广播消息队列用处也很多,比较难想到的一个场景就是这个协程的执行顺序控制！其他的就是正常的广播场景了！
 */

#[tokio::main]
async fn main() {
    // let _ = queue_approach_1().await;
    // 最优：使用广播队列解耦，在前期就控制
    let _ = queue_approach_2().await;
}
//这种方式：每个协程任务都要多一个queue去通知下一个任务；
//可以换成一个统一的广播线程，类似kafka的消息队列，先peek(),是自己的消息，然后在处理，take()出来；
//先 peek() 再 take() 的方式，就是广播的方式，适合多线程处理任务！
// 否则一上来就多线程take()处理，会出现其他线程没有机会判断消息的情况，下面有个错误例子；
// 下面这个方法是每个协程对应一个队列，效率不高；使用 peek() 的看第二个方法
pub async fn queue_approach_1() {

    let (tx,rx) = flume::unbounded::<i32>();
    //下面这两个用于发送消息，互相通知 各自 的任务执行！
    let (tx1,rx1) = flume::unbounded::<i32>();
    let (tx2,rx2) = flume::unbounded::<i32>();

    let mut tasks = vec![];
    //producer
    let jd1 = tokio::spawn(async move {
        for i in 0..20 {
            let _ = tx.send_async(i).await; 
        }
    });
    
    let _ = tx1.send(0);

    let rxk = rx.clone();
    let jd2 = tokio::spawn(async move {
        'out: loop {
            let _ = rx1.recv_async().await;
            if let Ok(msg) = rxk.recv_async().await {
                println!("consumer: {}, msg: {}", 0, msg);
                let _ = tx2.send(0);
            } else {
                println!("exit loop1");
                break 'out;
            }
        }
    });

    // let rx = rx.clone();
    // let cnt = cnt.clone();
    let rxk = rx.clone();
    let jd3 = tokio::spawn(async move {
        'out: loop {
            let _ = rx2.recv_async().await;
            // let _ = rx2.recv_async().await;
            if let Ok(msg) = rxk.recv_async().await {
                println!("consumer: {}, msg: {}", 1, msg);
                let _ = tx1.send(0);
            } else {
                // println!("recv error");
                println!("exit loop2");
                break 'out;
            }
        }
    });

    tasks.push(jd1);
    tasks.push(jd2);
    tasks.push(jd3);

    
    for jd in tasks {
        let _ = jd.await;
    } 
    //concumser
}

//flume 的队列不支持 peek()/take() 的方式，其实这种 peek()/take() 的方式，就是广播的一种方式
//直接使用tokio::sync::broadcast::channel(16) 广播通道 进行广播消息即可!
pub async fn queue_approach_2() {
    let (tx,rx) = flume::unbounded::<i32>();
    let (btx,mut brx1) = tokio::sync::broadcast::channel::<i32>(16);
    let mut brx2 = btx.subscribe();

    let mut tasks = vec![];
    //producer
    let jd1 = tokio::spawn(async move {
        for i in 0..20 {
            let _ = tx.send_async(i).await; 
        }
    });

    let _ = btx.send(0);
    
    let rx1 = rx.clone();
    let btx1 = btx.clone();
    let jd2 = tokio::spawn(async move {
        'out: loop {
            if let Ok(0) = brx1.recv().await {
                if let Ok(msg) = rx1.recv_async().await {
                    println!("consumer: {}, msg: {}", 0, msg);
                    let _ = btx1.send(1);
                } else {
                    println!("exit loop1");
                    let _ = btx1.send(1);
                    break 'out;
                }
            }
        }
    });

    let rx2 = rx.clone();
    let btx2 = btx.clone();
    let jd3 = tokio::spawn(async move {
        'out: loop {
            if let Ok(1) = brx2.recv().await {
                if let Ok(msg) = rx2.recv_async().await {
                    println!("consumer: {}, msg: {}", 1, msg);
                    let _ = btx2.send(0);
                } else {
                    // println!("recv error");
                    println!("exit loop2");
                    let _ = btx2.send(0);
                    break 'out;
                }
            }
        }
    });

    tasks.push(jd1);
    tasks.push(jd2);
    tasks.push(jd3);
    
    for jd in tasks {
        let _ = jd.await;
    } 
    //concumser
}



//下面这种方式不行，只能使用队列去通知才行！
// #[tokio::main]
// async fn main() {
//     let (tx,rx) = flume::unbounded::<i32>();
//     let (tx1,rx1) = flume::unbounded::<i32>();
//     let (tx2,rx2) = flume::unbounded::<i32>();

//     let mut tasks = vec![];
//     //producer
//     let jd1 = tokio::spawn(async move {
//         for i in 0..20 {
//             if i % 2 == 0 {
//                 let _ = tx1.send_async(i).await; 
//             } else {
//                 let _ = tx2.send_async(i).await; 
//             }
//         }
//     });
    
//     let txk = tx.clone();
//     let rxk = rx.clone();
//     let jd2 = tokio::spawn(async move {
//         'out: loop {
//             if rxk.recv_async().await.unwrap() % 2 != 0 {
//                 continue ;
//             }
//             if let Ok(msg) = rx1.recv_async().await {
//                 println!("consumer: {}, msg: {}", 0, msg);
//                 let _ = txk.send(1);
//             } else {
//                 println!("exit loop1");
//                 break 'out;
//             }
//         }
//     });

//     // let rx = rx.clone();
//     // let cnt = cnt.clone();
//     let txk = tx.clone();
//     let rxk = rx.clone();
//     let jd3 = tokio::spawn(async move {
//         'out: loop {
//             if rxk.recv_async().await.unwrap() % 2 != 1 {
//                 continue;
//             }
//             // let _ = rx2.recv_async().await;
//             if let Ok(msg) = rx2.recv_async().await {
//                 println!("consumer: {}, msg: {}", 1, msg);
//                 let _ = txk.send(0);
//             } else {
//                 // println!("recv error");
//                 println!("exit loop2");
//                 break 'out;
//             }
//         }
//     });

//     let _ = tx.send(0);
//     tasks.push(jd1);
//     tasks.push(jd2);
//     tasks.push(jd3);

    
//     for jd in tasks {
//         let _ = jd.await;
//     } 
//     //concumser
// }