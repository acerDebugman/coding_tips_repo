use std::sync::Arc;
use std::fmt::Debug;

use multi_index_map::MultiIndexMap;
use tokio::io::unix::AsyncFd;
use tokio::runtime::Handle;
use tokio::sync::RwLock;


#[derive(Debug)]
pub struct Activity(pub usize);


#[derive(MultiIndexMap, Debug)]
pub struct AgentTask {
    #[multi_index(hashed_non_unique)]
    pub agent_id: i64,
    #[multi_index(ordered_unique)]
    pub task_id: i64,
    pub agent_state: Arc<RwLock<String>>,
    pub sender: tokio::sync::mpsc::Sender<Activity>,
    pub stop_sender: Arc<tokio::sync::oneshot::Sender<anyhow::Result<()>>>,
    pub rand_n: i64,
}

impl Debug for MultiIndexAgentTaskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_list();
        for task in self.iter_by_task_id() {
            debug.entry(task);
        }
        debug.finish()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let total = 100;
    let agent_tasks = Arc::new(RwLock::new(MultiIndexAgentTaskMap::default()));
    let (tx, _rx) = tokio::sync::mpsc::channel(1000);
    let mut tasks = vec![];
    //one insert
    let jd = tokio::spawn({
        let agent_tasks = agent_tasks.clone();
        let tx = tx.clone();
        async move {
            // loop {
            //     for idx in 0..1000 {
            //         let task = AgentTask { 
            //             agent_id: idx % 10,
            //             task_id: idx as usize,
            //             agent_state: Arc::new(RwLock::new("test".to_string())),
            //             sender: tx.clone(),
            //             stop_sender: Arc::new(tokio::sync::oneshot::channel().0),
            //         };
            //         let mut writer_guard = agent_tasks.write().await;
            //         writer_guard.insert(task);
            //     }
            // }

                for idx in 0..total {
                    let task = AgentTask { 
                        agent_id: idx % total,
                        task_id: idx,
                        agent_state: Arc::new(RwLock::new("test".to_string())),
                        sender: tx.clone(),
                        stop_sender: Arc::new(tokio::sync::oneshot::channel().0),
                        rand_n: rand::random::<i64>(),
                    };
                    let mut writer_guard = agent_tasks.write().await;
                    writer_guard.insert(task);
                }
            println!("insert agent_tasks exit");
        }
    });
    tasks.push(jd);
    //one clear
    tasks.push(tokio::spawn({
        let agent_tasks = agent_tasks.clone();
        async move {
            let mut idx = 0;
            // loop {
            //     idx += 1;
            //     let mut writer_guard = agent_tasks.write().await;
            //     writer_guard.clear();
            //     tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;
            //     if idx % 1000 == 0 {
            //         println!("clear agent_tasks {idx} times");
            //     }
            // }
            
            //will make modify panic
            // loop {
            //     idx += 1;
            //     for _ in 0..total {
            //         let n = rand::random::<i64>() % total;
            //         let mut writer_guard = agent_tasks.write().await;
            //         writer_guard.remove_by_task_id(&n);
            //     }
            //     tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;
            //     if idx % 1000 == 0 {
            //         println!("clear agent_tasks {idx} times");
            //     }
            // }

            //will make modify panic
            loop {
                idx += 1;
                let mut writer_guard = agent_tasks.write().await;
                for _ in 0..total {
                    let n = rand::random::<i64>() % total;
                    println!("will remove task_id {n}");
                    writer_guard.remove_by_task_id(&n);
                }
                tokio::time::sleep(tokio::time::Duration::from_micros(1)).await;
                if idx % 1000 == 0 {
                    println!("clear agent_tasks {idx} times");
                }
            }
            println!("clear dify agent_tasks exit");
        }
    }));
    //one modify
    tasks.push(tokio::spawn({
        let agent_tasks = agent_tasks.clone();
        async move {
            loop {
                let mut writer_guard = agent_tasks.write().await;
                for idx in 0..total {
                    println!("modify agent_id {idx}");
                    writer_guard.modify_by_agent_id(&idx, |task| {
                        task.rand_n = rand::random::<i64>();
                        // tokio::task::block_in_place(|| {
                        //     Handle::current().block_on(async {
                        //         *task.agent_state.write().await = "test_modified".to_string();
                        //     })
                        // }) 
                    });
                }
            }
            println!("modify agent_tasks exit");
        }
    }));

    for task in tasks {
        task.await?;
    }

    Ok(())
}