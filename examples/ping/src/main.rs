use snowboard::{response, Server};

fn main() {
    Server::new("localhost:8080", "")
        .on_request(|_, _| response!(ok))
        .run();
}