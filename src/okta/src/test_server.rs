use serde_json::Value;
use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::JoinHandle,
    time::{Duration, Instant},
};
pub struct Reply {
    pub prefix: &'static str,
    pub body: Value,
    pub status: u16,
    pub delay: Duration,
    pub headers: Vec<(String, String)>,
}
impl Reply {
    pub fn ok(prefix: &'static str, body: Value) -> Self {
        Self {
            prefix,
            body,
            status: 200,
            delay: Duration::ZERO,
            headers: vec![],
        }
    }
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
}
pub struct Server {
    pub origin: String,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<Vec<String>>>,
}
impl Server {
    pub fn new(replies: impl FnOnce(&str) -> Vec<Reply>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let origin = format!("http://{}", listener.local_addr().unwrap());
        let replies = replies(&origin);
        let stop = Arc::new(AtomicBool::new(false));
        let abort = stop.clone();
        let thread = std::thread::spawn(move || {
            let mut requests = vec![];
            for reply in replies {
                let deadline = Instant::now() + Duration::from_secs(8);
                let (mut stream, _) = loop {
                    if abort.load(Ordering::Relaxed) {
                        return requests;
                    }
                    match listener.accept() {
                        Ok(s) => break s,
                        Err(e)
                            if e.kind() == std::io::ErrorKind::WouldBlock
                                && Instant::now() < deadline =>
                        {
                            std::thread::sleep(Duration::from_millis(1))
                        }
                        Err(e) => panic!("missing fixture request {}: {e}", reply.prefix),
                    }
                };
                stream
                    .set_read_timeout(Some(Duration::from_secs(3)))
                    .unwrap();
                let mut bytes = vec![];
                let mut buffer = [0; 4096];
                loop {
                    let n = stream.read(&mut buffer).unwrap();
                    assert_ne!(n, 0);
                    bytes.extend_from_slice(&buffer[..n]);
                    if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                        let header = String::from_utf8_lossy(&bytes[..end]);
                        let length = header
                            .lines()
                            .find_map(|line| {
                                line.to_ascii_lowercase()
                                    .strip_prefix("content-length:")
                                    .map(|s| s.trim().parse::<usize>().unwrap())
                            })
                            .unwrap_or(0);
                        if bytes.len() >= end + 4 + length {
                            break;
                        }
                    }
                }
                let request = String::from_utf8(bytes).unwrap();
                assert!(
                    request.starts_with(reply.prefix),
                    "unexpected fixture request: {request}"
                );
                requests.push(request);
                let body = reply.body.to_string();
                std::thread::sleep(reply.delay);
                let headers = reply
                    .headers
                    .iter()
                    .map(|(name, value)| format!("{name}: {value}\r\n"))
                    .collect::<String>();
                let sent=write!(stream,"HTTP/1.1 {} Fixture\r\nContent-Type: application/json\r\n{}Content-Length: {}\r\nConnection: close\r\n\r\n{}",reply.status,headers,body.len(),body);
                if reply.delay.is_zero() {
                    sent.unwrap();
                }
            }
            requests
        });
        Self {
            origin,
            stop,
            thread: Some(thread),
        }
    }
    pub fn finish(mut self) -> Vec<String> {
        self.thread.take().unwrap().join().unwrap()
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}
