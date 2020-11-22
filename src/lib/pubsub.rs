use serde::Serialize;
use std::thread;

pub struct Publisher<T>
where
    T: Serialize,
{
    socket: zmq::Socket,
    topic: &'static str,
    _phantom: std::marker::PhantomData<T>, // needed to let the user specify the type of struct they wish to publish
}

impl<T> Publisher<T>
where
    T: Serialize,
{
    // Create a new publisher with a topic. Returns a result with a publisher that's ready to go or an error.
    pub fn new(topic: &'static str) -> Result<Publisher<T>, zmq::Error> {
        let context = zmq::Context::new();
        let socket = context.socket(zmq::PUB)?;
        socket.connect(super::PUB_PROXY_EP)?;
        let _phantom = std::marker::PhantomData;

        Ok(Publisher {
            socket,
            topic,
            _phantom,
        })
    }

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

pub struct Subscriber<T>
where
    T: serde::de::DeserializeOwned + 'static,
{
    topic: &'static str,
    callbackfn: fn(T),
}

impl<'a, T> Subscriber<T>
where
    T: serde::de::DeserializeOwned + 'static,
{
    pub fn new(topic: &'static str, callbackfn: fn(T)) -> Result<Subscriber<T>, zmq::Error> {
        Ok(Subscriber { topic, callbackfn })
    }

    pub fn run(&self) -> Result<thread::JoinHandle<()>, zmq::Error> {
        let topic = self.topic;
        let func = self.callbackfn;
        let ctx = zmq::Context::new();
        let socket = ctx.socket(zmq::SUB)?;
        socket.set_subscribe((&topic).as_bytes())?;
        socket.connect(super::SUB_PROXY_EP)?;
        Ok(thread::spawn(move || loop {
            let recv_topic = socket
                .recv_string(0)
                .expect("failed to read topic")
                .expect("failed to do something");

            if recv_topic != topic {
                continue;
            }

            let bytes = socket.recv_bytes(0).expect("failed to read msg as bytes");
            let msg = bincode::deserialize(&bytes[..]).expect("failed to deserialize msg");
            func(msg);
        }))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pub_sub_test() {}
}
