use bitcoincore_rpc::{Auth, Client, Error};

pub fn create_rpc_client() -> Result<Client, Error> {
    let rpc_url = "http://127.0.0.1:18443";
    let rpc_auth = Auth::UserPass("bitcoinuser".into(), "strongpassword".into());
    Client::new(rpc_url, rpc_auth)
}

