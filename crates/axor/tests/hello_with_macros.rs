use std::sync::Arc;

use axor::{Agent, Inject, AxorContext, Payload, OperationDescriptor, InvokeResult};
use axor_macros::{agent, agent_impl, operation};

#[agent]
struct HelloAgent {
    pub logger: Inject<Arc<dyn Logger>>,
    pub console_logger: Inject<ConsoleLogger>,
}

#[agent_impl]
impl HelloAgent {

    #[operation]
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

#[agent]
struct PrintAgent;

#[agent_impl]
impl PrintAgent {
    fn print_message(&self, message: &str) {
        println!("From PrintAgent : {}", message);
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

#[agent]
struct WorkflowAgent {
    hello: Inject<HelloAgent>,
    print: Inject<PrintAgent>,
}

#[agent_impl]
impl WorkflowAgent {


    fn run(&self) -> &str {
        let hello = self.hello.resolve();
        let print_agent = self.print.resolve();
        let message = hello.hello();
        print_agent.print_message(message);
        message
    }
}

#[test]
fn hello_with_macros() {
    let context = AxorContext::new();

    context.register(HelloAgent::default());
    context.register(PrintAgent);
    context.register(WorkflowAgent::default());
    context.register_service::<Arc<dyn Logger>>(Arc::new(ConsoleLogger));
    context.register_service(ConsoleLogger);

    context.init();

    // Direct invocation with type safety
    // Direct invocation with type safety
    let agent = context.resolve::<PrintAgent>();
    let _ = agent.print_message("Hello world");


    let agent = context.resolve::<WorkflowAgent>();
    let result = agent.run();

    assert_eq!(result, "Hello, world!");

     let logger = context.get_service::<Arc<dyn Logger>>().unwrap();
    logger.log("Do Log !");

    // println!();
    // // Payload invoke from runtimes web, wasm, cli...
    let payload = Payload::with_data("PrintAgent.print_message", &"test".to_string());
    let response = context.invoke(payload);
    println!("Response : {:?}", response);
    assert!(response.success);

    let payload = Payload::new("HelloAgent.hello");
    let response = context.invoke(payload);
    println!("Response : {:?}", response);
    assert!(response.success);

   
}

