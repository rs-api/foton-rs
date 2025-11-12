#![deny(warnings)]

use bytes::Bytes;
use http_body_util::{BodyExt, Empty, Full};
use hyper::body::Incoming;
use hyper::{Request, Response, Uri};
use hyper_util::rt::TokioIo;
use std::time::Duration;
use tokio::net::TcpStream;

#[cfg(feature = "https")]
use tokio_native_tls::TlsConnector;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// HTTP client.
pub struct Client {
    timeout: Option<Duration>,
}

impl Client {
    /// Create new client.
    pub fn new() -> Self {
        Self {
            timeout: Some(Duration::from_secs(30)),
        }
    }

    pub fn with_timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    pub fn with_no_timeout(mut self) -> Self {
        self.timeout = None;
        self
    }

    pub fn set_timeout(&mut self, duration: Duration) {
        self.timeout = Some(duration);
    }

    pub fn disable_timeout(&mut self) {
        self.timeout = None;
    }

    pub async fn get(&self, url: &str) -> Result<Response<Incoming>> {
        let uri: Uri = url.parse()?;
        let req = Request::get(uri).body(Empty::<Bytes>::new())?;
        self.execute(req).await
    }

    pub async fn post(&self, url: &str, body: impl Into<Bytes>) -> Result<Response<Incoming>> {
        let uri: Uri = url.parse()?;
        let req = Request::post(uri).body(Full::new(body.into()))?;
        self.execute(req).await
    }

    pub async fn put(&self, url: &str, body: impl Into<Bytes>) -> Result<Response<Incoming>> {
        let uri: Uri = url.parse()?;
        let req = Request::put(uri).body(Full::new(body.into()))?;
        self.execute(req).await
    }

    pub async fn delete(&self, url: &str) -> Result<Response<Incoming>> {
        let uri: Uri = url.parse()?;
        let req = Request::delete(uri).body(Empty::<Bytes>::new())?;
        self.execute(req).await
    }

    pub async fn patch(&self, url: &str, body: impl Into<Bytes>) -> Result<Response<Incoming>> {
        let uri: Uri = url.parse()?;
        let req = Request::patch(uri).body(Full::new(body.into()))?;
        self.execute(req).await
    }

    async fn execute<B>(&self, req: Request<B>) -> Result<Response<Incoming>>
    where
        B: hyper::body::Body + Send + 'static,
        B::Data: Send,
        B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let fut = self.send_request(req);

        match self.timeout {
            Some(duration) => tokio::time::timeout(duration, fut)
                .await
                .map_err(|_| "Request timeout")?,
            None => fut.await,
        }
    }

    async fn send_request<B>(&self, req: Request<B>) -> Result<Response<Incoming>>
    where
        B: hyper::body::Body + Send + 'static,
        B::Data: Send,
        B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let uri = req.uri().clone();
        let host = uri.host().ok_or("URI has no host")?;
        let scheme = uri.scheme_str().unwrap_or("http");

        match scheme {
            "http" => {
                self.send_http(req, host, uri.port_u16().unwrap_or(80))
                    .await
            }
            #[cfg(feature = "https")]
            "https" => {
                self.send_https(req, host, uri.port_u16().unwrap_or(443))
                    .await
            }
            #[cfg(not(feature = "https"))]
            "https" => Err("HTTPS support not enabled. Enable the 'https' feature.".into()),
            _ => Err(format!("Unsupported scheme: {}", scheme).into()),
        }
    }

    async fn send_http<B>(
        &self,
        req: Request<B>,
        host: &str,
        port: u16,
    ) -> Result<Response<Incoming>>
    where
        B: hyper::body::Body + Send + 'static,
        B::Data: Send,
        B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(addr).await?;
        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

        tokio::task::spawn(async move {
            if let Err(_err) = conn.await {
                // Connection closed
            }
        });

        let res = sender.send_request(req).await?;
        Ok(res)
    }

    #[cfg(feature = "https")]
    async fn send_https<B>(
        &self,
        req: Request<B>,
        host: &str,
        port: u16,
    ) -> Result<Response<Incoming>>
    where
        B: hyper::body::Body + Send + 'static,
        B::Data: Send,
        B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(addr).await?;

        let cx = native_tls::TlsConnector::builder().build()?;
        let cx = TlsConnector::from(cx);
        let tls_stream = cx.connect(host, stream).await?;
        let io = TokioIo::new(tls_stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

        tokio::task::spawn(async move {
            if let Err(_err) = conn.await {
                // Connection closed
            }
        });

        let res = sender.send_request(req).await?;
        Ok(res)
    }

    pub async fn body_bytes(res: Response<Incoming>) -> Result<Bytes> {
        let body = res.collect().await?.to_bytes();
        Ok(body)
    }

    pub async fn body_text(res: Response<Incoming>) -> Result<String> {
        let bytes = Self::body_bytes(res).await?;
        Ok(String::from_utf8(bytes.to_vec())?)
    }

    #[cfg(feature = "json")]
    pub async fn post_json<T: serde::Serialize>(
        &self,
        url: &str,
        data: &T,
    ) -> Result<Response<Incoming>> {
        let uri: Uri = url.parse()?;
        let body = serde_json::to_vec(data)?;

        let req = Request::post(uri)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(Full::new(Bytes::from(body)))?;

        self.execute(req).await
    }

    #[cfg(feature = "json")]
    pub async fn put_json<T: serde::Serialize>(
        &self,
        url: &str,
        data: &T,
    ) -> Result<Response<Incoming>> {
        let uri: Uri = url.parse()?;
        let body = serde_json::to_vec(data)?;

        let req = Request::put(uri)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(Full::new(Bytes::from(body)))?;

        self.execute(req).await
    }

    #[cfg(feature = "json")]
    pub async fn patch_json<T: serde::Serialize>(
        &self,
        url: &str,
        data: &T,
    ) -> Result<Response<Incoming>> {
        let uri: Uri = url.parse()?;
        let body = serde_json::to_vec(data)?;

        let req = Request::patch(uri)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(Full::new(Bytes::from(body)))?;

        self.execute(req).await
    }

    #[cfg(feature = "json")]
    pub async fn body_json<T: serde::de::DeserializeOwned>(res: Response<Incoming>) -> Result<T> {
        let bytes = Self::body_bytes(res).await?;
        let data = serde_json::from_slice(&bytes)?;
        Ok(data)
    }

    #[cfg(feature = "json")]
    pub async fn get_json<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T> {
        let res = self.get(url).await?;
        Self::body_json(res).await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
