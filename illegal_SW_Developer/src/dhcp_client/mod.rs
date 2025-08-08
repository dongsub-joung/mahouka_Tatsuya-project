pub struct DHCPClient{
    iface: String,
    flag: bool,
    target_server: String,
}

impl DHCPClient {
    pub fn new(iface: String, flag: bool, target_server: String) -> Self{
        DHCPClient { iface, flag, target_server }
    }

    pub fn dhcp_dora(
            self, client_id: String, fqdn: String, requested_ip: String, dhcp_server: String
            , max_retry: usize, fqdn_server_flag: bool, relay_address: String) -> std::option::Option<String> {

        let mut leased_ip: Option<String>= None;

        return leased_ip;
    }
}