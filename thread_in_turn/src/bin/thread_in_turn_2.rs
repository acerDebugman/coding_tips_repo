use tokio::sync::mpsc;

/*
线程轮流打印的几种方式，这种是使用 Atomic 的方式；
并且用锁Mutex 控制一个状态变量，然后执行的效果和使用Atomic效果是一样的；
但是不论哪种方式，都是在一个线程执行完后，需要通知下一个线程，这里使用Atomic的方式去通知！
 */

#[tokio::main]
async fn main() {
    let (tx,rx) = flume::unbounded::<i32>();

    let mut tasks = vec![];
    //producer
    let jd1 = tokio::spawn(async move {
        for i in 0..20 {
            let _ = tx.send_async(i).await; 
        }
    });
    

    // let cnt = std::sync::atomic::AtomicU128::new(0);
    let cnt = std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0));

    let rx1 = rx.clone();
    let cnt1 = cnt.clone();
    let jd2 = tokio::spawn(async move {
        'out: loop {
            while cnt1.load(std::sync::atomic::Ordering::Relaxed) % 2 == 0 {
                // println!("p1 {}", cnt1.load(std::sync::atomic::Ordering::Relaxed));
                if let Ok(msg) = rx1.recv_async().await {
                    println!("consumer: {}, msg: {}", 0, msg);
                    cnt1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                } else {
                    println!("loop1");
                    cnt1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    break 'out;
                }
            } 
        }
    });

    // let rx = rx.clone();
    // let cnt = cnt.clone();
    let jd3 = tokio::spawn(async move {
        'out: loop {
            while cnt.load(std::sync::atomic::Ordering::SeqCst) % 2 == 1 {
                // println!("p2 {}", cnt.load(std::sync::atomic::Ordering::Relaxed));
                if let Ok(msg) = rx.recv_async().await {
                    println!("consumer: {}, msg: {}", 1, msg);
                    cnt.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                } else {
                    // println!("recv error");
                    println!("loop2");
                    cnt.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
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


//这个方法实现不了，while 应该在 队列收消息的recv() 前面
pub async fn use_atomic_2() {
    let (tx,rx) = flume::unbounded::<i32>();

    let mut tasks = vec![];
    //producer
    let jd1 = tokio::spawn(async move {
        for i in 0..20 {
            let _ = tx.send_async(i).await; 
        }
    });
    

    // let cnt = std::sync::atomic::AtomicU128::new(0);
    let cnt = std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0));

    let rx1 = rx.clone();
    let cnt1 = cnt.clone();
    let jd2 = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        'out: loop {
                // println!("p1 {}", cnt1.load(std::sync::atomic::Ordering::Relaxed));
                if let Ok(msg) = rx1.recv_async().await {
                    //while 放到外层是最好的，就是sec.rs实现的方法
                    while cnt1.load(std::sync::atomic::Ordering::SeqCst) % 2 != 0 { }
                    println!("consumer: {}, msg: {}", 0, msg);
                    cnt1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                } else {
                    println!("loop1");
                    cnt1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    break 'out;
                }
        }
    });

    // let rx = rx.clone();
    // let cnt = cnt.clone();
    // tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let rx2 = rx.clone();
    let jd3 = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        'out: loop {
                // println!("p2 {}", cnt.load(std::sync::atomic::Ordering::Relaxed));
                if let Ok(msg) = rx2.recv_async().await {
                    while cnt.load(std::sync::atomic::Ordering::SeqCst) % 2 != 1 {}
                    println!("consumer: {}, msg: {}", 1, msg);
                    cnt.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                } else {
                    // println!("recv error");
                    println!("loop2");
                    cnt.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
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
}