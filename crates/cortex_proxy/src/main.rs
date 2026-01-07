use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Cortex Shield listening on 3000");
    println!("Forwards traffic to NestJS (Brain) on 3001");

    loop {
        let (mut client_socket, client_addr) = listener.accept().await?;
        println!("Client connected: {}", client_addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // 유저가 보낸 데이터 읽기
            let n = match client_socket.read(&mut buf).await {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(_) => return,
            };
            let user_msg = String::from_utf8_lossy(&buf[0..n]).to_string();
            let clean_msg = user_msg.trim(); // 개행문자 제거

            println!("From Client: {}", clean_msg);

            // NestJS (3001)로 연결 시도
            match TcpStream::connect("127.0.0.1:3001").await {
                Ok(mut nest_socket) => {

                    // NestJS 프로토콜에 맞춰 JSON 포장
                    let payload = json!({
                        "pattern": "ping",
                        "data": clean_msg,
                        "id": "1"
                    });

                    let json_str = payload.to_string();

                    let framed_msg = format!("{}#{}", json_str.len(), json_str);

                    println!("Sending to NestJS: {}", framed_msg); // 디버깅용 로그

                    // 전송
                    nest_socket.write_all(framed_msg.as_bytes()).await.unwrap();

                    // NestJS 응답 기다리기
                    let mut nest_buf = [0; 1024];
                    match nest_socket.read(&mut nest_buf).await {
                        Ok(n) => {
                            let raw_response = String::from_utf8_lossy(&nest_buf[0..n]);
                            println!("From NestJS (Raw): {}", raw_response);

                            // 깔끔하게 보여주기 위해 '#' 뒤에 있는 JSON만 파싱 (약식)
                            let response_body = if let Some(idx) = raw_response.find('#') {
                                &raw_response[idx+1..]
                            } else {
                                &raw_response
                            };

                            let final_msg = format!("Proxy Received: {}", response_body);
                            client_socket.write_all(final_msg.as_bytes()).await.unwrap();
                        },
                        Err(e) => println!("Failed to read from NestJS: {}", e),
                    }
                }
                Err(e) => {
                    println!("Could not connect to NestJS (Is it running?): {}", e);
                    let _ = client_socket.write_all(b"Error: Brain is dead.\n").await;
                }
            }
        });
    }
}