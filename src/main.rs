use pcap::{Device, Packet};
use std::io::Cursor;
use etherparse::{InternetSlice, SlicedPacket};

fn main() {
    let mut cap = Device::lookup().unwrap().unwrap().open().unwrap();

    // Lese ein Paket aus der pcap-Datei
    while let Ok(packet) = cap.next_packet() {

        match SlicedPacket::from_ethernet(&packet) {
            Err(value) => println!("Err {:?}", value),
            Ok(value) => {
                /*println!("link: {:?}", value.link);
                println!("vlan: {:?}", value.vlan);
                println!("ip: {:?}", value.ip);
                println!("transport: {:?}", value.transport);*/

                if let Some(ip_slice) = value.ip {
                    match ip_slice {
                        InternetSlice::Ipv4(ipv4_header, ipv4_extensions) => {
                            // Hier kannst du auf die IPv4-Daten zugreifen
                            println!("IPv4: {:?}", ipv4_header);
                        }
                        InternetSlice::Ipv6(ipv6_header, ipv6_extensions) => {
                            // Hier kannst du auf die IPv6-Daten zugreifen
                            println!("IPv6: {:?}", ipv6_header);
                        }
                    }
                }
            }
        }

    }
}
