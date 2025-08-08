use std::collections::HashMap;

pub struct DHCPClient{
    iface: String,
    flag: bool,
    target_server: String,
}

const DHCP_TYPE_DISCOVER: &'static str = "discover";
const DHCP_TYPE_OFFER: &'static str = "offer";
const DHCP_TYPE_REQUEST: &'static str = "request";
const DHCP_TYPE_ACK: &'static str = "ack";
const DHCP_TYPE_NAK: &'static str = "nak";
const DHCP_TYPE_RELEASE: &'static str = "release";

enum DHCP_MESSAGE_TYPE {
    DHCP_TYPE_DISCOVER,
    DHCP_TYPE_OFFER,
    DHCP_TYPE_REQUEST,
    DHCP_TYPE_ACK,
    DHCP_TYPE_NAK,
}

const DHCP_OPTION_NAME_SERVER: &'static str = "name_server";
const DHCP_OPTION_DOMAIN:&'static str = "domain";
const DHCP_OPTION_MESSAGE_TYPE: &'static str = "message-type";
const DHCP_OPTION_REQUESTED_ADDRESS: &'static str = "requested_addr";
const DHCP_OPTION_SERVER_IDENTIFIER: &'static str = "server_id";
const DHCP_OPTION_PARAM_REQUEST_LIST: &'static str = "param_req_list";
const DHCP_OPTION_END: &'static str = "end";
const DHCP_OPTION_CLIENT_FQDN: &'static str = "client_FQDN";
const DHCP_OPTION_RELAY_AGENT_INFO: &'static str = "relay_agent_information";

// These filters assume that the DHCP message_type option is going to
// be the first option in the message.
// This is supposed to always be the bahavior with Microsoft DHCP server
const DHCP_OFFER_FILTER: &'static str = "udp and port 68 and (udp[247:4] = 0x63350102)";
const DHCP_ACK_FILTER: &'static str = "udp and port 68 and (udp[247:4] = 0x63350105)";

const DHCP_OFFER_FILTER_RELAY: &'static str = "udp and port 67 and (udp[247:4] = 0x63350102)";
const DHCP_ACK_FILTER_RELAY: &'static str = "udp and port 67 and (udp[247:4] = 0x63350105)";

const PACKET_SNIFF_TIMEOUT: usize = 3;

enum DHCP_OPTIONS{
    DHCP_OPTION_NAME_SERVER_option,
    DHCP_OPTION_DOMAIN_option
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