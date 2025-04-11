use tokio::sync::mpsc;

/*
线程轮流打印的几种方式，这种是使用 消息队列 的方式；
但是不论哪种方式，都是在一个线程执行完后，需要通知下一个线程;
使用 消息队列 的方式，也需要在一个线程执行完后，通知下一个线程！
这里使用notify的方式

线程轮流打印的关键是 顺序，一个线程执行完后，再执行下一个线程！！所以需要通知方式!
通知方式一般也是：共享变量，消息队列，锁notify通知

信号量notify的方式和消息队列的方式很像，Atomic 原子锁的方式比较不一样, Atomic的方式和锁Mutext控制一个状态的变量的方式本质一样；
实际使用，不考虑性能，使用消息广播队列控制多个协程执行的方式更好，可读性也更好！
3个协程以上按顺序执行，就只能用Atomic状态变量方法或者广播消息队列的方法了;Notify的方法不适用了！

但是核心都是控制顺序，控制顺序的方式都是 执行完任务后，通知下一个执行，
通知的方式要么是共享的中心数据状态的修改，要么是通过消息发送！
 */


#[tokio::main]
async fn main() {
    let (tx,rx) = flume::unbounded::<i32>();
    let notify1 = std::sync::Arc::new(tokio::sync::Notify::new());
    let notify2 = std::sync::Arc::new(tokio::sync::Notify::new());

    let mut tasks = vec![];
    //producer
    let jd1 = tokio::spawn(async move {
        for i in 0..20 {
            let _ = tx.send_async(i).await; 
        }
    });
    
    let notify1a = notify1.clone();
    let notify2a = notify2.clone();
    let rxk = rx.clone();
    let jd2 = tokio::spawn(async move {
        let _ = notify1a.notified().await;
        'out: loop {
            if let Ok(msg) = rxk.recv_async().await {
                println!("consumer: {}, msg: {}", 0, msg);
                notify2a.notify_waiters();
            } else {
                println!("exit loop1");
                notify2a.notify_waiters();
                break 'out;
            }
        }
    });

    // let rx = rx.clone();
    // let cnt = cnt.clone();
    let notify1a = notify1.clone();
    let notify2a = notify2.clone();
    let rxk = rx.clone();
    let jd3 = tokio::spawn(async move {
        'out: loop {
            let _ = notify2a.notified().await;
            // let _ = rx2.recv_async().await;
            if let Ok(msg) = rxk.recv_async().await {
                println!("consumer: {}, msg: {}", 1, msg);
                let _ = notify1a.notify_waiters();
            } else {
                // println!("recv error");
                let _ = notify1a.notify_waiters();
                println!("exit loop2");
                break 'out;
            }
        }
    });

    let _ = tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    notify1.notify_waiters();
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