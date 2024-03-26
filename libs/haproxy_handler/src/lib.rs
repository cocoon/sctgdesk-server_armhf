use tokio::net::TcpStream;
use std::net::{SocketAddr, ToSocketAddrs};
use bytes::BytesMut;
use tokio::io::AsyncReadExt;
use hbb_common::log;
use proxy_protocol::ProxyHeader;
use proxy_protocol::version2::ProxyAddresses;

pub async fn handle_haproxy_v2(stream: &mut TcpStream, addr: SocketAddr) -> SocketAddr {
        let mut source_address: SocketAddr = addr;
        let mut proxy_v2 = false;

        if let Ok(value) = std::env::var("PROXY-V2") {
            if value.to_uppercase() == "Y" {
                proxy_v2 = true;
                log::debug!("Haproxy v2 protocol is enabled");
            }
        }
        if proxy_v2 {
            // Read the header once at the beginning
            let mut buf_bytes = BytesMut::with_capacity(1024);
            match stream.read_buf(&mut buf_bytes).await {
                Ok(0) => {
                    // socket closed
                    return source_address;
                }
                Ok(_) => {
                    if let Ok(header) = proxy_protocol::parse(&mut buf_bytes) {
                        match header {
                            ProxyHeader::Version2 {
                                command,
                                transport_protocol,
                                addresses,
                            } => {
                                log::debug!("Received a proxied connection with a HAProxy V2 header, command: {:?}, transport protocol: {:?}, addresses: {:?}", command, transport_protocol, addresses);
                                match addresses {
                                    ProxyAddresses::Ipv4 { source, .. } => {
                                        // convert to SocketAddr
                                        source_address = 
                                            source.to_socket_addrs().unwrap().next().unwrap();
                                    }
                                    ProxyAddresses::Ipv6 { source, .. } => {
                                        // convert to SocketAddr
                                        source_address =
                                            source.to_socket_addrs().unwrap().next().unwrap();
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    // Clear the buffer
                    buf_bytes.clear();
                    return source_address;
                }
                Err(_) => {
                    // Error occurred, stop processing
                    return source_address;
                }
            }
        } else {
            return source_address;
        }
    }