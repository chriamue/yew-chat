# yew-chat
Chat component written in rust yew with axum router backend

## Architecture

Below is a C4 diagram representing the architecture of the yew-chat application:

![C4 Diagram for yew-chat](https://www.plantuml.com/plantuml/proxy?src=https://raw.githubusercontent.com/chriamue/yew-chat/main/yew_chat.puml)

## Features

- Real-time chat functionality
- Built with Rust and Yew framework
- WebAssembly for client-side performance
- In-memory message queue for temporary storage

## Getting Started

### Running the Application

```bash
trunk serve
```

### Running the Backend

```bash
cargo run --bin yew_chat_server --features server
```

### Running the Example

```bash
cargo run --example request_app --features yew
```