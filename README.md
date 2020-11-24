# roboxide

`roboxide` is a (very) stripped down library that aims to fulfill the same function as [ROS](https://www.ros.org/), while leveraging Rust's ecosystem
as much as possible. It is very opinionated about its use of Rust - there is no infrastructure to bridge other languages
with message and service files, client libraries, etc. The main advantages of this are:

#### Simpler Build Process
Instead of using tools like catkin/colcon and cmake, you can use cargo to build your libraries/nodes. All it takes is a
`[[bin]]` or `[[lib]]` entry in your `Cargo.toml`.

#### Builtin Types
You can use regular Rust structs as messages, instead of creating dedicated message and service files. The only
requirement is they implement serde's Serialize and Deserialize traits, which can easily be done with the `#[derive]`
macro.

#### Package Management
Packages can be uploaded to crates.io, circumventing the need for system dependent package managers. You just need
cargo.


### Dependencies
This library depends on [ZeroMQ](https://github.com/zeromq/libzmq), so `libzmq` needs to be
installed before building this library. A native Rust
implementation of ZeroMQ is in the works ([zmq.rs](https://github.com/zeromq/zmq.rs)), but it's not production ready as of
now. Hopefully in the future it will be, so `libzmq` won't have to be installed separately to use this library.

### Examples
Pub/sub is the only messaging mechanism implemented at the moment. You can test it with with some nodes in `examples/`. 

#### 1. Open a terminal and run the publisher example.
```bash
cargo run --example publisher
```

#### 2. Open another terminal and run the subscriber example.
```bash
cargo run --example subscriber
```

#### 3. Profit
The subscriber will begin receiving messages.
