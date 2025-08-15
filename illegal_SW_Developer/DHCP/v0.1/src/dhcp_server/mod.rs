use utf8_decode::Decoder;

#[derive(Debug)]
pub struct DHCPServer{
    pub ip_address: &'static str,
    pub domain_name:  Decoder<_>,
    pub dns_servers: Vec<String>,
    pub name_protection_status: Option<bool>,
}

impl DHCPServer{
    pub fn new() -> Self{
        let ip_address= "";
        let domain_name= "";
        let dns_servers: Vec<String>= Vec::new();
        let name_protection_status: Option<bool> = None;

        Self{ ip_address, domain_name, dns_servers, name_protection_status }
    }

    pub fn get_dhcp_server_str(self) -> String{
        let v: Option<bool>= self.name_protection_status.into();
        let flag= match v{
            Some(b) => format!("{}" , b),
            None => String::from("Unknown"),
        };

        return format!("
IP Address: {}
Domain Name: {}
DNS Servers: {:?}
Name Protection Enabled:  {}"
    , self.ip_address, self.domain_name, self.dns_servers, flag);
    }

    pub fn as_dict(self) -> Self{
        let name_protection_status= {if self.name_protection_status.is_none() {
            String::from("Unknown")
        }else{
            String::from(self.name_protection_status.into())
        }};
        DHCPServer { 
            ip_address: self.ip_address
            , domain_name: self.domain_name
            , dns_servers: self.dns_servers
            , name_protection_status: self.name_protection_status 
        }
    }
}

