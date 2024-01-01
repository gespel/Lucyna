use std::process::exit;
use etherparse::{InternetSlice, SlicedPacket};
use pcap::{Active, Capture, Device};
use crate::parser;

pub(crate) struct Lucyna {
    cap: Capture<Active>
}

impl Lucyna {
    pub(crate) fn new() -> Lucyna {
        Lucyna {
            cap: Device::lookup().unwrap().unwrap().open().unwrap()
        }
    }
    pub(crate) fn setup_sniffer(&self) {

    }
    pub(crate) fn sniff(&mut self) {
        let mut p = parser::LucynaParse::new();
        println!("Sniffer started!");
        while let Ok(packet) = self.cap.next_packet() {

            match SlicedPacket::from_ethernet(&packet) {
                Err(value) => println!("Err {:?}", value),
                Ok(value) => {
                    p.parse_packet(value);
                }
            }
            println!("{:?}", p.get_seen_ips());
        }
    }
}