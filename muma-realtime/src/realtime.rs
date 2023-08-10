use crate::stream;
use parking_lot::Mutex;
use serde::Serialize;

/// A thread-safe implementation of a stream that can take supported
/// data types and stream them to a series of connected clients.
pub struct Realtime {
    inner: Mutex<RealtimeInner>,
}

impl Realtime {
    /// Creates a new instance of Realtime
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(RealtimeInner::default()),
        }
    }
}

#[derive(Default)]
struct RealtimeInner {
    clients: Vec<stream::Sender>,
}

impl Realtime {
    /// Create a subscription responder for actix
    pub async fn subscribe(&self, buffer: usize) -> stream::JsonPatchStream {
        let (tx, response) = stream::JsonPatchStream::channel(buffer);
        self.inner.lock().clients.push(tx);
        response
    }

    /// Sends a plain text message to the channel
    pub async fn publish(&self, msg: &str) {
        let clients = self.inner.lock().clients.clone();
        let send_futures = clients.iter().map(|c| c.send(stream::Payload::new(msg)));
        futures::future::join_all(send_futures).await;
    }

    /// Sends a json message to the channel
    pub async fn publish_json(&self, msg: impl Serialize + Clone) {
        let inner = self.inner.lock();
        let send_futures = inner
            .clients
            .iter()
            .map(|c| c.send(stream::Payload::new_json(msg.clone()).unwrap()));
        futures::future::join_all(send_futures).await;
    }
}
