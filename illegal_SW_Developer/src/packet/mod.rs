use pcap::Packet;

pub struct PacketHeader {
    /// The time when the packet was captured
    pub ts: libc::timeval,
    /// The number of bytes of the packet that are available from the capture
    pub caplen: u32,
    /// The length of the packet, in bytes (which might be more than the number of bytes available
    /// from the capture, if the length of the packet is larger than the maximum number of bytes to
    /// capture)
    pub len: u32,
}

pub trait EnhancePacket {
    fn get_options(&self);
} 

pub struct PacketOverride{
    pub header: PacketHeader,
    pub data: [u8; 1024],
    pub options: Vec<&'static str>,
}
impl EnhancePacket for PacketOverride{
    fn get_options(){

    }
}