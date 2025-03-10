<center>

## hyperlane

<img src="./img/logo.png" alt="" height="160">

[![](https://img.shields.io/crates/v/hyperlane.svg)](https://crates.io/crates/hyperlane)
[![](https://img.shields.io/crates/d/hyperlane.svg)](https://img.shields.io/crates/d/hyperlane.svg)
[![](https://docs.rs/hyperlane/badge.svg)](https://docs.rs/hyperlane)
[![](https://github.com/ltpp-universe/hyperlane/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/hyperlane/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/hyperlane.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/hyperlane/)

[Api Docs](https://docs.rs/hyperlane/latest/hyperlane/)

> hyperlane is a lightweight and high-performance Rust HTTP server library designed to simplify network service development. It supports HTTP request parsing, response building, TCP communication, and redirection features, making it ideal for building modern web services.

## Installation

To use this crate, you can run cmd:

```shell
cargo add hyperlane
```

## Quick start

- [hyperlane-quick-start git](https://github.com/ltpp-universe/hyperlane-quick-start)
- [hyperlane-quick-start docs](https://docs.ltpp.vip/hyperlane/quick-start/)

```sh
git clone https://github.com/ltpp-universe/hyperlane-quick-start.git
```

## Use

```rust
use hyperlane::*;

async fn request_middleware(controller_data: ControllerData) {
    let socket_addr: String = controller_data
        .get_socket_addr()
        .await
        .unwrap_or(DEFAULT_SOCKET_ADDR)
        .to_string();
    controller_data
        .set_response_header(SERVER, "hyperlane")
        .await
        .set_response_header(CONNECTION, CONNECTION_KEEP_ALIVE)
        .await
        .set_response_header("SocketAddr", socket_addr)
        .await;
}

async fn response_middleware(controller_data: ControllerData) {
    let request: String = controller_data.get_request().await.to_string();
    let response: String = controller_data.get_response().await.to_string();
    controller_data
        .log_info(format!("Request => {}", request), log_handler)
        .await
        .log_info(format!("Response => {}", response), log_handler)
        .await;
}

async fn root_router(controller_data: ControllerData) {
    let _ = controller_data
        .send_response(200, "hello hyperlane => /")
        .await;
}

async fn websocket_router(controller_data: ControllerData) {
    let request_body: Vec<u8> = controller_data.get_request_body().await;
    let _ = controller_data.send_response_body(request_body).await;
}

async fn run_server() {
    let mut server: Server = Server::new();
    server.host("0.0.0.0").await;
    server.port(60000).await;
    server.log_dir("./logs").await;
    server.log_size(100_024_000).await;
    server.log_interval_millis(1000).await;
    server.websocket_buffer_size(4096).await;
    server.request_middleware(request_middleware).await;
    server.response_middleware(response_middleware).await;
    server.router("/", root_router).await;
    server.router("/websocket", websocket_router).await;
    let test_string: String = "hello hyperlane".to_owned();
    server
        .router(
            "/test/panic",
            async_func!(test_string, |data| {
                println_success!(test_string);
                println_success!(format!("{:?}", data));
                panic!("test panic");
            }),
        )
        .await;
    server.listen().await;
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
