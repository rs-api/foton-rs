//! Response streaming support.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use rust_api::{Res, StreamBody};
//!
//! async fn stream_handler() -> Res {
//!     Res::stream(|mut tx| async move {
//!         tx.send("chunk 1\n").await.ok();
//!         tx.send("chunk 2\n").await.ok();
//!         tx.send("chunk 3\n").await.ok();
//!     })
//! }
//! ```

use bytes::Bytes;
use http_body_util::{BodyExt, StreamBody as HttpStreamBody};
use hyper::body::Frame;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use crate::{Error, Result};

type BoxBody = http_body_util::combinators::BoxBody<Bytes, Error>;

/// Channel sender for streaming response chunks.
pub struct StreamSender {
    tx: mpsc::Sender<Result<Bytes>>,
}

impl StreamSender {
    /// Send a chunk of data.
    pub async fn send(&mut self, data: impl Into<Bytes>) -> Result<()> {
        self.tx
            .send(Ok(data.into()))
            .await
            .map_err(|_| Error::Custom("Stream channel closed".into()))
    }

    /// Send an error to close the stream.
    pub async fn send_error(&mut self, error: Error) -> Result<()> {
        self.tx
            .send(Err(error))
            .await
            .map_err(|_| Error::Custom("Stream channel closed".into()))
    }
}

/// Create streaming response body.
pub fn create_stream<F, Fut>(f: F) -> BoxBody
where
    F: FnOnce(StreamSender) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let (tx, rx) = mpsc::channel::<Result<Bytes>>(100);

    let sender = StreamSender { tx };

    tokio::spawn(async move {
        f(sender).await;
    });

    let stream = ReceiverStream::new(rx).map_ok(Frame::data);
    let stream_body = HttpStreamBody::new(stream);

    stream_body.boxed()
}
