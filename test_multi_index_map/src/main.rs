use std::sync::Arc;
use std::fmt::Debug;

use multi_index_map::MultiIndexMap;
use tokio::sync::RwLock;



#[derive(Debug)]
pub struct Activity(pub usize);


#[derive(MultiIndexMap, Debug)]
pub struct AgentTask {
    #[multi_index(hashed_non_unique)]
    pub agent_id: i64,
    #[multi_index(ordered_unique)]
    pub task_id: usize,
    pub agent_state: Arc<RwLock<String>>,
    pub sender: tokio::sync::mpsc::Sender<Activity>,
    pub stop_sender: Arc<tokio::sync::oneshot::Sender<anyhow::Result<()>>>,
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
    let agent_tasks = Arc::new(RwLock::new(MultiIndexAgentTaskMap::default()));
    tokio::spawn({
        let agent_tasks = agent_tasks.clone();
        async move {
            loop {
                let agent_task = AgentTask { 
                    agent_id: todo!(), 
                    task_id: todo!(), 
                    agent_state: todo!(), 
                    sender: todo!(), 
                    stop_sender: todo!() 
                };
            }
            agent_tasks.write().await.insert(agent_task);
            
        }
    });
    

    // agent_tasks.write().await.insert(AgentTask {

    Ok(())
}