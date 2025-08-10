use std::collections::HashMap;
use clap::builder::Str;
use rand::prelude::*;
use std::{thread, time::Duration};

use crate::dhcp_server::DHCPServer;

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
impl DHCP_MESSAGE_TYPE {
    fn from_index(index: usize) -> Option<DHCP_MESSAGE_TYPE> {
            match index {
                0 => Some(DHCP_MESSAGE_TYPE::DHCP_TYPE_DISCOVER),
                1 => Some(DHCP_MESSAGE_TYPE::DHCP_TYPE_OFFER),
                2 => Some(DHCP_MESSAGE_TYPE::DHCP_TYPE_REQUEST),
                3 => Some(DHCP_MESSAGE_TYPE::DHCP_TYPE_ACK),
                4 => Some(DHCP_MESSAGE_TYPE::DHCP_TYPE_NAK),
                _ => None, // Handle invalid index
            }
    }
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
        const DHCP_RELEASE_PACKET_STR_comment: &'static str=
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

        const DHCP_DORA_STR_comment: &'static str=
        "\n
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
            // from scapy.all import BOOTP, DHCP, IP, UDP, Ether, Packet, get_if_hwaddr, sendp
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
                // from scapy.all import BOOTP, DHCP, IP, UDP, Ether, Packet, get_if_hwaddr, sendp
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

    pub fn initialize_bootp_layer(
        self, client_address: String, client_id: usize, relay_address: String
    ) -> BOOTP{
        let relay_address_comment= String::from("\n
        initialize a scapy BOOTP layer for our packets\n
        :param client_address: IP address of the client\n
        :param client_id: MAC address of the client\n
        :param relay_address: ip address of the relay agent to use.\n
        :return: BOOTP object with the specified data\n
        \n");
        
        if !relay_address.is_empty() {
            let op= 1;
            let chaddr= binascii.unhexlify(client_id);
            let ciaddr=client_address;
            let xid= generate_random();
            let giaddr= relay_address;

            // from scapy.all import BOOTP, DHCP, IP, UDP, Ether, Packet, get_if_hwaddr, sendp
            return BOOTP(
                op,
                chaddr,
                ciaddr,
                xid,
                giaddr,
            );
        }else{
            let op= 1;
            let chaddr= binascii.unhexlify(client_id);
            let ciaddr= client_address;
            let xid= generate_random();

            return BOOTP(
                op,
                chaddr,
                ciaddr,
                xid,
            );
        }
    }

    pub fn initialize_dhcp_discover_options(
        self,
        dhcp_server: &'static str,
        requested_ip: &'static str,
        param_req_list: Vec<&'static str>,
        relay_address: &'static str,
    )
    -> Vec<(&'static str, &'static str)>{

        let discovery_packet_comment= "\n
        Initialize the DHCP options for a Discover packet\n
        :param dhcp_server: IP address of the target server, would be used in the server_id/ option\n
        :param requested_ip: Requested IP address, would be used in the requested_ip option\n
        :param param_req_list: List of params to request from the DHCP server, would be used in the param_req_list option\n
        :param relay_address: ip address of the relay agent to use.\n
        :return: List containing DHCP options in the expected format for scapy\n
        \n";

        let mut dhcp_options= Vec::from([(DHCP_OPTION_MESSAGE_TYPE, DHCP_TYPE_DISCOVER)]);

        if !dhcp_server.is_empty(){
            dhcp_options.push((DHCP_OPTION_SERVER_IDENTIFIER, dhcp_server));
        }

        if !requested_ip.is_empty() {
            dhcp_options.push((DHCP_OPTION_REQUESTED_ADDRESS, requested_ip));
        }

        if !param_req_list.is_empty() {
            // Request the domain name and configured name servers from the DHCP servers.
            // [DHCP_OPTIONS[param] for param in param_req_list]
            let DHCP_OPTIONS: Vec<String>= param_req_list.iter().map(|f| String::from(f)).collect();
            let dhcp_options_str= make_from_vector_to_string(DHCP_OPTIONS);
            dhcp_options.push(
                (DHCP_OPTION_PARAM_REQUEST_LIST, &dhcp_options_str),
            );
        }

        if !relay_address.is_empty() {
            // 0x05 is sub-option 5, 0x04 is length of the data - 4 bytes representing an IP address
            let option82: &'static str = b"\x05\x04" + ip_to_bytes(requested_ip);
            dhcp_options.push((DHCP_OPTION_RELAY_AGENT_INFO, option82));
        }
    
        dhcp_options.push((DHCP_OPTION_END, ""));

        return dhcp_options;
    }

    pub fn initialize_dhcp_request_options(
        self,
        requested_addr: &'static str,
        dhcp_server: &'static str,
        fqdn: String,
        fqdn_server_flag: bool,
        relay_address: String,
    ) -> Vec<(&'static str, &'static str)>{

        let init_dhcp_options_comment= ("\n
        Initialize the DHCP options for a Request packet\n
        :param requested_addr: Requested IP address, would be used in the requested_ip option\n
        :param dhcp_server: IP address of the target server, would be used in the server_id option\n
        :param fqdn: FQDN of the client, would be used in the Client_FQDN option.\n
        :param fqdn_server_flag: set the server flag in the FQDN option to True or False.\n
        :param relay_address: ip address of the relay agent to use.\n
        :return: List containing DHCP options in the expected format for scapy\n
        \n");

        let mut dhcp_options = Vec::from([
            (DHCP_OPTION_MESSAGE_TYPE, DHCP_TYPE_REQUEST),
            (DHCP_OPTION_REQUESTED_ADDRESS, requested_addr),
        ]);

        if !dhcp_server.is_empty() {
            dhcp_options.push((DHCP_OPTION_SERVER_IDENTIFIER, dhcp_server));
        }

        if !fqdn.is_empty() {
            let fqdn_flags= if fqdn_server_flag {
                b"\x01\x00\x00"
            }else{
                b"\x00\x00\x00"
            };


            // These are the flags of the FQDN option. in this case, only the Server flag is set,
            // to indicate that the server should create a record on behalf of the client.
            dhcp_options.push((
                DHCP_OPTION_CLIENT_FQDN,
                &format!("{}, {}", fqdn_flags, bytes(fqdn, "utf-8")),
            ));
        }

        if !relay_address.is_empty() {
            // 0x05 is sub-option 5, 0x04 is length of the data - 4 bytes representing an IP address
            let option82 = b"\x05\x04" + ip_to_bytes(requested_addr);

            dhcp_options.push((DHCP_OPTION_RELAY_AGENT_INFO, option82));
        }

        dhcp_options.push((DHCP_OPTION_END, ""));

        dhcp_options
    }

    pub fn initialize_dhcp_release_options(self, dhcp_server: &'static str) -> Vec<(&'static str, &'static str)>{
        
        let comment= "\n
        Initialize the DHCP options for a Release packet\n
        :param dhcp_server: IP address of the target server, would be used in the server_id option\n
        :return: List containing DHCP options in the expected format for scapy\n
        \n";

        let mut dhcp_options = Vec::from([(DHCP_OPTION_MESSAGE_TYPE, DHCP_TYPE_RELEASE)]);

        if !dhcp_server.is_empty() {
           dhcp_options.push((DHCP_OPTION_SERVER_IDENTIFIER, dhcp_server));
        }

        dhcp_options.push((DHCP_OPTION_END, ""));

        return dhcp_options;
    }

    pub fn send_recv_dhcp(
         self, packet: Packet, recv_filter: String, recv_type: usize, max_retry: usize
    ) -> Packet{

        let comment= "\n
        Send a DHCP packet and recieve the expected response from the server\n
        :param packet: scapy Packet to send\n
        :param recv_filter: BPF filter for the expected reply for our packet\n
        :param recv_type: the DHCP type of the packet that we expect to recieve\n
        :param max_retry: max times to attempt to re-send the packet if a response is not captured\n
        :return: the response packet that was captured\n
        \n";

        let mut retry= 0;
        while retry <= max_retry {
            retry += 1

            let ret_packets: Vec<Packet> = send_recv_with_filter(
                packet, recv_filter, PACKET_SNIFF_TIMEOUT, self.iface
            );

            // if not ret_packets:
            if !ret_packets.is_empty() {
                if self.verbose {
                    println!("[*] DHCP DORA didnt get {}, retrying", recv_type);
                }
                continue;
            }

            for packet in ret_packets {
                let message_type_option = get_dhcp_option(packet,
                    DHCP_OPTION_MESSAGE_TYPE
                )[0];

                let dhcp_msg_type= DHCP_MESSAGE_TYPE::from_index(recv_type);
                if message_type_option == dhcp_msg_type.into() {
                    return packet;
                }
            }
        }
    }

    pub fn discover_dhcp_servers(
        self, client_id: usize, max_retry: usize
    ) -> HashMap<String, DHCPServer>{
        let comment= "
        Identifies all DHCP servers in the LAN and extracts useful data about them.\n
        :param client_id: the id to use in the Discover packets sent\n
        :param max_retry: Amount of Discover packets to send before returning.\n
        :return: A Dictionary with data regarding the DHCP servers found.\n
        \n";

        let bootp = self.initialize_bootp_layer("0.0.0.0", client_id, String::new());

        let dhcp_discover_options = self.initialize_dhcp_discover_options(
            "", "",
            Vec::from([DHCP_OPTION_NAME_SERVER, DHCP_OPTION_DOMAIN]),
            ""
        );

        let dhcp_discover = DHCP(dhcp_discover_options);

        let discover_packet = self.packet_base / bootp / dhcp_discover;

        let mut dhcp_servers: Vec<usize> = Vec::new();
    
        let filter = DHCP_OFFER_FILTER;

        for i in 0..max_retry {
            let ret_packets = send_recv_with_filter(
                discover_packet, filter, PACKET_SNIFF_TIMEOUT, self.iface
            );

            for packet in ret_packets {
                let message_type_option = get_dhcp_option(packet,DHCP_OPTION_MESSAGE_TYPE)[0];

                if message_type_option == DHCP_MESSAGE_TYPE::DHCP_TYPE_OFFER {
                    let dhcp_server_ip = packet[BOOTP].siaddr;
                    if !dhcp_servers.contain(dhcp_server_ip) {
                        // dhcp_servers[dhcp_server_ip] = self.parse_dhcp_server_offer_params(packet);
                        dhcp_servers= add_element_in_vector(dhcp_server_ip, dhcp_servers, self.parse_dhcp_server_offer_params(packet));
                        
                        // Remove the servers we already found from the filtering. this makes the capture more accurate.
                        let filter = format!(" and not ip host {}",dhcp_server_ip);
                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }
        }

        dhcp_servers

    }
}

fn add_element_in_vector(idx: usize, mut v: Vec<usize>, add_element: usize) -> Vec<usize>{
    for (i, e) in v.iter().enumerate() {
        if idx == i {
            v[i]= add_element;
        }
    }

    v
}

fn generate_random() -> i32{
    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (0..9999).collect();
    nums.shuffle(&mut rng);
    
    nums.choose(&mut rng).unwrap().clone()
}

fn make_from_vector_to_string(v: Vec<String>) -> String{
    let mut result_str= String::new();
    for e in v {
        result_str.push_str(&e);
    }

    result_str
}