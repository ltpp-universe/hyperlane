use color_output::*;

#[allow(dead_code)]
fn output(title: &str, msg: &str, color: Color) {
    OutputBuilder::new()
        .show_time(true)
        .time_bg_color(ColorType::Use(Color::Cyan))
        .time_text_blod(true)
        .text(title)
        .text_blod(true)
        .text_bg_color(ColorType::Use(Color::Red))
        .endl(true)
        .build()
        .output();
    OutputBuilder::new()
        .show_time(false)
        .text(msg)
        .text_bg_color(ColorType::Use(color.clone()))
        .text_blod(true)
        .endl(true)
        .build()
        .output();
}

#[test]
fn test_server_basic_usage() {
    use crate::*;
    let mut server: Server = Server::new();
    server.host("0.0.0.0");
    server.port(80);
    server.middleware(|controller_data| {
        let request: Request = controller_data.request();
        output("Request", &format!("{:#?}", request), Color::Yellow);
    });
    server.router("/", |controller_data| {
        let mut response: Response = controller_data.response.clone();
        let body: Vec<u8> = "404 Not Found".as_bytes().to_vec();
        let stream: std::sync::Arc<std::net::TcpStream> = controller_data.stream();
        let res: Result<(), ResponseError> = response
            .body(body)
            .status_code(404)
            .header("server", "hyperlane")
            .send(&stream);
        output("Response", &format!("{:#?}", res), Color::Green);
    });
    server.router("/hello", |controller_data| {
        let mut response: Response = controller_data.response.clone();
        let body: Vec<u8> = "hello world!".as_bytes().to_vec();
        let stream: std::sync::Arc<std::net::TcpStream> = controller_data.stream();
        let res: Result<(), ResponseError> = response
            .body(body)
            .status_code(200)
            .header("server", "hyperlane")
            .send(&stream);
        output("Response", &format!("{:#?}", res), Color::Green);
    });
    server.listen();
}

#[test]
fn test_server_with_chained_methods() {
    use crate::*;
    Server::new()
        .host("0.0.0.0")
        .port(80)
        .middleware(|controller_data| {
            let request: Request = controller_data.request();
            output("Request", &format!("{:#?}", request), Color::Yellow);
        })
        .router("/", |controller_data| {
            let mut response: Response = controller_data.response.clone();
            let body: Vec<u8> = "404 Not Found".as_bytes().to_vec();
            let stream: std::sync::Arc<std::net::TcpStream> = controller_data.stream();
            let res: Result<(), ResponseError> = response
                .body(body)
                .status_code(404)
                .header("server", "hyperlane")
                .send(&stream);
            output("Response", &format!("{:#?}", res), Color::Green);
        })
        .router("/hello", |controller_data| {
            let mut response: Response = controller_data.response.clone();
            let body: Vec<u8> = "hello world!".as_bytes().to_vec();
            let stream: std::sync::Arc<std::net::TcpStream> = controller_data.stream();
            let res: Result<(), ResponseError> = response
                .body(body)
                .status_code(200)
                .header("server", "hyperlane")
                .send(&stream);
            output("Response", &format!("{:#?}", res), Color::Green);
        })
        .listen();
}
