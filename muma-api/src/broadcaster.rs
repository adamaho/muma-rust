use bytes::{BufMut, Bytes, BytesMut};
use futures::{future, Stream};
use serde::{Deserialize, Serialize};
use serde_json;
use std::convert::Infallible;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageQuery {
    pub message: String,
}

impl MessageQuery {
    fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::new();
        let message = serde_json::to_string(&self).unwrap();
        buf.put_slice(message.as_bytes());
        buf.put_u8(b'\n');
        buf.freeze()
    }
}

#[derive(Debug, Default, Clone)]
struct BroadcasterInner {
    clients: Vec<Sender<MessageQuery>>,
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

    pub async fn new_client(&self) -> ChannelStream {
        let (tx, rx): (Sender<MessageQuery>, Receiver<MessageQuery>) = mpsc::channel(10);

        match tx
            .send(MessageQuery {
                message: String::from("Connected"),
            })
            .await
        {
            Ok(_) => {
                self.inner.lock().unwrap().clients.push(tx);
            }
            Err(_) => println!("An error occured"),
        };

        ChannelStream(rx)
    }

    pub async fn publish(&self, query: &MessageQuery) {
        let clients = self.inner.lock().unwrap().clients.clone();

        let send_futures = clients.iter().map(|client| client.send(query.clone()));

        future::join_all(send_futures).await;
    }
}

pub struct ChannelStream(Receiver<MessageQuery>);

impl Stream for ChannelStream {
    type Item = Result<Bytes, Infallible>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.poll_recv(cx).map(|ev| ev.map(|m| Ok(m.to_bytes())))
    }
}
