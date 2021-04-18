use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResult {
    ssh_port: Option<String>,
    mosh_ports: Option<String>,
    server: Option<String>
}
