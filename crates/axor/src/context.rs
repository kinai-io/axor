use crate::{Agent, InvokeResult, Payload};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct AxorContext {
    agents: RwLock<HashMap<TypeId, Arc<dyn Agent>>>,
    services: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl AxorContext {
    pub fn new() -> Self {
        Self {
            agents: RwLock::new(HashMap::new()),
            services: RwLock::new(HashMap::new()),
        }
    }

    pub fn register<T: Agent + 'static>(&self, agent: T) {
        let agent_arc: Arc<T> = Arc::new(agent);
        let agent_dyn: Arc<dyn Agent> = agent_arc.clone();
        let service_dyn: Arc<dyn Any + Send + Sync> = agent_arc;

        self.agents
            .write()
            .unwrap()
            .insert(TypeId::of::<T>(), agent_dyn);
        self.services
            .write()
            .unwrap()
            .insert(TypeId::of::<T>(), service_dyn);
    }

    pub fn get<T: Agent + 'static>(&self) -> Option<Arc<T>> {
        let map = self.agents.read().unwrap();
        map.get(&TypeId::of::<T>())
            .and_then(|agent| agent.clone().downcast_arc::<T>().ok())
    }

    pub fn register_service<T: Send + Sync + 'static>(&self, service: T) {
        let mut map = self.services.write().unwrap();
        map.insert(TypeId::of::<T>(), Arc::new(service));
    }

    pub fn get_service<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        let map = self.services.read().unwrap();
        map.get(&TypeId::of::<T>())?.clone().downcast::<T>().ok()
    }

    pub fn resolve<T: Send + Sync + 'static>(&self) -> Arc<T> {
        let map = self.services.read().unwrap();
        let service = map
            .get(&TypeId::of::<T>())
            .expect("Service not found")
            .clone();

        downcast_arc::<T>(service).expect("Type mismatch when downcasting service")
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
                    return result;
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

pub trait DowncastArc: Any + Send + Sync {
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
}

impl<T: Any + Send + Sync> DowncastArc for T {
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
        self
    }
}

pub fn downcast_arc<T: Any + Send + Sync>(arc: Arc<dyn Any + Send + Sync>) -> Result<Arc<T>, Arc<dyn Any + Send + Sync>> {
    if arc.is::<T>() {
        // SAFETY: verified via is::<T>()
        let ptr = Arc::into_raw(arc) as *const T;
        unsafe { Ok(Arc::from_raw(ptr)) }
    } else {
        Err(arc)
    }
}