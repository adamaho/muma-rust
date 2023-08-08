use actix_web::body::{BodySize, MessageBody};
use bytes::{BufMut, Bytes, BytesMut};
use bytestring::ByteString;
use futures::stream::Stream;
use pin_project_lite::pin_project;
use serde::Serialize;
use std::convert::Infallible;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct Payload(ByteString);

impl Payload {
    /// For the given data, construct a new plain text message to send
    /// to the user.
    ///
    /// Example:
    ///
    /// ```
    /// let message = stream::Message::new("Hello World");
    /// ```
    pub fn new(data: impl Into<ByteString>) -> Self {
        Self(data.into())
    }
    /// For the given piece of data, construct a new json message to send
    /// to a user
    ///
    /// Example:
    ///
    /// ```
    /// #[derive(Serialize)]
    /// struct Foo {
    ///    hello: String
    /// }
    ///
    /// let message = event::Message::new_json(Foo { hello: String::from("world")});
    /// ```
    ///
    pub fn new_json(data: impl Serialize) -> anyhow::Result<Self, serde_json::Error> {
        Ok(Self(serde_json::to_string(&data)?.into()))
    }

    /// Converts the ByteString to Bytes for a stream
    pub fn into_bytes(&self) -> Bytes {
        let mut buf = BytesMut::new();
        buf.put_slice(self.0.as_bytes());
        buf.put_u8(b'\n');
        buf.freeze()
    }
}

/// A Channel implementation of a stream that accepts
/// `Payload`'s as messages in the channel
///
/// see the public interface [channel] for how to consume.
#[derive(Debug)]
pub struct ChannelStream(mpsc::Receiver<Payload>);

impl Stream for ChannelStream {
    type Item = Result<Payload, Infallible>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.poll_recv(cx).map(|ev| ev.map(|m| Ok(m)))
    }
}

#[derive(Clone)]
pub struct Sender {
    tx: mpsc::Sender<Payload>,
}

impl Sender {
    /// Creates a new instance of a sender
    pub fn new(tx: mpsc::Sender<Payload>) -> Self {
        Self { tx }
    }

    /// Converts the provided msg into a Payload and sends
    /// it to the channel when there is enough capacity for it
    pub async fn send(
        &self,
        msg: impl Into<Payload>,
    ) -> anyhow::Result<(), mpsc::error::SendError<Payload>> {
        self.tx.send(msg.into()).await
    }
}

pin_project! {
    /// Streams ndjson values for the desired messages
    pub struct NdJsonStream {
        #[pin]
        stream: ChannelStream,
    }
}

impl NdJsonStream {
    pub fn channel(buffer: usize) -> (Sender, Self) {
        let (tx, rx): (mpsc::Sender<Payload>, mpsc::Receiver<Payload>) = mpsc::channel(buffer);
        (
            Sender { tx },
            Self {
                stream: ChannelStream(rx),
            },
        )
    }
}

impl MessageBody for NdJsonStream {
    type Error = Box<dyn std::error::Error>;

    fn size(&self) -> BodySize {
        BodySize::Stream
    }

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        let this = self.project();

        if let Poll::Ready(msg) = this.stream.poll_next(cx) {
            return match msg {
                Some(Ok(msg)) => Poll::Ready(Some(Ok(msg.into_bytes()))),
                Some(Err(err)) => Poll::Ready(Some(Err(err.into()))),
                None => Poll::Ready(None),
            };
        }

        Poll::Pending
    }
}

// TODO: Implement responder with the correct headers
