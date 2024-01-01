use etherparse::{InternetSlice, SlicedPacket};
use etherparse::IpNumber::Stream;
use crate::lucyna::Lucyna;

pub struct LucynaParse {
    ip: IPParser,
}

impl LucynaParse {
    pub fn new() -> LucynaParse {
        LucynaParse {
            ip: IPParser::new(),
        }
    }

    pub fn parse_packet(&mut self, packet: SlicedPacket) {
        //println!("link: {:?}", packet.link);
        //println!("vlan: {:?}", packet.vlan);
        if let Some(p) = packet.ip {
            self.ip.parse_ip_part(p);
        }

        //println!("ip: {:?}", packet.ip);
        //println!("transport: {:?}", packet.transport);
    }
    pub fn get_seen_ips(&self) -> Vec<String> {
        return self.ip.discovered.clone();
    }
}

pub struct EthernetParser {

}

impl EthernetParser {
    fn new() -> EthernetParser {
        EthernetParser {

        }
    }
}

pub struct IPParser {
    pub discovered: Vec<String>,
}

impl IPParser {
    fn new() -> IPParser {
        IPParser {
            discovered: vec![]
        }
    }

    fn parse_ip_part(&mut self, ip_slice: InternetSlice) {
        match ip_slice {
            InternetSlice::Ipv4(ipv4_header, ipv4_extensions) => {
                // Hier kannst du auf die IPv4-Daten zugreifen
                let mut src = String::new();
                let mut dst = String::new();
                src = ipv4_header.source_addr().to_string();
                dst = ipv4_header.destination_addr().to_string();

                if !self.discovered.iter().any(|i| i==&src) {
                    println!("New IP found!");
                    self.discovered.push(src.clone());
                }
                if !self.discovered.iter().any(|i| i==&dst) {
                    println!("New IP found!");
                    self.discovered.push(dst.clone());
                }

                //println!("IPv4-Source: {:?}", ipv4_header.source_addr());
                //println!("IPv4-Destination: {:?}", ipv4_header.destination_addr());

            }
            InternetSlice::Ipv6(ipv6_header, ipv6_extensions) => {
                //println!("IPv6-Source: {:?}", ipv6_header.source_addr());
                //println!("IPv4-Destination: {:?}", ipv6_header.destination_addr());
            }
        }
    }
}