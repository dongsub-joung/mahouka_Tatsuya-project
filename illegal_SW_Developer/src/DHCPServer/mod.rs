struct DHCPServer{
    ip_address: &'static str,
    domain_name: &'static str,
    dns_servers: Vec<&'static str>,
    name_protection_status: bool,
}