use crate::{
    // dns_packet::DnsPacket,
    dns_types::{DnsRecord, QueryType, ResultCode},
    Result,
};
use colored::*;
use std::io::{self, Write};
pub fn print_header() {
    println!("{}", "\n========================= DNS Resolution Results =========================".green());
    println!("{:<10} | {:<11} | {:<43} | {:<5}", 
             "Query Type".bold(), 
             "Response Code".bold(), 
             "Record".bold(), 
             "TTL".bold());
    println!("--------------------------------------------------------------------");
}

pub fn print_record(qtype: &QueryType, response_code: &ResultCode, record: &DnsRecord) {
    match record {
        DnsRecord::A { domain, addr, ttl } => {
            println!("{:<10} | {:<12} | {:<40} | {:<5}", 
                     qtype.to_string().blue(), 
                     response_code.to_string().blue(), 
                     format!("{} - {}", domain, addr).cyan(), 
                     ttl.to_string().yellow());
        }
        DnsRecord::AAAA { domain, addr, ttl } => {
            println!("{:<10} | {:<12} | {:<40} | {:<5}", 
                     qtype.to_string().blue(), 
                     response_code.to_string().blue(), 
                     format!("{} - {}", domain, addr).cyan(), 
                     ttl.to_string().yellow());
        }
        DnsRecord::CNAME { domain, host, ttl } => {
            println!("{:<10} | {:<12} | {:<40} | {:<5}", 
                     qtype.to_string().blue(), 
                     response_code.to_string().blue(), 
                     format!("{} - {}", domain, host).cyan(), 
                     ttl.to_string().yellow());
        }
        DnsRecord::MX { domain, priority, host, ttl } => {
            println!("{:<10} | {:<12} | {:<40} | {:<5}", 
                     qtype.to_string().blue(), 
                     response_code.to_string().blue(), 
                     format!("{} - {} - {}", domain, priority, host).cyan(), 
                     ttl.to_string().yellow());
        }
        DnsRecord::NS { domain, host, ttl } => {
            println!("{:<10} | {:<12} | {:<40} | {:<5}", 
                     qtype.to_string().blue(), 
                     response_code.to_string().blue(), 
                     format!("{} - {}", domain, host).cyan(), 
                     ttl.to_string().yellow());
        }
        DnsRecord::SRV { domain, priority, weight, port, target, ttl } => {
            println!("{:<10} | {:<12} | {:<40} | {:<5}", 
                     qtype.to_string().blue(), 
                     response_code.to_string().blue(), 
                     format!("{} - {}:{}:{} - {}", domain, priority, weight, port, target).cyan(), 
                     ttl.to_string().yellow());
        }
         DnsRecord::TXT { domain, text, ttl } => {
            println!("{:<10} | {:<12} | {:<40} | {:<5}", 
                     qtype.to_string().blue(), 
                     response_code.to_string().blue(), 
                     format!("{} - {}", domain, text).cyan(), 
                     ttl.to_string().yellow());
        }
        DnsRecord::UNKNOWN { domain, qtype, data_len, ttl } => {
            println!("{:<10} | {:<12} | {:<40} | {:<5}", 
                     qtype.to_string().blue(), 
                     response_code.to_string().blue(), 
                     format!("{} - QType: {}, Data Length: {}", domain, qtype, data_len).cyan(), 
                     ttl.to_string().yellow());
        }
    }
}

pub fn print_no_records(qtype: &QueryType, response_code: &ResultCode) {
    println!("{:<10} | {:<12} | {:<40} | {:<5}", 
             qtype.to_string().blue(), 
             response_code.to_string().blue(), 
             "No Records Found".red(), 
             "".yellow());
}

pub fn handle_user_input() -> Result<()> {
    loop {
          // Prompt for domain name
        print!("Enter domain to resolve: ");
        io::stdout().flush()?;
        let mut domain = String::new();
        io::stdin().read_line(&mut domain)?;
        let domain = domain.trim();

        // Prompt for query type
        print!("Enter query type (A, AAAA, MX, NS, CNAME, SRV, TXT, ALL): ");
        io::stdout().flush()?;
        let mut qtype_str = String::new();
        io::stdin().read_line(&mut qtype_str)?;
        let qtype_str = qtype_str.trim().to_uppercase();

        let qtype = match qtype_str.as_str() {
            "A" => QueryType::A,
            "AAAA" => QueryType::AAAA,
            "MX" => QueryType::MX,
            "NS" => QueryType::NS,
            "CNAME" => QueryType::CNAME,
            "SRV" => QueryType::SRV,
            "TXT" => QueryType::TXT,
            "ALL" => QueryType::ALL,
            _ => {
                println!("Invalid query type, defaulting to A");
                QueryType::A
            }
        };

        println!("Attempting to resolve {} with type {}", domain, qtype_str);
        let result = match qtype {
            QueryType::ALL => {
                let types = vec![
                    QueryType::A,
                    QueryType::AAAA,
                    QueryType::MX,
                    QueryType::NS,
                    QueryType::CNAME,
                     QueryType::SRV,
                     QueryType::TXT,
                ];
                let mut all_packets = Vec::new();
                for t in types {
                    match crate::resolver::recursive_lookup(domain, t) {
                        Ok(packet) => all_packets.push(packet),
                        Err(e) => {
                            eprintln!("Error during resolution of {:?}: {}", t, e);
                        }
                    }
                }
                Ok(all_packets)
            }
            _ => match crate::resolver::recursive_lookup(domain, qtype) {
                Ok(packet) => Ok(vec![packet]),
                Err(e) => Err(e),
            },
        };

        match result {
            Ok(packets) => {
                print_header();
                for packet in packets {
                    let qtype = packet.questions.get(0).map_or(QueryType::UNKNOWN(0), |q| q.qtype);
                    let response_code = packet.header.rescode;

                    for record in &packet.answers {
                        print_record(&qtype, &response_code, record);
                    }

                    if packet.answers.is_empty() {
                        print_no_records(&qtype, &response_code);
                    }
                }
                println!("{}", "\n==================== Resolution Complete ====================".green());
            }
            Err(e) => eprintln!("Error during resolution: {}", e),
        }

        // Ask if the user wants to perform another query
         print!("\nDo you want to resolve another domain? (y/n): ");
        io::stdout().flush()?;
        let mut continue_query = String::new();
        io::stdin().read_line(&mut continue_query)?;
        if continue_query.trim().to_lowercase() != "y" {
            println!("Exiting DNS Resolver...");
            break;
        }

    }
    Ok(())
}