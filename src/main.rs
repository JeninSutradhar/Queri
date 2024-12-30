use std::net::UdpSocket;
mod buffer;
mod dns_types;
mod dns_packet;
mod resolver;
mod cli;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;
const _: usize = 65535; // Max UDP packet size.

fn handle_query(socket: &UdpSocket) -> Result<()> {
    let mut req_buffer = buffer::BytePacketBuffer::new();
    let (_, src) = socket.recv_from(&mut req_buffer.buf)?;
    let mut request = dns_packet::DnsPacket::from_buffer(&mut req_buffer)?;

    let mut packet = dns_packet::DnsPacket::new();
    packet.header.id = request.header.id;
    packet.header.recursion_desired = true;
    packet.header.recursion_available = true;
    packet.header.response = true;

    if let Some(question) = request.questions.pop() {
        println!("Received query: {:?}", question);

        if let Ok(result) = resolver::recursive_lookup(&question.name, question.qtype) {
            packet.questions.push(question.clone());
            packet.header.rescode = result.header.rescode;

            for rec in result.answers {
                println!("Answer: {:?}", rec);
                packet.answers.push(rec);
            }
             for rec in result.authorities {
                println!("Authority: {:?}", rec);
                packet.authorities.push(rec);
            }
            for rec in result.resources {
                println!("Resource: {:?}", rec);
                packet.resources.push(rec);
            }
        } else {
            packet.header.rescode = dns_types::ResultCode::SERVFAIL;
        }
    } else {
        packet.header.rescode = dns_types::ResultCode::FORMERR;
    }

    let mut res_buffer = buffer::BytePacketBuffer::new();
    packet.write(&mut res_buffer)?;

    let len = res_buffer.pos();
    let data = res_buffer.get_range(0, len)?;

    socket.send_to(data, src)?;
    Ok(())
}
fn main() -> Result<()> {
    let socket = UdpSocket::bind(("0.0.0.0", 8080))?;
    
    // Start a new thread for the DNS server
    std::thread::spawn(move || {
        loop {
            if let Err(e) = handle_query(&socket) {
                 eprintln!("Error handling query: {}", e);
            }
        }
    });
    cli::handle_user_input()?;
    Ok(())
}