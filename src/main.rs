use rust_course::ThreadPool;
use std::net::TcpListener; // 引入TCP标准库
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

// std::io::prelude 是一个模块，包含了许多对 I/O 有用的 trait。不同于 std::prelude，std::io::prelude 需要被显示 use。
use std::io::prelude::*;

fn main() {
    // 绑定监听器，设置端口8080
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let pool = ThreadPool::new(4);

    // 从监听器的incoming函数迭代器传入stream（流）数据
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down");
}

// handle_connection传入了 stream 流参数
fn handle_connection(mut stream: TcpStream) {
    // 创建了1mb的缓冲区
    let mut buffer = [0; 1024]; // 创建接受数据缓冲区

    // 将数据读取到缓冲区
    stream.read(&mut buffer).unwrap();

    // 打印接收到内容
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let index_page = b"GET / HTTP/1.1\r\n";
    let json_page = b"GET /json HTTP/1.1\r\n";

    // 默认 body 为 404
    let body = "{\"code\":404}";
    let mut response = format!(
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}",
        body.len(),
        body
    );

    if buffer.starts_with(index_page) {
        // 首页index page 的 body 
        let body = "{\"code\":200}";
        response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}",
            body.len(),
            body
        );
    } else if buffer.starts_with(json_page) {
        // 首页json page 的 body 
        let body = "{\"code\":200, \"message\":\"my is json\"}";
        response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}",
            body.len(),
            body
        );
    } else{
        thread::sleep(Duration::from_millis(50000));
    }
    // 返回主体
    // let body = "{\"code\":200}";

    // 返回http协议格式
    // let response = format!(
    //     "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n{}",
    //     body.len(),
    //     body
    // );
    stream.write(response.as_bytes()).unwrap();

    // flush也就是将数据推送出去
    stream.flush().unwrap();
}
