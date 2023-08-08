use crate::stream;
use parking_lot::Mutex;

/// alright things that are left todo
///
/// 1. Create a custom responder that responds with the stream response or a normal response based on a header
///
///
/// let realtime = Realtime::new();
///
/// Publish to a stream:
/// ```
/// realtime.publish()
/// ```
///
/// Subsribe to a stream:
///
/// ```
/// realtime.subscribe().await
/// ```
///

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

impl Realtime {
    pub async fn subscribe(&self) -> stream::Stream {
        let (tx, response) = stream::Stream::channel(10).await;
        self.inner.lock().unwrap().clients.push(tx);
        response
    }
}

#[derive(Default)]
struct RealtimeInner {
    clients: Vec<stream::Sender>,
}
