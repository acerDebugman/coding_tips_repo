use std::{io::Write, sync::Arc};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    // 0: A 还没打印完；1: A 已打印完，可以打印 B
    let flag = Arc::new(RwLock::new(0u8));

    let flag_a = flag.clone();
    let handle_a = tokio::spawn(async move {
        loop {
            // 先拿写锁，保证独占
            let mut lock = flag_a.write().await;
            for i in 1..=100 {
                println!("A_{i}");
                // if i % 10 == 0 {
                //     println!(); // 每 10 个换行，方便肉眼检查
                // }
            }
            std::io::stdout().flush().unwrap();
            *lock = 1; // 标记 A 阶段完成
                    // 锁在这里释放
        }
    });

    let flag_b = flag.clone();
    let handle_b = tokio::spawn(async move {
        loop {
            // 等待 A 阶段结束（flag 变成 1）
            // loop {
            //     {
            //         let lock = flag_b.read().await;
            //         if *lock == 1 {
            //             break;
            //         }
            //     } // 读锁释放，避免忙等
            //     tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            // }

            // 升级为写锁，独占打印 B
            let mut lock = flag_b.write().await;
            for i in 1..=100 {
                println!("BBBBBBBBB");
                // if i % 10 == 0 {
                //     println!();
                // }
            }
            std::io::stdout().flush().unwrap();
            *lock = 2; 
        }
    });

    // 等待两个任务完成
    let _ = tokio::join!(handle_a, handle_b);
    println!("\n=== 全部完成 ===");
}