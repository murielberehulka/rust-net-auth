use rust_net::Server;
use rust_net_auth::{Client, User};
use mongodb::sync::Collection;

struct Context {
    pub clients_collection: Collection<Client>,
    pub users_collection: Collection<User>
}
impl rust_net_auth::AuthContext for Context {
    fn clients(&mut self) -> &mut Collection<Client> { &mut self.clients_collection }
    fn users(&mut self) -> &mut Collection<User> { &mut self.users_collection }
    fn salts(&self) -> &rust_net_auth::encryption::Salts {
        //Random numbers for password encryptation before send to database
        &[165,89,92,232,18,17,5,162,33,142,37,44,20,186,82,71]
    }
    fn admin_levels(&self) -> &'static [&'static str] {
        &["normal", "admin", "owner"]
    }
    fn max_users(&self) -> usize {
        100
    }
}

fn main() {
    let (clients_collection, users_collection) = rust_net_auth::get_collections("mongodb_url");
    let context = Context {
        clients_collection,
        users_collection
    };

    let mut server = Server::new(rust_net::Settings {
        static_files: None,
        ..Default::default()
    }, context);

    rust_net_auth::set_auth_routes(&mut server);

    println!("Server running ...");
    server.run();
}