use mongodb::{bson::{bson, Bson}};
use rust_net::Socket;
use mongodb::options::CountOptions;

pub const MIN_EMAIL_LENGTH: usize = 5;
pub const MAX_EMAIL_LENGTH: usize = 40;
pub const MIN_NAME_LENGTH: usize = 5;
pub const MAX_NAME_LENGTH: usize = 40;
pub const MIN_PASSWORD_LENGTH: usize = 5;
pub const MAX_PASSWORD_LENGTH: usize = 40;

#[macro_export]
macro_rules! get_client_id {
    ($context: expr, $socket: expr, $v: expr) => {
        match $context.users().find_one(doc!{ "token": $v }, None) {
            Ok(res) => match res {
                Some(user) => user.client_id,
                None => return $socket.send_400(b"User not found")
            },
            Err(e) => return $socket.send_500(e)
        }
    };
}

pub fn count_opts() -> Option<CountOptions> {
    Some(CountOptions::builder()
        .limit(1)
        .build())
}

pub fn byte_array_to_bson(v: &[u8]) -> Vec<Bson> {
    let mut res: Vec<Bson> = vec![];
    for i in v {
        res.push(bson!(*i as i32));
    }
    res
}

pub fn bson_array_to_vec_u8(arr: &Vec<Bson>) -> Vec<u8> {
    let mut res = vec![];
    for v in arr {
        if let Some(v) = v.as_i32() {
            res.push(v as u8)
        }
    }
    res
}

pub fn verify_email(socket: &mut Socket, v: &[u8]) -> bool {
    if v.len() < MIN_EMAIL_LENGTH {
        socket.send_400(b"E-mail is too short");
        return false
    }
    if v.len() > MAX_EMAIL_LENGTH {
        socket.send_400(b"E-mail is too long");
        return false
    }
    true
}
pub fn verify_name(socket: &mut Socket, v: &[u8]) -> bool {
    if v.len() < MIN_NAME_LENGTH {
        socket.send_400(b"Name is too short");
        return false
    }
    if v.len() > MAX_NAME_LENGTH {
        socket.send_400(b"Name is too long");
        return false
    }
    true
}
pub fn verify_password(socket: &mut Socket, v: &[u8]) -> bool {
    if v.len() < MIN_PASSWORD_LENGTH {
        socket.send_400(b"Password is too short");
        return false
    }
    if v.len() > MAX_PASSWORD_LENGTH {
        socket.send_400(b"Password is too long");
        return false
    }
    true
}