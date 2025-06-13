# Axor

**Axor** is a zero-overhead, modular framework for backend development in Rust, built around injectable, self-contained **agents**.

It helps you structure business logic into typed units with automatic dependency injection and optional RPC-style invocation — all with zero runtime cost when called directly.

> _“Give me six hours to chop down a tree and I will spend the first four sharpening the axe.”_  
> — Abraham Lincoln

Axor is your sharpened axe: focus on business logic, not boilerplate.

✅ Clean design  
🧪 No ceremony  
🚀 Zero-cost direct calls (fully static)  
🌐 Optional RPC interface via `Payload`  
🔌 Injectable services, no trait or macro required

Agents are **exposed** to the runtime. Services, in contrast, can be injected and shared **without being publicly invocable** — keeping your context clean and focused.

---

## ✨ Features

- ✅ **Strongly-typed agents** with `#[agent]` and `#[agent_impl]`
- 🔌 **Service or agent injection** via `Inject<T>`
- ⚙️ **Auto-published operations** using `#[operation]`
- 🧪 **Logic testable** without network/server
- 🌐 **Unified runtime**: one codebase for web, CLI, desktop...
- 🚀 **Zero overhead** when invoking agents directly

> Just `use axor::prelude::*` to get started.

---

## 🧩 Crates

| Crate        | Role                                                   |
|--------------|--------------------------------------------------------|
| `axor`       | Core: agents, DI, RPC operations, context              |
| `axor-web`   | (coming soon) HTTP runtime powered by Axum                          |
| `axor-tauri` | (coming soon) Desktop runtime for Tauri apps          |
| `axor-cli`   | (coming soon) CLI runtime: map agents to commands     |
| `axor-doc`   | (coming soon) Auto-generated docs + OpenAPI manifest  |

---

## 🚀 Quick Start

```rust
use axor::prelude::*;

#[agent]
struct HelloAgent;

#[agent_impl]
impl HelloAgent {
    #[operation]
    fn hello(&self) -> &'static str {
        "Hello, world!"
    }
}

fn main() {
    let mut context = AxorContext::default();
    context.register(HelloAgent);
    
    axor_web::serve(context); // Serve your agents via HTTP (if axor-web is used)
}
````

---

## 📋 Comparison

| Framework       | Typed DI | Auto Routing | Auto Ops | Web Ready | Modular |
| --------------- | -------- | ------------ | -------- | --------- | ------- |
| **Axor**        | ✅        | ✅            | ✅        | ✅         | ✅       |
| Axum            | ❌        | ❌            | Handlers | ✅         | ✅       |
| Actix Web       | ❌        | ❌            | Traits   | ✅         | ✅       |

---

## 📌 Roadmap

* ⏳ HTTP support via `axor-web`
* ⏳ Tauri support (`axor-tauri`)
* ⏳ CLI runtime (`axor-cli`)
* ⏳ Documentation + OpenAPI via `axor-doc`
* ⏳ Built-in agents: auth, metrics, async tasks

---

## ⚠️ Limitations

* Operations must return `Serialize` types and accept at most **one input** (must be `Deserialize`)
* No native reflection: macros are required for agent/operation declaration
* No dynamic documentation yet (coming via `axor-doc`)
* Multithreaded agents require proper `Arc` usage

---

## 📄 License

MIT © [Axor Contributors](https://github.com/kinai-io/axor)

---

> 💬 Feedback or ideas? [Open an issue](https://github.com/kinai-io/axor/axor/issues) or start a discussion — Axor is built for and with developers.