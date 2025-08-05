use std::collections::{self, HashMap};

struct SpooferConfig{
    iface: &'static str,
    max_retry: i32,
    client_id: &'static str,
    target_server: &'static str,
    requested_ip: &'static str,
    dhcp_servers: HashMap<&'static str, DHCPServer>
}

impl SpooferConfig{
    pub fn new() -> Self {
        let iface= "";
        let max_retry= 5;
        let client_id= "";
        let target_server= "";
        let requested_ip= "";
        let dhcp_servers: HashMap<&'static str, DHCPServer>= HashMap::new();

        Self { iface, max_retry, client_id, target_server, requested_ip, dhcp_servers }
    }

    pub fn str_print(){
            text = f"""
----------------------------------------
             Running Config             
----------------------------------------
Working Interface: {self.iface}
Max Retries: {self.max_retry}
Client ID: {self.client_id}
Requested IP: {self.requested_ip}
Target Server: {self.target_server}

----------------------------------------
             DHCP Servers             
----------------------------------------
        """
    }
}