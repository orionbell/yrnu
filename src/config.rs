use crate::core::IpAddress;
use ssh2::Session;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

pub enum SSHAuthType<'a> {
    Arguments(String, String),
    UserInput,
    KeyPair(String, Option<&'a Path>, &'a Path, Option<String>),
    Agent(String),
}

pub async fn connect(
    host: &IpAddress,
    port: Option<u16>,
    auth: SSHAuthType<'_>,
) -> Result<Session, Box<dyn Error>> {
    let mut sess = Session::new()?;
    let addr = host.address();
    let port = port.unwrap_or(22);
    sess.set_tcp_stream(
        tokio::net::TcpStream::connect(format!("{addr}:{port}"))
            .await
            .unwrap(),
    );
    sess.handshake()?;

    match auth {
        SSHAuthType::Arguments(user, passwd) => {
            sess.userauth_password(&user, &passwd)?;
        }
        SSHAuthType::UserInput => {
            let mut user = String::new();
            let passwd;
            print!("User: ");
            _ = std::io::stdout().flush();
            _ = std::io::stdin().read_line(&mut user);
            print!("Password: ");
            _ = std::io::stdout().flush();
            passwd = rpassword::read_password()?;
            sess.userauth_password(&user, &passwd)?;
        }
        SSHAuthType::KeyPair(user, public, private, passphrase) => {
            sess.userauth_pubkey_file(&user, public, private, passphrase.as_deref())?;
        }
        SSHAuthType::Agent(user) => {
            sess.userauth_agent(&user)?;
        }
    };
    Ok(sess)
}

pub fn run(sess: &Session, config: String) -> Result<Option<String>, Box<dyn Error>> {
    let mut chan = sess.channel_session()?;
    let mut output = String::new();
    for line in config.lines() {
        chan.exec(line)?;
        match chan.read_to_string(&mut output) {
            Ok(_) => {}
            Err(e) => {}
        };
    }
    chan.wait_close()?;
    Ok(Some(output))
}
