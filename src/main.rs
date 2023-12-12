mod lucyna;
mod parser;

use pcap::{Device, Packet};
use std::io::Cursor;
use etherparse::{InternetSlice, SlicedPacket};

fn main() {
    let mut l = lucyna::Lucyna::new();
    l.setup_sniffer();
    l.sniff();

}
