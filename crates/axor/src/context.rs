use crate::{Agent, InvokeResult, Payload};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct AxorContext {
    agents: RwLock<HashMap<TypeId, Arc<dyn Agent>>>,
}

impl AxorContext {
    pub fn new() -> Self {
        Self {
            agents: RwLock::new(HashMap::new()),
        }
    }

    pub fn register<T: Agent + 'static>(&self, agent: T) {
        let mut map = self.agents.write().unwrap();
        map.insert(TypeId::of::<T>(), Arc::new(agent));
    }

    pub fn get<T: Agent + 'static>(&self) -> Option<Arc<T>> {
        let map = self.agents.read().unwrap();
        map.get(&TypeId::of::<T>())
            .and_then(|agent| agent.clone().downcast_arc::<T>().ok())
    }

    pub fn resolve<T: Agent + 'static>(&self) -> Arc<T> {
        self.get::<T>().expect("Agent not found in AxorContext")
    }

    pub fn init(&self) {
        let agents = self.agents.read().unwrap();
        for agent in agents.values() {
            agent.inject_dependencies(self);
        }
    }

    pub fn invoke(&self, payload: Payload) -> InvokeResult {
        let (agent_name, _) = match payload.name.split_once('.') {
            Some(res) => res,
            None => {
                return InvokeResult {
                    operation: payload.name.to_string(),
                    data: None,
                    success: false,
                }
            }
        };

        let agents = self.agents.read().unwrap();

        for agent in agents.values() {
            if agent.name() == agent_name {
                if payload.op_name().is_some() {
                    let result = agent.call_operation(&payload);
                    return InvokeResult {
                        operation: payload.name.to_string(),
                        data: result,
                        success: true,
                    };
                }
            }
        }
        InvokeResult {
            operation: payload.name.to_string(),
            data: None,
            success: false,
        }
    }
}
