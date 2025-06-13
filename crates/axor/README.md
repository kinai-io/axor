# Axor

**Axor** is a zero-overhead, modular backend framework based on agents.

It helps you structure your backend logic into isolated, injectable components — called **agents** — that expose typed operations and can be invoked either directly or dynamically via RPC.

> _“Give me six hours to chop down a tree and I will spend the first four sharpening the axe.”_  
> — Abraham Lincoln

Axor is your sharpened axe: focus on business logic, not boilerplate.

✅ Clean design  
🧪 No ceremony  
🚀 Zero runtime cost when calling directly  
🌐 RPC-style invocation when needed  
🔌 Inject and share **any service**, with **no trait or macro required**

Agents are exposed to the runtime. Services, in contrast, can be injected and shared without being registered as public operations — keeping your context clean and focused.

---

## ✨ Features

- ✅ **Strongly-typed agents** with auto-injection
- 🔌 **Service/Agent DI** via `Inject<T>`
- ⚙️ **Operations** with `#[operation]`, exposed automatically
- 🧪 **Business logic is testable** directly (no runtime required)
- 🚀 **Zero overhead** in direct mode, thanks to static dispatch
- 🌐 **Optional RPC-style interface** via `Payload`

> Use `axor::prelude::*` for a complete, developer-friendly import.

---

## 📦 Installation

```toml
[dependencies]
axor = "0.1"
axor-macros = "0.1" # Required for attribute macros
````

> ⚠️ You must declare `axor-macros` in your `Cargo.toml`,
> but you don't need to import it in your source code — just use the prelude:

```rust
use axor::prelude::*;
```

---

## 🛠️ Basic usage

### 1. Define a service and an agent

```rust
use std::sync::Arc;
use axor::prelude::*;

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
struct HelloAgent {
    logger: Inject<Arc<dyn Logger>>,
}

#[agent_impl]
impl HelloAgent {
    #[operation]
    fn hello(&self) -> &'static str {
        self.logger.resolve().log("Saying hello");
        "Hello, world!"
    }
}
```

---

### 2. Register and run

```rust
let mut context = AxorContext::new();
context.register(HelloAgent::default());
context.register_service::<Arc<dyn Logger>>(Arc::new(ConsoleLogger));
context.init();

// Direct call (zero overhead)
let agent = context.resolve::<HelloAgent>();
assert_eq!(agent.hello(), "Hello, world!");

// RPC-style used by web, cli and tauri runtimes
let payload = Payload::new("HelloAgent.hello");
let result = context.invoke(payload);
assert!(result.success);
```

---

## 📜 Manifest support

Introspect all registered agents and operations:

```rust
let manifest = context.manifest();
println!("{}", serde_json::to_string_pretty(&manifest).unwrap());
```

---

## ⚠️ Limitations

### ❌ No reflection

Rust doesn’t provide reflection. You must:

* Add `#[agent]` to your struct
* Add `#[agent_impl]` to your impl block
* Use `#[operation]` to expose methods

### 🎯 Operation constraints

Each operation may take **zero or one input**, which must:

* Be `DeserializeOwned`
* Not be a reference (e.g., use `String`, not `&str`)

Return values must be `Serialize`, but they are **optional**.

### 🔄 Runtime cost only when using `Payload`

In direct mode, everything is statically dispatched and compiled away.
RPC-style invocation uses `serde_json::Value`, which involves serialization overhead — by design.

---

## 🧭 Roadmap

* [x] Core agent system and macros
* [x] Dependency injection with `Inject<T>`
* [x] Operation exposure
* [x] Manifest generation
* [ ] Type metadata for operations
* [ ] `axor-web` (RPC over HTTP)
* [ ] `axor-cli` (invoke agents from CLI)
* [ ] `axor-tauri` (bindings for desktop apps)
* [ ] TypeScript client generator
* [ ] `axor-doc` for endpoint documentation

---

## 🔗 License

MIT © 2025 — Made with ❤️ in Rust