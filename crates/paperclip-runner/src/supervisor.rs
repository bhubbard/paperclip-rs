use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[derive(Clone, Debug)]
pub struct SupervisedTask {
    pub run_id: String,
    pub agent_id: String,
    pub issue_id: String,
}

pub struct AgentSupervisor {
    active_runs: Arc<Mutex<HashMap<String, SupervisedTask>>>,
    cancel_sender: broadcast::Sender<String>,
}

impl Default for AgentSupervisor {
    fn default() -> Self {
        let (tx, _) = broadcast::channel(32);
        Self {
            active_runs: Arc::new(Mutex::new(HashMap::new())),
            cancel_sender: tx,
        }
    }
}

impl AgentSupervisor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_run(&self, task: SupervisedTask) {
        let mut guard = self.active_runs.lock().unwrap();
        guard.insert(task.run_id.clone(), task);
    }

    pub fn unregister_run(&self, run_id: &str) -> Option<SupervisedTask> {
        let mut guard = self.active_runs.lock().unwrap();
        guard.remove(run_id)
    }

    pub fn cancel_run(&self, run_id: &str) {
        let _ = self.cancel_sender.send(run_id.to_string());
    }

    pub fn active_count(&self) -> usize {
        let guard = self.active_runs.lock().unwrap();
        guard.len()
    }

    pub fn subscribe_cancel(&self) -> broadcast::Receiver<String> {
        self.cancel_sender.subscribe()
    }

    pub fn is_agent_busy(&self, agent_id: &str) -> bool {
        let guard = self.active_runs.lock().unwrap();
        guard.values().any(|t| t.agent_id == agent_id)
    }
}
