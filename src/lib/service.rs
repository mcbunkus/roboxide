use super::socket;
use super::RbxMessage;
use std::thread;

fn service_proxy() -> Result<(), zmq::Error> {
    let frontend_endpoint = format!("tcp://*:{}", super::SERV_FRONTEND_PORT);
    let backend_endpoint = format!("tcp://*:{}", super::SERV_BACKEND_PORT);
    let context = zmq::Context::new();
    let frontend = socket::create_and_bind_socket(&context, zmq::ROUTER, &frontend_endpoint)?;
    let backend = socket::create_and_bind_socket(&context, zmq::ROUTER, &backend_endpoint)?;

    loop {
        let mut items = [
            frontend.as_poll_item(zmq::POLLIN),
            backend.as_poll_item(zmq::POLLIN),
        ];
        zmq::poll(&mut items, -1).unwrap();

        if items[0].is_readable() {
            loop {
                let message = frontend.recv_msg(0).unwrap();
                let more = message.get_more();
                backend
                    .send(message, if more { zmq::SNDMORE } else { 0 })
                    .unwrap();
                if !more {
                    break;
                }
            }
        }
        if items[1].is_readable() {
            loop {
                let message = backend.recv_msg(0).unwrap();
                let more = message.get_more();
                frontend
                    .send(message, if more { zmq::SNDMORE } else { 0 })
                    .unwrap();
                if !more {
                    break;
                }
            }
        }
    }
}

pub struct Service<T>
where
    T: RbxMessage,
{
    socket: zmq::Socket,
    name: &'static str,
    callback: fn(T),
}

impl<T> Service<T>
where
    T: RbxMessage,
{
    fn init(
        endpoint: &'static str,
        name: &'static str,
        callback: fn(T),
    ) -> Result<Service<T>, zmq::Error> {
        let context = zmq::Context::new();
        let socket = socket::create_and_connect_socket(&context, zmq::REP, endpoint)?;

        std::thread::spawn(service_proxy);

        Ok(Service {
            socket,
            name,
            callback,
        })
    }

    pub fn run(self) -> thread::JoinHandle<()> {
        thread::spawn(move || loop {
            let recv_bytes = self
                .socket
                .recv_bytes(0)
                .expect("failed to read msg as bytes");
            let msg = bincode::deserialize(&recv_bytes[..]).expect("failed to deserialize msg");
            (self.callback)(msg);
        })
    }
}

pub struct Client<T>
where
    T: RbxMessage,
{
    socket: zmq::Socket,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Client<T>
where
    T: RbxMessage,
{
    fn init(endpoint: &'static str) -> Result<Client<T>, zmq::Error> {
        let context = zmq::Context::new();
        let socket = super::socket::create_and_bind_socket(&context, zmq::REQ, endpoint)?;
        Ok(Client {
            socket,
            _phantom: std::marker::PhantomData,
        })
    }

    pub fn call(&self, req: T) -> Result<T, zmq::Error> {
        let req_as_bytes = bincode::serialize(&req).expect("failed to serialize request message");
        self.socket.send(req_as_bytes, 0)?;
        let response = self
            .socket
            .recv_bytes(0)
            .expect("failed to get response message");

        let encoded_resp =
            bincode::deserialize(&response[..]).expect("failed to deserialize response");
        Ok(encoded_resp)
    }
}
