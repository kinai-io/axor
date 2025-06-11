use serde_json::Value;

use crate::{Agent, AxorContext, Inject, OperationDescriptor, Payload};

struct HelloAgent;

impl HelloAgent {
    
    fn hello(&self) -> &'static str {
        println!("From Hello Agent:");
        "Hello, world!"
    }
}

impl Agent for HelloAgent {

    fn name(&self) -> &'static str {
        "HelloAgent"
    }

    fn operations(&self) -> Vec<crate::OperationDescriptor> {
        vec![OperationDescriptor { name: "hello" }]
    }

    fn inject_dependencies(&self, _context: &AxorContext) {}

    fn call_operation(&self, payload: &crate::Payload) -> Option<Value> {
        match payload.op_name_unchecked() {
            "hello" => {
                let res = self.hello();
                let json_data = serde_json::to_value(res).expect("Response Serialization Error");
                Some(json_data)
            }
            _ => None,
        }
    }
}

struct PrintAgent;

impl PrintAgent {
    fn print_message(&self, message: &str) {
        println!("From PrintAgent : {}", message);
    }
}

impl Agent for PrintAgent {
    fn name(&self) -> &'static str {
        "PrintAgent"
    }

    fn operations(&self) -> Vec<crate::OperationDescriptor> {
        vec![OperationDescriptor {
            name: "prinln_message",
        }]
    }
    fn inject_dependencies(&self, _context: &AxorContext) {}

    fn call_operation(&self, payload: &crate::Payload) -> Option<Value> {
        match payload.op_name_unchecked() {
            "print_message" => {
                let message: String = payload.input_as()?;
                self.print_message(message.as_ref());
                None
            }
            _ => None,
        }
    }
}

#[derive(Default)]
struct WorkflowAgent {
    hello: Inject<HelloAgent>,
    print: Inject<PrintAgent>,
}

impl WorkflowAgent {
    fn run(&self) -> &str {
        let hello = self.hello.resolve();
        let print_agent = self.print.resolve();
        let message = hello.hello();
        print_agent.print_message(message);
        message
    }
}

impl Agent for WorkflowAgent {

    fn name(&self) -> &'static str {
        "WorkflowAgent"
    }

    fn operations(&self) -> Vec<crate::OperationDescriptor> {
        vec![OperationDescriptor { name: "run" }]
    }

    fn inject_dependencies(&self, context: &AxorContext) {
        self.hello.inject(context.resolve::<HelloAgent>());
        self.print.inject(context.resolve::<PrintAgent>());
    }

    fn call_operation(&self, payload: &crate::Payload) -> Option<Value> {
        match payload.op_name_unchecked() {
            "hello" => {
                let res = self.run();
                let json_data = serde_json::to_value(res).expect("Response Serialization Error");
                Some(json_data)
            }
            _ => None,
        }
    }
}

#[test]
fn hello_world_no_macros() {
    let context = AxorContext::new();

    context.register(HelloAgent);
    context.register(PrintAgent);
    context.register(WorkflowAgent::default());

    context.init();

    // Direct invocation with type safety
    let agent = context.resolve::<WorkflowAgent>();
    let result = agent.run();

    assert_eq!(result, "Hello, world!");

    println!();
    // Payload invoke from runtimes web, wasm, cli...
    let payload = Payload::with_data("PrintAgent.print_message", &"test".to_string());
    let response = context.invoke(payload);
    println!("Response : {:?}", response);
    assert!(response.success);

    let payload = Payload::new("HelloAgent.hello");
    let response = context.invoke(payload);
    println!("Response : {:?}", response);
    assert!(response.success);

}
