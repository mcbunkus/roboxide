pub mod pubsub;

pub const XPUB_PORT: &'static str = "10240";
pub const XSUB_PORT: &'static str = "10241";

pub const PUB_PROXY_EP: &'static str = "tcp://localhost:10241";
pub const SUB_PROXY_EP: &'static str = "tcp://localhost:10240";

/// Convenience function for creating and binding sockets
pub fn create_and_bind_socket(
    zmq_ctx: &zmq::Context,
    socket_type: zmq::SocketType,
    endpoint: &str,
) -> Result<zmq::Socket, zmq::Error> {
    let socket = zmq_ctx.socket(socket_type)?;
    socket.bind(endpoint)?;
    Ok(socket)
}

/// Convenience function for creating and connecting to already bound sockets
pub fn create_and_connect_socket(
    zmq_ctx: &zmq::Context,
    socket_type: zmq::SocketType,
    endpoint: &str,
) -> Result<zmq::Socket, zmq::Error> {
    let socket = zmq_ctx.socket(socket_type)?;
    socket.connect(endpoint)?;
    Ok(socket)
}

pub fn start_pubsub_proxy() -> Result<(), zmq::Error> {
    let frontend_endpoint: String = format!("tcp://*:{}", XSUB_PORT);
    let backend_endpoint: String = format!("tcp://*:{}", XPUB_PORT);

    let context = zmq::Context::new();
    let frontend = create_and_bind_socket(&context, zmq::XSUB, frontend_endpoint.as_str())?;
    let backend = create_and_bind_socket(&context, zmq::XPUB, backend_endpoint.as_str())?;

    zmq::proxy(&frontend, &backend)?;
    Ok(())
}
