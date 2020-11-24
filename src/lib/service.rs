fn service_proxy() -> Result<(), zmq::Error> {
    let context = zmq::Context::new();
    let frontend = super::create_and_bind_socket(&context, zmq::ROUTER, "tcp://*:10422")?;
    let backend = super::create_and_bind_socket(&context, zmq::ROUTER, "tcp://*:10423")?;

    zmq::poll(
        &mut [
            frontend.as_poll_item(zmq::POLLIN),
            backend.as_poll_item(zmq::POLLIN),
        ],
        3,
    )?;
    Ok(())
}
