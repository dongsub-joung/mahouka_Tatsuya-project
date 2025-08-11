use pcap::Packet;

pub trait EnhancePacket {
    fn new(packet: pcap::Packet) -> Self;
    fn get_options(&self) -> Vec<Vec<&'static str>>;
} 


#[derive(Clone)]
pub struct PacketOverride{
    pub header: pcap::PacketHeader,
    pub data: Vec<u8>,
    pub options: Vec<Vec<&'static str>>,
}
impl EnhancePacket for PacketOverride{
    fn new(packet: pcap::Packet) -> Self{
        let v_v: Vec<Vec<&'static str>> = Vec::from(Vec::new());
        let data_v:Vec<u8>= {
            let mut v: Vec<u8>= Vec::new();
            for e in packet.data{
                v.push(*e);
            }

            v
        };
        
        PacketOverride { header: *packet.header, data: data_v, options: v_v }
    }
    fn get_options(&self) -> Vec<Vec<&'static str>> {
        self.options.clone()
    }
}