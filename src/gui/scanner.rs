use futures::stream::FuturesUnordered;
use futures::StreamExt;
use itertools::Itertools;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

// https://cookie.engineer/weblog/articles/implementers-guide-to-socks.html

async fn check_proxy(proxy_addr: &SocketAddr,timeout_time: u64) -> Option<(SocketAddr, Duration)> {
    let start = Instant::now();

    let mut stream = match timeout(Duration::from_secs(timeout_time), TcpStream::connect(proxy_addr)).await {
        Ok(Ok(stream)) => stream,
        _ => return None,
    };

    //stream.set_nodelay(true).unwrap();

    let handshake = [0x05, 0x01, 0x00];

    if stream.write_all(&handshake).await.is_err() {
        return None;
    }

    let mut response = [0; 2];
    if timeout(Duration::from_secs(timeout_time), stream.read_exact(&mut response)).await.is_err(){
        return None;
    }

    if response[1] != 0 || response[0] != 5 {
        return None;
    }

    Some((proxy_addr.to_owned(), start.elapsed()))
}

pub struct ProxyResult {
    pub ip: SocketAddr,
    pub delay: f32,
}

pub fn scan(list: &String, tx: Sender<Option<ProxyResult>>, timeout:u64, batch_size:u64) {
    let proxies: Vec<SocketAddr> = list
        .lines()
        .into_iter()
        .unique()
        .map(|proxy| SocketAddr::from_str(proxy).expect("Coundent parse ip"))
        .collect();

    tokio::spawn(async move {
        let len = proxies.len();

        println!("Starting: {} Proxies to check", len);
        //let start = Instant::now();

        let mut ftrs = FuturesUnordered::new();
        let mut proxy_iterator = proxies.iter();

        
        for _ in 0..batch_size {
            if let Some(proxy) = proxy_iterator.next() {
                ftrs.push(check_proxy(proxy,timeout));
            } else {    
                break;
            }
        }

        //let mut out:Vec<ProxyResult> = Vec::with_capacity(len/2);
        while let Some(result) = ftrs.next().await {
            if let Some(proxy) = proxy_iterator.next() {
                ftrs.push(check_proxy(proxy,timeout));
            }

            if let Some((proxy_addr, speed)) = result {
                //out.push(ProxyResult { ip: proxy_addr, delay: speed.as_secs_f32() })
                tx.send(Some(ProxyResult {
                    ip: proxy_addr,
                    delay: speed.as_secs_f32(),
                }))
                .expect("error couldent send");
            }
        }
        tx.send(None).expect("error couldent send");
        
        //let elapsed = start.elapsed().as_secs_f32();
        //println!("Done took {} Proxy/s Took {} Seconds",len as f32 / elapsed,elapsed);
    });
}
