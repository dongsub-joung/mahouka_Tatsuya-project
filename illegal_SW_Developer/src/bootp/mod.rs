pub struct Bootp{
    pub op: i32,
    pub chaddr: IpAddr,
    pub ciaddr: IpAddr,
    pub xid: i32,
    pub giaddr: IpAddr,
}

impl Bootp{
    pub fn new(op: i32, chaddr: IpAddr , ciaddr: IpAddr, xid: i32, giaddr: IpAddr) -> Self{
        Self { op, chaddr, ciaddr, xid, giaddr }
    }
}