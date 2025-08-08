use std::collections::HashMap;

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

pub struct DHCPClient{
    iface: String,
    flag: bool,
    target_server: String,
    packet_base: String,
    verbose: bool,
}

impl DHCPClient {
    pub fn new(iface: String, flag: bool, target_server: String) -> Self{
        DHCPClient { iface, flag, target_server
            , packet_base: String::new()
            , verbose: false
        }
    }

    pub fn init(self, iface: String, verbose: bool, server_ip: Option<String>){
        self.iface = iface;

        if server_ip.is_none()[
            self.packet_base = self.get_broadcast_dhcp_packet(get_if_hwaddr(iface))
        ]else{
            self.packet_base = self.get_unicast_dhcp_packet(get_if_hwaddr(iface), server_ip)
        }
            
        self.verbose = verbose;
    }

    pub fn send_release(self, client_id: String, release_addr: String, dhcp_server: String){
        // send_release maybe?
        const DHCP_RELEASE_PACKET_STR: &'static str=
        "
        Send a DHCP release packet of a specified IP address. For the release packet to work, the CID of our client must
        match the CID of the original leasing client.
        :param client_id: CID to use when sending the packet
        :param release_addr: IP address to release
        :param dhcp_server: Optionally target only a specific server. By default, all receiving servers would process the request.

        :return:
        ";

        let bootp = self._initialize_bootp_layer(release_addr, client_id);

        let dhcp_options = self._initialize_dhcp_release_options(dhcp_server);

        // division
        let packet = (self._packet_base) / (bootp) / (DHCP(options=dhcp_options));

        let iface=self.iface;
        let verbose= false;

        return sendp(packet, iface, verbose);
    }

    pub fn dhcp_dora(
            self, client_id: String, fqdn: String, requested_ip: String, dhcp_server: String
            , max_retry: usize, fqdn_server_flag: bool, relay_address: String) -> std::option::Option<String> {

        const DHCP_DORA_STR: &'static str=
        "
        Perform a DHCP DORA with a specified FQDN to invoke a DHCP DNS Dynamic Update.
        :param fqdn: Optional. The FQDN to send to the DHCP server.
        :param requested_ip: Optional. a specific IP address to request from the DHCP server.
        if the IP is not in the scope of the server or taken, a different address would be leased.
        :param dhcp_server: Optional. The specific DHCP server address to target. Without it, a broadcast is sent
        and the first server to reply would be used.
        :param max_retry: Maximum amount of retries to the DORA process.
        :param relay_address: ip address of the relay agent to use.
        :return: Return the IP address that was leased to the client, or None if the lease failed
        ";
        const ZERO_ROOP_IP: &'static str= "0.0.0.0";

        let mut leased_ip: Option<String>= None;

        let bootp = self.initialize_bootp_layer(ZERO_ROOP_IP, client_id, relay_address);


        return leased_ip;
    }
}