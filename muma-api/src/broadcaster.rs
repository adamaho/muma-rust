use crate::event::Message;
use bytes::Bytes;
use bytestring::ByteString;
use futures::{future, Stream};
use std::convert::Infallible;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Debug, Clone)]
struct BroadcastSender {
    tx: Sender<Message>,
}

impl BroadcastSender {
    pub async fn send(&self, message: impl Into<Message>) {
        let _ = self.tx.send(message.into()).await;
    }
}

#[derive(Debug, Default, Clone)]
struct BroadcasterInner {
    pub request_count: usize,
    clients: Vec<BroadcastSender>,
}

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

impl Broadcaster {
    pub fn new() -> Arc<Self> {
        Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        })
    }

    /// Creates a new stream for the connected client along with a channel to
    /// send messages to
    pub async fn new_client(&self) -> ChannelStream {
        let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel(100);
        let _ = tx.send(Message::new("Connected")).await;
        let _ = self
            .inner
            .lock()
            .unwrap()
            .clients
            .push(BroadcastSender { tx });
        ChannelStream(rx)
    }

    /// Publishes a message to all clients that are subscribed to the stream
    pub async fn publish(&self, _message: impl Into<Message> + Clone) {
        let mut inner = self.inner.lock().unwrap();
        inner.request_count += 1;
        let m = format!("{} - {}\n", "Hello world", inner.request_count);
        let clients = inner.clients.clone();
        let send_futures = clients
            .iter()
            .map(|client| client.send(Message::new(m.as_str())));
        future::join_all(send_futures).await;
    }
}

pub struct ChannelStream(Receiver<Message>);

impl Stream for ChannelStream {
    type Item = Result<Bytes, Infallible>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0
            .poll_recv(cx)
            .map(|ev| ev.map(|m| Ok(m.data.into_bytes())))
    }
}
