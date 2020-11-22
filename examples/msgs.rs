// This is equivalent to a ROS message
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub data: u128,
}

#[allow(dead_code)]
// fn main() only needed because this is located in the examples directory.
// This is not needed in your code
fn main() {}
