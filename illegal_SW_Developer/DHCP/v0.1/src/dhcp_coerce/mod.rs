struct Option{
    option_str: &'static str,
    short_str: &'static str,
    help_str: &'static str,
}

// "-i","--iface", "The name of the interface to use when sending packets"
pub const IFACE: Option= Option{
    option_str: "--iface",
    short_str: "-i",
    help_str: "The name of the interface to use when sending packets",
};

// "-d", "--domain-name", "The FQDN of the domain we are targeting"
pub const DOMAIN_NAME: Option= Option{
    option_str: "--domain-name",
    short_str: "-d",
    help_str: "The FQDN of the domain we are targeting",
};

// "-s", "--target-server", help="The IP address of the target DHCP server"
pub const TARGET_SERVER: Option= Option{
    option_str: "--target-server",
    short_str: "-s",
    help_str: "The IP address of the target DHCP server",
};

// "-c", "--coerce-ip", help="An IP address that is part of the DHCP coercion scope we previously created on the target server"
pub const COERE_IP: Option= Option{
    option_str: "--coerce-ip",
    short_str: "-c",
    help_str: "An IP address that is part of the DHCP coercion scope we previously created on the target server",
};

// "-ip", "--relay-ip", help="The IP address of our machine. This address needs to be part of an existing scope on the target server"
pub const RELAY_IP: Option= Option{
    option_str: "--relay-ip",
    short_str: "-ip",
    help_str: "The IP address of our machine. This address needs to be part of an existing scope on the target server",
};