use etherparse::{InternetSlice, SlicedPacket};
use crate::lucyna::Lucyna;

pub struct LucynaParse {
    ip: IPParser,
}

impl LucynaParse {
    pub fn new() -> LucynaParse {
        LucynaParse {
            ip: IPParser {},
        }
    }

    pub fn parse_packet(&self, packet: SlicedPacket) {
        println!("link: {:?}", packet.link);
        println!("vlan: {:?}", packet.vlan);
        if let Some(p) = packet.ip {
            self.ip.parse_ip_part(p);
        }

        //println!("ip: {:?}", packet.ip);
        println!("transport: {:?}", packet.transport);
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

}

impl IPParser {
    fn new() -> IPParser {
        IPParser {

        }
    }

    fn parse_ip_part(&self, ip_slice: InternetSlice) {
        match ip_slice {
            InternetSlice::Ipv4(ipv4_header, ipv4_extensions) => {
                // Hier kannst du auf die IPv4-Daten zugreifen
                println!("IPv4: {:?}", ipv4_header.source_addr());
            }
            InternetSlice::Ipv6(ipv6_header, ipv6_extensions) => {
                // Hier kannst du auf die IPv6-Daten zugreifen
                println!("IPv6: {:?}", ipv6_header);
            }
        }
    }
}