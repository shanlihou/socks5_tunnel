use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub remote_host: String,
    pub local_host: String,
    pub socks_host: String,
}


