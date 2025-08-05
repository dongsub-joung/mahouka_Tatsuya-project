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

    pub fn str_print(self){
            let text = format!("
----------------------------------------
             Running Config             
----------------------------------------
Working Interface: {}
Max Retries: {}
Client ID: {}
Requested IP: {}
Target Server: {}

----------------------------------------
             DHCP Servers             
----------------------------------------
        ", self.iface, self.max_retry, self.client_id, self.requested_ip, self.target_server);

        for (k,v) in self.dhcp_servers {
            println!("{}: {}", k, v);
        }
    }
}