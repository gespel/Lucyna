use etherparse::{Ethernet2HeaderSlice, InternetSlice, LinkSlice, SlicedPacket, TransportSlice};
use etherparse::IpNumber::Stream;
use crate::lucyna::Lucyna;

pub struct LucynaParse {
    ep: EthernetParser,
    ipp: IPParser,
    discovered_ips: Vec<String>
}

impl LucynaParse {
    pub fn new() -> LucynaParse {
        LucynaParse {
            ep: EthernetParser::new(),
            ipp: IPParser::new(),
            discovered_ips: Vec::new()
        }
    }

    pub fn parse_packet(&mut self, packet: SlicedPacket) {
        //println!("link: {:?}", packet.link);
        //println!("vlan: {:?}", packet.vlan);
        if let Some(p) = packet.ip {
            let (src, dst) = self.ipp.parse_ip_part(p);

            if !self.discovered_ips.iter().any(|i| i==&src) {
                println!("New IP found!");
                self.discovered_ips.push(src.clone());
            }
            if !self.discovered_ips.iter().any(|i| i==&dst) {
                println!("New IP found!");
                self.discovered_ips.push(dst.clone());
            }
        }
        if let Some(p) = packet.link {
            let (src, dst) = self.ep.parse_ethernet_part(p);
            println!("{} {}", src, dst);
        }
    }
    pub fn get_seen_ips(&self) -> Vec<String> {
        return self.discovered_ips.clone();
    }
}

pub struct EthernetParser {

}

impl EthernetParser {
    fn new() -> EthernetParser {
        EthernetParser {

        }
    }

    fn parse_ethernet_part(&self, link_slice: LinkSlice) -> (String, String) {
        return match link_slice {
            LinkSlice::Ethernet2(ether) => {
                let mut src: String = String::new();
                let mut dst: String = String::new();
                for p in ether.source() {
                    src.push_str(&*p.to_string());
                    src.push_str(":");
                }
                src.pop();
                for p in ether.destination() {
                    dst.push_str(&*p.to_string());
                    dst.push_str(":");
                }
                dst.pop();
                (src, dst)
            }
        }
    }
}

pub struct IPParser {
}

impl IPParser {
    fn new() -> IPParser {
        IPParser {
        }
    }

    fn parse_ip_part(&mut self, ip_slice: InternetSlice) -> (String, String) {
        return match ip_slice {
            InternetSlice::Ipv4(ipv4_header, ipv4_extensions) => {
                // Hier kannst du auf die IPv4-Daten zugreifen
                let mut src = String::new();
                let mut dst = String::new();
                src = ipv4_header.source_addr().to_string();
                dst = ipv4_header.destination_addr().to_string();
                (src, dst)
            }
            InternetSlice::Ipv6(ipv6_header, ipv6_extensions) => {
                let mut src = String::new();
                let mut dst = String::new();
                src = ipv6_header.source_addr().to_string();
                dst = ipv6_header.destination_addr().to_string();
                (src, dst)
            }
        }
    }
}