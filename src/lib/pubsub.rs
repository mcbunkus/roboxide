use serde::Serialize;
use std::thread;

/// Publisher can take any struct that implements serde's Serialize and Deserialize traits and publish them over ZMQ
/// sockets.
pub struct Publisher<T>
where
    T: Serialize + serde::de::DeserializeOwned + 'static,
{
    socket: zmq::Socket,
    topic: &'static str,

    // needed to let the user specify the type of struct they wish to publish when constructing a new publisher. This is
    // meant for clarity when writing nodes, so _phantom is not needed elsewhere by Publisher
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Publisher<T>
where
    T: Serialize + serde::de::DeserializeOwned + 'static,
{
    /// Create a new publisher with a topic. Returns a result with a publisher that's ready to go or an error.
    pub fn new(topic: &'static str) -> Result<Publisher<T>, zmq::Error> {
        let context = zmq::Context::new();
        let socket = super::create_and_connect_socket(&context, zmq::PUB, super::PUB_PROXY_EP)?;
        let _phantom = std::marker::PhantomData;

        Ok(Publisher {
            socket,
            topic,
            _phantom,
        })
    }

    /// Takes a message and encodes it before sending it on the socket
    pub fn publish(&self, message: T) -> Result<(), zmq::Error> {
        let encoded_msg = match bincode::serialize(&message) {
            Ok(enc) => enc,
            Err(_) => return Err(zmq::Error::EINVAL),
        };
        self.socket.send(self.topic, zmq::SNDMORE)?;
        self.socket.send(encoded_msg, 0)?;
        Ok(())
    }
}

/// Subscriber is functionally similar to a ROS subscriber. It works by taking a topic and a callback function. Calling
/// run on a subscriber instance will spawn a thread that listens to the socket, and calls the callback function when it
/// receives a message. The callback function is required to take the message as an argument.
pub struct Subscriber<T>
where
    T: Serialize + serde::de::DeserializeOwned + 'static,
{
    topic: &'static str,
    callbackfn: fn(T),
    socket: zmq::Socket,
}

impl<'a, T> Subscriber<T>
where
    T: Serialize + serde::de::DeserializeOwned + 'static,
{
    pub fn new(topic: &'static str, callbackfn: fn(T)) -> Result<Subscriber<T>, zmq::Error> {
        let context = zmq::Context::new();
        let socket = super::create_and_connect_socket(&context, zmq::SUB, super::SUB_PROXY_EP)?;
        socket.set_subscribe((&topic).as_bytes())?;
        Ok(Subscriber {
            topic,
            callbackfn,
            socket,
        })
    }

    // Spawn a thread that listens to the socket and calls the callback function on received messages. The receive
    // methods on zmq sockets are blocking, so it will wait at each recv call until it receives the message.
    pub fn run(self) -> Result<thread::JoinHandle<()>, zmq::Error> {
        Ok(thread::spawn(move || loop {
            let recv_topic = self
                .socket
                .recv_string(0)
                .expect("failed to read topic")
                .expect("failed to parse topic");

            if recv_topic != self.topic {
                continue;
            }

            let bytes = self
                .socket
                .recv_bytes(0)
                .expect("failed to read msg as bytes");
            let msg = bincode::deserialize(&bytes[..]).expect("failed to deserialize msg");
            (self.callbackfn)(msg);
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Dummy {
        data: u8,
    }

    #[test]
    fn new_publisher_test() {
        let publisher = Publisher::<Dummy>::new("/dummy/topic");
        assert!(
            publisher.is_ok(),
            format!("failed to create a publisher: {:#?}", publisher.err())
        );
    }

    #[test]
    fn new_subscriber_test() {
        let subscriber = Subscriber::<Dummy>::new("/dummy/topic", |_dummy: Dummy| {});
        assert!(
            subscriber.is_ok(),
            format!("failed to create a subscriber: {:#?}", subscriber.err())
        );
    }
}
