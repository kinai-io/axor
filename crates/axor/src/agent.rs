use crate::{operation::OperationDescriptor, AxorContext, Payload};
use downcast_rs::{impl_downcast, DowncastSync};
use serde_json::Value;

pub trait Agent: DowncastSync + Send + Sync {

    fn name(&self) -> &'static str;

    fn operations(&self) -> Vec<OperationDescriptor>;

    fn inject_dependencies(&self, context: &AxorContext);

    fn call_operation(&self, payload: &Payload) -> Option<Value>;

}

impl_downcast!(sync Agent);
