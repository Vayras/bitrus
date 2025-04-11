use bitcoincore_rpc::{Auth, Client, Error};
use std::env;
use dotenv::dotenv;

pub fn create_rpc_client() -> Result<Client, Error> {
    dotenv().ok(); // Load .env

    let user = env::var("BITCOIN_RPC_USER").unwrap_or("bitcoinuser".into());
    let pass = env::var("BITCOIN_RPC_PASS").unwrap_or("strongpassword".into());
    let host = env::var("BITCOIN_RPC_HOST").unwrap_or("127.0.0.1".into());
    let port = env::var("BITCOIN_RPC_PORT").unwrap_or("18443".into());

    let rpc_url = format!("http://{}:{}", host, port);
    let rpc_auth = Auth::UserPass(user, pass);

    Client::new(&rpc_url, rpc_auth)
}

