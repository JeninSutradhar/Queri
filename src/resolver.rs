use crate::{
    buffer::BytePacketBuffer,
    dns_packet::DnsPacket,
    dns_types::{QueryType,ResultCode, DnsQuestion},
    Result,
};
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::{Duration, Instant},
};

const TIMEOUT_DURATION: Duration = Duration::from_secs(10);
fn lookup(qname: &str, qtype: QueryType, server: (Ipv4Addr, u16)) -> Result<DnsPacket> {
    let socket = UdpSocket::bind(("0.0.0.0", 0))?;
    socket.set_read_timeout(Some(TIMEOUT_DURATION))?;

    let mut packet = DnsPacket::new();
    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer)?;
    let server_addr = SocketAddr::from((server.0, server.1));
    socket.send_to(&req_buffer.buf[0..req_buffer.pos], server_addr)?;

    let mut res_buffer = BytePacketBuffer::new();
    let (_, _) = match socket.recv_from(&mut res_buffer.buf) {
        Ok(res) => res,
        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            return Err("Timeout during DNS query".into())
        }
        Err(e) => return Err(e.into()),
    };
    DnsPacket::from_buffer(&mut res_buffer)
}
pub fn recursive_lookup(qname: &str, qtype: QueryType) -> Result<DnsPacket> {
    let mut ns = "198.41.0.4".parse::<Ipv4Addr>().unwrap();

    let mut cache: HashMap<(String, QueryType), DnsPacket> = HashMap::new();

    let now = Instant::now();
    let max_time = Duration::from_secs(10);
    let _ = 0;

    loop {
        if now.elapsed() > max_time {
            println!("Timeout out, failed to find {} for {:#?}", qname, qtype);
            break;
        }

        println!("attempting lookup of {:?} {} with ns {}", qtype, qname, ns);
        let cache_key = (qname.to_string(), qtype);
        if let Some(cached_packet) = cache.get(&cache_key) {
            println!("found cached entry");
            return Ok(cached_packet.clone());
        }
        let ns_copy = ns;
        let server = (ns_copy, 53);

        let response = match lookup(qname, qtype, server) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error during lookup: {}", e);
                return Err(e);
            }
        };

        if !response.answers.is_empty() && response.header.rescode == ResultCode::NOERROR {
            cache.insert(cache_key, response.clone());
            return Ok(response);
        }

        if response.header.rescode == ResultCode::NXDOMAIN {
            return Ok(response);
        }

        if let Some(new_ns) = response.get_resolved_ns(qname) {
            ns = new_ns;
            continue;
        }

        let new_ns_name = match response.get_unresolved_ns(qname) {
            Some(x) => x,
            None => return Ok(response),
        };
        let recursive_response = match recursive_lookup(&new_ns_name, QueryType::A) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Error resolving NS: {}", e);
                return Ok(response);
            }
        };

        if let Some(new_ns) = recursive_response.get_random_a() {
            ns = new_ns;
        } else {
            return Ok(response);
        }
    }
    Err("Failed to lookup, timeout".into())
}