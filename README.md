# Axor

**Axor** is a modular Rust framework for backend development, focused on structuring business logic through injectable, self-contained agents. It enables instant publication across multiple environments — from HTTP to CLI and desktop — without code duplication.

## Vision

> One core logic, exposed anywhere — without rewriting it.

Axor empowers developers to build backends around **typed agents**, **self-publishing operations**, and a **composable runtime model**. With minimal boilerplate and full testability, Axor helps you scale your app — not your complexity.

## Core Features

- ✅ Typed dependency injection with a central `AxorContext`
- ✅ Composable, self-contained business agents (`#[agent]`)
- ✅ Auto-registered operations (`#[operation]`)
- ✅ HTTP-ready with `axor-web`, powered by Axum
- ✅ Multi-runtime support (web, CLI, Tauri...)
- ✅ Testable without a server, thanks to agent isolation

## Crates

| Crate        | Role                                                   |
|--------------|--------------------------------------------------------|
| `axor`       | Core framework: agent system, DI, operations           |
| `axor-web`   | HTTP runtime based on Axum                            |
| `axor-tauri` | (coming soon) Tauri runtime for desktop apps          |
| `axor-cli`   | (coming soon) CLI runtime: turn agents into commands  |
| `axor-doc`   | (coming soon) Auto-generated docs + OpenAPI manifest  |

## Quick Example

```rust
#[agent]
pub struct UserService;

#[operation(GET, "/user/:id")]
fn get_user(&self, id: String) -> Result<User> {
    // Your business logic here
}

fn main() {
    let mut context = AxorContext::default();
    context.register(UserService);

    axor_web::serve(context);
}
````

## Comparison

| Framework       | Typed DI | Auto Routing | Auto Op Export  | Web Ready | Modular |
| --------------- | -------- | ------------ | --------------- | --------- | ------- |
| **Axor**        | ✅        | ✅            | ✅               | ✅         | ✅       |
| Axum            | ❌        | ❌            | Handler-based   | ✅         | ✅       |
| Actix Web       | ❌        | ❌            | Trait-based     | ✅         | ✅       |
| Shuttle Service | ✅        | ❌            | ❌               | ✅         | ❌       |
| async-graphql   | ✅        | ✅ (GQL)      | ✅ (`#[Object]`) | ❌         | ✅       |

## Roadmap

* ✅ HTTP runtime via `axor-web`
* ⏳ Tauri runtime (`axor-tauri`)
* ⏳ CLI runtime (`axor-cli`)
* ⏳ Documentation & OpenAPI via `axor-doc`
* ⏳ Built-in auth, metrics, and async support as standard agents

## License

MIT © Axor Contributors

