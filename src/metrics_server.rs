use http::{Request, Response};
use http_body_util::Full;
use hyper::{body, body::Bytes, server::conn::http1, service::service_fn};
use std::{convert::Infallible, net::SocketAddr};
use tokio::net::TcpListener;

pub async fn start_metrics() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 62007).into();

    let tcp_listener = TcpListener::bind(addr).await?;
    loop {
        let (tcp_stream, _) = tcp_listener.accept().await?;
        tokio::task::spawn(async move {
            if let Err(http_err) = http1::Builder::new()
                .keep_alive(true)
                .serve_connection(tcp_stream, service_fn(hello))
                .await
            {
                eprintln!("Error while serving HTTP connection: {}", http_err);
            }
        });
    }
}

async fn hello(_req: Request<body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from(
        autometrics::encode_global_metrics().unwrap_or_default(),
    ))))
}
