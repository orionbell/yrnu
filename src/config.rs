use crate::core::IpAddress;
use ssh2::Session;
use std::error::Error;
use std::io::prelude::*;
use std::path::PathBuf;

pub enum SSHAuthType {
    Arguments(String, String),
    UserInput,
    KeyPair(String, Option<PathBuf>, PathBuf, Option<String>),
    Agent(String),
}

pub async fn connect(
    host: IpAddress,
    port: Option<u16>,
    auth: SSHAuthType,
) -> Result<Session, Box<dyn Error>> {
    let mut sess = Session::new()?;
    let addr = host.address();
    let port = port.unwrap_or(22);
    let sock = tokio::net::TcpStream::connect(format!("{addr}:{port}")).await;
    if let Err(e) = sock {
        return Err(Box::new(e));
    }
    sess.set_tcp_stream(sock.unwrap());
    sess.handshake()?;

    match auth {
        SSHAuthType::Arguments(user, passwd) => {
            sess.userauth_password(user.trim(), passwd.trim())?;
        }
        SSHAuthType::UserInput => {
            let mut user = String::new();
            let passwd;
            print!("User: ");
            _ = std::io::stdout().flush();
            _ = std::io::stdin().read_line(&mut user);
            print!("Password: ");
            _ = std::io::stdout().flush();
            passwd = rpassword::read_password().unwrap_or_else(|e| {
                eprintln!("Couldn't read user password: {e}");
                String::new()
            });
            sess.userauth_password(user.trim(), passwd.trim())?;
        }
        SSHAuthType::KeyPair(user, public, private, passphrase) => {
            let public = if let Some(public) = public {
                Some(public)
            } else {
                None
            };
            sess.userauth_pubkey_file(&user, public.as_deref(), &private, passphrase.as_deref())?;
        }
        SSHAuthType::Agent(user) => {
            sess.userauth_agent(&user)?;
        }
    };
    Ok(sess)
}

pub fn run(
    sess: &Session,
    config: String,
) -> Result<(Option<String>, Option<String>, i32), ssh2::Error> {
    let mut chan = sess.channel_session()?;
    let mut stdout = String::new();
    let mut stderr = String::new();
    for line in config.lines() {
        match chan.exec(line) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };
        _ = chan.read_to_string(&mut stdout);
        _ = chan.stderr().read_to_string(&mut stderr);
    }
    chan.wait_close()?;
    Ok((
        Some(stdout.trim().to_string()),
        Some(stderr.trim().to_string()),
        chan.exit_status()?,
    ))
}
