use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::Command,
};

#[tokio::test]
async fn initialize_lsp() {
    let mut server = Command::new(env!("CARGO_BIN_EXE_depend"))
        .arg("lsp")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to start LSP server");

    let mut stdin = server.stdin.take().unwrap();
    let stdout = server.stdout.take().unwrap();

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "processId": null,
            "capabilities": {},
            "rootUri": null
        }
    });

    let body = serde_json::to_vec(&request).unwrap();

    let message = format!("Content-Length: {}\r\n\r\n", body.len());

    stdin.write_all(message.as_bytes()).await.unwrap();
    stdin.write_all(&body).await.unwrap();
    stdin.flush().await.unwrap();

    let mut stdout = BufReader::new(stdout);

    let mut content_length = 0;

    loop {
        let mut line = String::new();
        stdout.read_line(&mut line).await.unwrap();

        if line == "\r\n" {
            break;
        }

        if let Some(value) = line.strip_prefix("Content-Length: ") {
            content_length = value.trim().parse().unwrap();
        }
    }

    let mut response = vec![0; content_length];
    stdout.read_exact(&mut response).await.unwrap();

    let response: serde_json::Value = serde_json::from_slice(&response).unwrap();

    println!("{}", serde_json::to_string_pretty(&response).unwrap());

    server.kill().await.unwrap();
}
