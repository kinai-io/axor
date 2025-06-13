use std::sync::Arc;

use axor::{Agent, AxorContext, Inject, InvokeResult, OperationDescriptor, Payload};

#[derive(Default)]
struct HelloAgent {
    pub logger: Inject<Arc<dyn Logger>>,
    pub console_logger: Inject<ConsoleLogger>,
}

impl HelloAgent {
    fn hello(&self) -> &'static str {
        // Resolve by trait
        let logger = self.logger.resolve();
        let logger = logger.as_ref();
        logger.log("HelloAgent Trait");

        // Resolve by concrete type
        let console = self.console_logger.resolve();
        console.log("HelloAgent Console");

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

    fn inject_dependencies(&self, context: &AxorContext) {
        self.logger.from_context(context);
        self.console_logger.from_context(context);
        // self.logger.inject(logger);
    }

    fn call_operation(&self, payload: &crate::Payload) -> InvokeResult {
        let return_value = match payload.op_name_unchecked() {
            "hello" => {
                let res = self.hello();
                let json_data = serde_json::to_value(res).expect("Response Serialization Error");
                Some(json_data)
            }
            _ => {
                return InvokeResult {
                    operation: payload.name.to_string(),
                    success: true,
                    data: None,
                }
            }
        };
        InvokeResult {
            operation: payload.name.to_string(),
            success: true,
            data: return_value,
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

    fn call_operation(&self, payload: &crate::Payload) -> InvokeResult {
        let return_value = match payload.op_name_unchecked() {
            "print_message" => {
                let message: String = match payload.input_as() {
                    Some(input) => input,
                    None => {
                        return InvokeResult {
                            operation: payload.name.to_string(),
                            success: false,
                            data: None,
                        }
                    }
                };
                self.print_message(message.as_ref());
                None
            }
            _ => {
                return InvokeResult {
                    operation: payload.name.to_string(),
                    success: true,
                    data: None,
                }
            }
        };
        InvokeResult {
            operation: payload.name.to_string(),
            success: true,
            data: return_value,
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
        self.hello.from_context(context);
        self.print.from_context(context);
    }

    fn call_operation(&self, payload: &crate::Payload) -> InvokeResult {
        let return_value = match payload.op_name_unchecked() {
            "hello" => {
                let res = self.run();
                let json_data = serde_json::to_value(res).expect("Response Serialization Error");
                Some(json_data)
            }
            _ => {
                return InvokeResult {
                    operation: payload.name.to_string(),
                    success: true,
                    data: None,
                }
            }
        };
        InvokeResult {
            operation: payload.name.to_string(),
            success: true,
            data: return_value,
        }
    }
}

trait Logger: Send + Sync {
    fn log(&self, message: &str);
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[LOG] {}", message);
    }
}

#[test]
fn hello_world_no_macros() {
    let context = AxorContext::new();

    context.register(HelloAgent::default());
    context.register(PrintAgent);
    context.register(WorkflowAgent::default());
    context.register_service::<Arc<dyn Logger>>(Arc::new(ConsoleLogger));
    context.register_service(ConsoleLogger);

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

    let logger = context.get_service::<Arc<dyn Logger>>().unwrap();
    logger.log("Do Log !");
}
