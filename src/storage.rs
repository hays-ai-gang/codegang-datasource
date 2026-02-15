use std::path::{Path, PathBuf};
use std::sync::RwLock;

use crate::model::{
    Datasource, NosqlContract, ProtoContract, QueueContract, ServiceDefinition,
};

pub struct AppState {
    data: RwLock<Datasource>,
    file_path: PathBuf,
}

impl AppState {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        let file_path = file_path.into();
        let data = Self::load_from_file(&file_path).unwrap_or_else(|| Datasource {
            services: Vec::new(),
            queue_contracts: Vec::new(),
            nosql_contracts: Vec::new(),
            proto_contracts: Vec::new(),
        });
        Self {
            data: RwLock::new(data),
            file_path,
        }
    }

    fn load_from_file(path: &Path) -> Option<Datasource> {
        let content = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }

    fn save(&self) {
        let data = self.data.read().unwrap();
        if let Ok(json) = serde_json::to_string_pretty(&*data) {
            let _ = std::fs::write(&self.file_path, json);
        }
    }

    // ── Full datasource ──────────────────────────────────────────

    pub fn get_datasource(&self) -> Datasource {
        self.data.read().unwrap().clone()
    }

    pub fn replace_datasource(&self, ds: Datasource) {
        let mut data = self.data.write().unwrap();
        *data = ds;
        drop(data);
        self.save();
    }

    // ── Services ─────────────────────────────────────────────────

    pub fn get_services(&self) -> Vec<ServiceDefinition> {
        self.data.read().unwrap().services.clone()
    }

    pub fn get_service(&self, name: &str) -> Option<ServiceDefinition> {
        self.data.read().unwrap().services.iter().find(|s| s.name == name).cloned()
    }

    pub fn upsert_service(&self, svc: ServiceDefinition) {
        let mut data = self.data.write().unwrap();
        if let Some(idx) = data.services.iter().position(|s| s.name == svc.name) {
            data.services[idx] = svc;
        } else {
            data.services.push(svc);
        }
        drop(data);
        self.save();
    }

    pub fn delete_service(&self, name: &str) -> Result<(), String> {
        let mut data = self.data.write().unwrap();
        let idx = data.services.iter().position(|s| s.name == name)
            .ok_or_else(|| format!("Service '{name}' not found"))?;
        data.services.remove(idx);
        drop(data);
        self.save();
        Ok(())
    }

    // ── Queue contracts ──────────────────────────────────────────

    pub fn get_queue_contracts(&self) -> Vec<QueueContract> {
        self.data.read().unwrap().queue_contracts.clone()
    }

    pub fn get_queue_contract(&self, topic: &str) -> Option<QueueContract> {
        self.data.read().unwrap().queue_contracts.iter().find(|q| q.topic_name == topic).cloned()
    }

    pub fn upsert_queue_contract(&self, qc: QueueContract) {
        let mut data = self.data.write().unwrap();
        if let Some(idx) = data.queue_contracts.iter().position(|q| q.topic_name == qc.topic_name) {
            data.queue_contracts[idx] = qc;
        } else {
            data.queue_contracts.push(qc);
        }
        drop(data);
        self.save();
    }

    pub fn delete_queue_contract(&self, topic: &str) -> Result<(), String> {
        let mut data = self.data.write().unwrap();
        let idx = data.queue_contracts.iter().position(|q| q.topic_name == topic)
            .ok_or_else(|| format!("Queue contract '{topic}' not found"))?;
        data.queue_contracts.remove(idx);
        drop(data);
        self.save();
        Ok(())
    }

    // ── NoSQL contracts ──────────────────────────────────────────

    pub fn get_nosql_contracts(&self) -> Vec<NosqlContract> {
        self.data.read().unwrap().nosql_contracts.clone()
    }

    pub fn get_nosql_contract(&self, entity: &str) -> Option<NosqlContract> {
        self.data.read().unwrap().nosql_contracts.iter().find(|n| n.entity_name == entity).cloned()
    }

    pub fn upsert_nosql_contract(&self, nc: NosqlContract) {
        let mut data = self.data.write().unwrap();
        if let Some(idx) = data.nosql_contracts.iter().position(|n| n.entity_name == nc.entity_name) {
            data.nosql_contracts[idx] = nc;
        } else {
            data.nosql_contracts.push(nc);
        }
        drop(data);
        self.save();
    }

    pub fn delete_nosql_contract(&self, entity: &str) -> Result<(), String> {
        let mut data = self.data.write().unwrap();
        let idx = data.nosql_contracts.iter().position(|n| n.entity_name == entity)
            .ok_or_else(|| format!("NoSQL contract '{entity}' not found"))?;
        data.nosql_contracts.remove(idx);
        drop(data);
        self.save();
        Ok(())
    }

    // ── Proto contracts ──────────────────────────────────────────

    pub fn get_proto_contracts(&self) -> Vec<ProtoContract> {
        self.data.read().unwrap().proto_contracts.clone()
    }

    pub fn get_proto_contract(&self, name: &str) -> Option<ProtoContract> {
        self.data.read().unwrap().proto_contracts.iter().find(|p| p.name == name).cloned()
    }

    pub fn upsert_proto_contract(&self, pc: ProtoContract) {
        let mut data = self.data.write().unwrap();
        if let Some(idx) = data.proto_contracts.iter().position(|p| p.name == pc.name) {
            data.proto_contracts[idx] = pc;
        } else {
            data.proto_contracts.push(pc);
        }
        drop(data);
        self.save();
    }

    pub fn delete_proto_contract(&self, name: &str) -> Result<(), String> {
        let mut data = self.data.write().unwrap();
        let idx = data.proto_contracts.iter().position(|p| p.name == name)
            .ok_or_else(|| format!("Proto contract '{name}' not found"))?;
        data.proto_contracts.remove(idx);
        drop(data);
        self.save();
        Ok(())
    }
}
