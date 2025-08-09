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
    packet_base: usize,
    verbose: bool,
}

impl DHCPClient {
    pub fn new(iface: String, flag: bool, target_server: String) -> Self{
        DHCPClient { iface, flag, target_server
            , packet_base: 9999
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
        Send a DHCP release packet of a specified IP address. For the release packet to work, the CID of our client must\n
        match the CID of the original leasing client.\n
        :param client_id: CID to use when sending the packet\n
        :param release_addr: IP address to release\n
        :param dhcp_server: Optionally target only a specific server. By default, all receiving servers would process the request.\n
\n
        :return:\n
        ";

        let bootp = self._initialize_bootp_layer(release_addr, client_id);

        let dhcp_options = self._initialize_dhcp_release_options(dhcp_server);

        // division
        let packet = (self.packet_base) / (bootp) / (DHCP(options=dhcp_options));

        let iface=self.iface;
        let verbose= false;

        return sendp(packet, iface, verbose);
    }

    pub fn dhcp_dora(
            self, client_id: String, fqdn: String, requested_ip: String, dhcp_server: String
            , max_retry: usize, fqdn_server_flag: bool, relay_address: &'static str) -> std::option::Option<String> {

        const DHCP_DORA_STR: &'static str=
        "
        Perform a DHCP DORA with a specified FQDN to invoke a DHCP DNS Dynamic Update.\n
        :param fqdn: Optional. The FQDN to send to the DHCP server.\n
        :param requested_ip: Optional. a specific IP address to request from the DHCP server.\n
        if the IP is not in the scope of the server or taken, a different address would be leased.\n
        :param dhcp_server: Optional. The specific DHCP server address to target. Without it, a broadcast is sent\n
        and the first server to reply would be used.\n
        :param max_retry: Maximum amount of retries to the DORA process.\n
        :param relay_address: ip address of the relay agent to use.\n
        :return: Return the IP address that was leased to the client, or None if the lease failed\n
        ";
        const ZERO_ROOP_IP: &'static str= "0.0.0.0";

        let mut leased_ip: Option<String>= None;

        let bootp = self.initialize_bootp_layer(ZERO_ROOP_IP, client_id, relay_address);
        
        let dhcp_discover_options = self.initialize_dhcp_discover_options(
            dhcp_server, requested_ip, relay_address
        );

        // from scapy.all import BOOTP, DHCP, IP, UDP, Ether, Packet, get_if_hwaddr, sendp
        // let dhcp_discover = DHCP(options=dhcp_discover_options);
        let discover_packet = (self.packet_base) / (bootp) / (dhcp_discover);

        let DHCP_OFFER_FILTER_RELAY= {if !relay_address.is_empty() {
            relay_address
        }else{
            DHCP_OFFER_FILTER
        }};

        let offer_packet = self.send_recv_dhcp(
            discover_packet, DHCP_OFFER_FILTER_RELAY, DHCP_TYPE_OFFER, max_retry
        );

        if offer_packet{
            let offer_addr = offer_packet[BOOTP].yiaddr;

            if offer_addr {
                let requested_addr=offer_addr.clone();
                let dhcp_server=dhcp_server.clone();
                let fqdn=fqdn.clone();
                let fqdn_server_flag=fqdn_server_flag.clone();
                let relay_address=relay_address.clone();

                let dhcp_request_options = self.initialize_dhcp_request_options(
                    requested_addr, dhcp_server, fqdn, fqdn_server_flag,
                    relay_address
                );

                let options=dhcp_request_options.clone();
                let dhcp_request = DHCP(options);

                let request_packet = (self.packet_base) / (bootp) / (dhcp_request);
                let dhcp_ack_filter_relay_value= match DHCP_ACK_FILTER_RELAY{
                    relay_address => relay_address,
                    _ => DHCP_ACK_FILTER,
                };

                let ack_packet = self.send_recv_dhcp(
                    request_packet, dhcp_ack_filter_relay_value, DHCP_TYPE_ACK, max_retry
                );

                if !ack_packet {
                    if self.verbose {
                        println!(
                            "[*] DHCP DORA didnt get ACK, need to verify record creation"
                        );
                    }
                }

                offer_addr
            }
        }

        return leased_ip;
    }
}