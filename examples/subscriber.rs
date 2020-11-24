mod msgs;
use roboxide::pubsub::Subscriber;

// This function will be called whenever the subscriber gets a message from the publisher.
// A callback function requires the message as an argument.
fn callback(msg: msgs::Message) {
    println!("received msg -> {}", msg.data);
}

fn main() -> Result<(), zmq::Error> {
    let subscriber = Subscriber::<msgs::Message>::new("/hello/world", callback)?;

    // subscriber.run() actually spawns a thread that listens to the topic and calls the callback function
    // when it gets a message. It returns the thread's handle, so you can use the thread handle directly...
    let _subhandle = subscriber.run()?;

    loop {
        // ...or you can do work in here. Make sure you either have a loop like this, or call
        // subhandle.join(), or else the main thread will end and the subscriber will no
        // longer receive messages.
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
