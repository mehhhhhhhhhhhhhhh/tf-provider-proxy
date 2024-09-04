use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::os::fd::{AsFd, OwnedFd};
use std::str;
use hyper_util::rt::TokioIo;
use tokio::net::UnixStream;
use tonic::transport::{Channel, Endpoint, Uri};
use tower::service_fn;

use super::proto::ProviderClient;

struct ChildGuard(Child);
impl Drop for ChildGuard {
    fn drop(&mut self) {
        match self.0.kill() {
            Err(e) => eprintln!("Could not kill child process: {}", e),
            Ok(_) => {},
        }
    }
}
impl ChildGuard {
    fn stdout(&self) -> OwnedFd {
        self.0.stdout.as_ref().unwrap().as_fd().try_clone_to_owned().unwrap()
    }
}

pub struct Client {
    pub client: ProviderClient<Channel>,
    process: ChildGuard,
    stdout: BufReader<File>,
}
impl Client {
    pub async fn with_binary(process_path: &PathBuf) -> Self {
        let junk = Command::new(process_path)
            .env("TF_PLUGIN_MAGIC_COOKIE", "d602bf8f470bc67ca7faa0386276bbdd4330efaf76d1a219cb4d6991ca9872b2")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let child_g = ChildGuard(junk);
        let output = File::from(child_g.stdout());
        let mut output = BufReader::new(output);

        let mut buf = vec!();
        let shit = output.read_until('\n' as u8, &mut buf).unwrap();
        eprintln!("{}", &shit);
        let shit = str::from_utf8(&buf).unwrap().trim_end();
        eprintln!("{}", &shit);
        let sock_path = shit.split('|').nth(3).unwrap().to_string();
        eprintln!("{}", &sock_path);

        let uri = format!("http://[::]:51515{}", sock_path);
            // pointlessly embed the unix path in fake http: url, to work around the Url type

        eprintln!("{}", uri);
        let channel = Endpoint::try_from(uri).unwrap()
            .connect_with_connector(service_fn(|sock_uri: Uri| async move {
                let stream = UnixStream::connect(&sock_uri.path()).await.unwrap();
                Ok::<_, std::io::Error>(TokioIo::new(stream))
            }))
            .await
            .unwrap();
        let client = ProviderClient::new(channel);

        Client {
            client,
            process: child_g,
            stdout: output,
        }
            // TODO this struct is just a way of returning values. need to ensure these things are retained together!
    }
}
