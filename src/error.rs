use rust_net::{Socket, TcpStream};

pub fn on_error_500(socket: &mut TcpStream, e: impl std::fmt::Display) {
    socket.send_500(e.to_string().as_bytes());
    println!("Internal error: {}", e);
}