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
