// Spin up a proxy server so publishers and subscribers can connect to the same ports
fn main() -> Result<(), zmq::Error> {
    let frontend_endpoint: String = format!("tcp://*:{}", roboxide::XSUB_PORT);
    let backend_endpoint: String = format!("tcp://*:{}", roboxide::XPUB_PORT);

    let context = zmq::Context::new();
    let frontend =
        roboxide::create_and_bind_socket(&context, zmq::XSUB, frontend_endpoint.as_str())?;
    let backend = roboxide::create_and_bind_socket(&context, zmq::XPUB, backend_endpoint.as_str())?;

    zmq::proxy(&frontend, &backend).expect("failed to start proxy");
    Ok(())
}
