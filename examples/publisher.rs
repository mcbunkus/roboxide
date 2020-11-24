mod msgs;
use roboxide::pubsub::Publisher;

// Publishers work the same way as they do in ROS. Returning Result<(), zmq::Error> is not required,
// but doing so allows you to use the ? operator on Results returned by roboxide functions.
fn main() -> Result<(), zmq::Error> {
    let publisher = Publisher::<msgs::Message>::new("/hello/world")?;
    let mut counter: u128 = 0;
    loop {
        println!("publishing data -> {}", counter);
        let data = msgs::Message { data: counter };
        publisher.publish(data)?;
        counter += 1;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
