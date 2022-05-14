use std::str;
use rust_net::{Response, Socket, get_body_utf8};
use mongodb::{bson::{oid::ObjectId, doc, Bson}, sync::Collection};
use serde::{Deserialize, Serialize};
pub mod encryption;
pub mod utils;
pub mod token;
use encryption::*;
use utils::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    _id: ObjectId,
    token: String,
    email: String,
    password: Vec<Bson>,
    users: Vec<User>
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    _id: ObjectId,
    client_id: ObjectId,
    token: String,
    privilege: u32,
    name: String,
    prof_pic: String,
    password: String
}
impl From<User> for Bson {
    fn from(user: User) -> Self {
        Self::from(doc! {
            "_id": user._id.clone(),
            "client_id": user.client_id.clone(),
            "token": user.token.clone(),
            "privilege": user.privilege.clone(),
            "name": user.name.clone(),
            "prof_pic": user.prof_pic.clone(),
            "password": user.password.clone()
        })
    }
}

pub trait AuthContext {
    fn clients(&mut self) -> &mut Collection<Client>;
    fn users(&mut self) -> &mut Collection<User>;
    fn salts(&self) -> &Salts;
    fn admin_levels(&self) -> &'static [&'static str] {
        &["normal", "admin", "owner"]
    }
    fn max_users(&self) -> usize {
        100
    }
}

pub fn set_auth_routes(server: &mut rust_net::Server<impl AuthContext>) {
    server.add_post_route("client/exists/by/email", |context, socket, req| {
        let email_bytes = req.get_body();
        if !verify_email(socket, email_bytes) {return}
        let email = String::from_utf8_lossy(email_bytes).to_string();
        match context.clients().count_documents(doc!{ "email": &email }, count_opts()) {
            Ok(0) => return socket.send_400(b"Not found"),
            Ok(_) => return socket.send_200(b"Found"),
            Err(e) => return socket.send_500(e)
        }
    });
    server.add_post_route("client/exists/by/token", |context, socket, req| {
        let token = get_body_utf8!(req, socket);
        match context.clients().count_documents(doc!{ "token": &token }, count_opts()) {
            Ok(0) => return socket.send_400(b"Not found"),
            Ok(_) => return socket.send_200(b"Found"),
            Err(e) => return socket.send_500(e)
        }
    });
    
    server.add_post_route("client/new", |context, socket, req| {
        let data = req.get_body_formated();
        if data.len() != 2 {
            return socket.send_400(b"Bad request format");
        }
        let email_bytes = data[1];
        if !verify_email(socket, email_bytes) {return}
        let password_bytes = data[0];
        if !verify_password(socket, password_bytes) {return}

        let email = String::from_utf8_lossy(email_bytes).to_owned().to_string();
        match context.clients().count_documents(doc!{ "email": &email }, count_opts()) {
            Ok(0) => {},
            Ok(_) => return socket.send_400(b"E-mail is already in use"),
            Err(e) => return socket.send_500(e)
        }
        let password_encrypted = encrypt(context.salts(), email_bytes, password_bytes);
        let password_encrypted_bson_array = byte_array_to_bson(&password_encrypted);
        let token = token::new();
        match context.clients().insert_one(Client {
            _id: ObjectId::new(),
            token: token.clone(),
            email,
            password: password_encrypted_bson_array,
            users: vec![]
        }, None) {
            Ok (_) => return socket.send_200(token.as_bytes()),
            Err(e) => return socket.send_500(e)
        }
    });
    
    server.add_post_route("client/login", |context, socket, req| {
        let data = req.get_body_formated();
        if data.len() != 2 {
            return socket.send_400(b"Bad request format");
        }
        let email_bytes = data[1];
        let password_bytes = data[0];
        let email = String::from_utf8_lossy(email_bytes).to_owned().to_string();
        let client = match context.clients().find_one(doc!{ "email": &email }, None) {
            Ok(client) => match client {
                Some(client) => client,
                None => return socket.send_400(b"E-mail not found")
            },
            Err(e) => return socket.send_500(e)
        };
        let password_encrypted = bson_array_to_vec_u8(&client.password);
        if verify(context.salts(), email_bytes, password_bytes, &password_encrypted) {
            socket.send_200(client.token.as_bytes());
        }else {
            return socket.send_400(b"Wrong password")
        }
    });

    server.add_post_route("client/user", |context, socket, req| {
        match context.users().find_one(doc!{ "token": &get_body_utf8!(req, socket) }, None) {
            Ok(res) => match res {
                Some(user) =>
                    socket.send_200(format!("{}|{}|{}|{}", 
                        user.name,
                        user.prof_pic,
                        user.password.len() > 0,
                        user.privilege
                    ).to_string().as_bytes()),
                None => return socket.send_400(b"User not found")
            },
            Err(e) => return socket.send_500(e)
        }
    });
    
    server.add_post_route("client/users", |context, socket, req| {
        let token = get_body_utf8!(req, socket);
        let client = match context.clients().find_one(doc!{ "token": &token }, None) {
            Ok(client) => match client {
                Some(client) => client,
                None => return socket.send_400(b"Client not found")
            },
            Err(e) => return socket.send_500(e)
        };
        let mut users = vec![];
        for user in client.users {
            users.push(format!("{}|{}|{}|{}", 
                user.name,
                user.prof_pic,
                user.password.len() > 0,
                user.privilege
            ).to_string())
        }
        let res = users.join("#");
        socket.send_200(res.as_bytes());
    });
    
    server.add_post_route("client/users/new", |context, socket, req| {
        let data = req.get_body_formated();
        if data.len() != 3 {
            return socket.send_400(b"Bad request format");
        }
        let token_bytes = data[2];
        let name_bytes = data[1];
        let password_bytes = data[0];
        if !verify_name(socket, name_bytes) {return}
        if password_bytes.len() > MAX_PASSWORD_LENGTH {
            return socket.send_400(b"Password is too long")
        }
        let token = String::from_utf8_lossy(token_bytes).to_string();
        let name = String::from_utf8_lossy(name_bytes).to_string();
        let mut privilege = context.admin_levels().len() as u32 - 1;
        let client_id;
        match context.clients().find_one(doc!{ "token": &token }, None) {
            Ok(client) => match client {
                Some(client) => {
                    if client.users.len() >= context.max_users() {
                        return socket.send_400(format!("Can not create more than {} users", context.max_users()).to_string().as_bytes())
                    }
                    for user in client.users {
                        if user.name == name {
                            return socket.send_400(b"Name already in use")
                        }
                        if user.privilege == privilege {
                            privilege = 0
                        }
                    }
                    client_id = client._id;
                },
                None => return socket.send_400(b"Client not found")
            },
            Err(e) => return socket.send_500(e)
        }
        let password = String::from_utf8_lossy(password_bytes).to_string();
        let user = User {
            _id: ObjectId::new(),
            client_id,
            token: token::new(),
            privilege,
            name: name.clone(),
            prof_pic: String::new(),
            password: password.clone()
        };
        match context.clients().update_one(
            doc!{ "token": &token },
            doc!{ "$push": { 
                "users": &user
            } },
            None
        ) {
            Ok (_) => {
                match context.users().insert_one(&user, None) {
                    Ok (_) => return socket.send_200(b"OK"),
                    Err(e) => return socket.send_500(e)
                }
            },
            Err(e) => return socket.send_500(e)
        }
    });
    
    server.add_post_route("client/users/login", |context, socket, req| {
        let data = req.get_body_formated();
        if data.len() != 3 {
            return socket.send_400(b"Bad request format");
        }
        let token_bytes = data[2];
        let name_bytes = data[1];
        let password_bytes = data[0];
        let token = String::from_utf8_lossy(token_bytes).to_string();
        let name = String::from_utf8_lossy(name_bytes).to_string();
        let password = String::from_utf8_lossy(password_bytes).to_string();
        match context.clients().find_one(doc!{ "token": token }, None) {
            Ok (client) => match client {
                Some(client) => {
                    for user in client.users {
                        if user.name == name {
                            if user.password == password {
                                return socket.send_200(user.token.as_bytes())
                            }else {
                                return socket.send_400(b"Wrong password")
                            }
                        }
                    }
                    return socket.send_400(b"User not found")
                },
                None => return socket.send_400(b"Client not found")
            },
            Err(e) => return socket.send_500(e)
        }
    });
}

pub fn get_collections<S: Into<String>>(mongodb_url: S) -> (Collection<Client>, Collection<User>) {
    println!("Connecting to mongodb ...");
    let mongodb_url = &mongodb_url.into();
    let client_options = match mongodb::options::ClientOptions::parse(mongodb_url) {
        Ok(v) => v,
        Err(e) => panic!("Mongodb client options error: {}", e)
    };
    let client = match mongodb::sync::Client::with_options(client_options) {
        Ok(v) => v,
        Err(e) => panic!("Can't connect to mongodb url\r\n\turl: {}\r\n\terror: {}", mongodb_url, e)
    };
    let db = client.database("account");
    (db.collection("clients"), db.collection("users"))
}
