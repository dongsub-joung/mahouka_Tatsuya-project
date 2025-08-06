struct DHCPServer{
    ip_address: &'static str,
    domain_name: &'static str,
    dns_servers: Vec<String>,
    name_protection_status: bool,
}

impl DHCPServer{
    pub fn new() -> Self{
        let ip_address= "";
        let domain_name= "";
        let dns_servers: Vec<String>= Vec::new();
        let name_protection_status: Option<bool> = None;

        Self{ ip_address, domain_name, dns_servers, name_protection_status }
    }

    fn return_str(self) -> String{
        let flag= match self.name_protection_status 
        {
            Some(b) => String::from(b),
            None => String::from("Unknown"),
        };

        return format!("
IP Address: {}
Domain Name: {}
DNS Servers: {}
Name Protection Enabled:  {}"
    , self.ip_address, self.domain_name, self.dns_servers, );
    }
}

