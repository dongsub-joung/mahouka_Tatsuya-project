use std::net::{self, IpAddr, Ipv4Addr, UdpSocket};
use device_query::{DeviceQuery, DeviceState, MouseState};

use serde::{Deserialize, Serialize};
use serialize_deserialize_u8_i32::s_d_u8_i32;
use bincode;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Position {
    x: u32,
    x_bar: bool,
    y: u32,
    y_bar: bool,
}

fn send(socket: &net::UdpSocket, receiver: &str, msg: &Vec<u8>) -> usize {

    println!("sending message: {:?}", msg);
    let result: usize = 0;
    match socket.send_to(&msg, receiver) {
        Ok(number_of_bytes) => println!("{:?}", number_of_bytes),
        Err(fail) => println!("failed sending {:?}", fail),
    }

    result
}

fn main() ->  std::io::Result<()> {
    
    #[cfg(not(any(target_os = "windows")))]
    {
        println!("Running on an un-windows operating system!");

        // about conn info
        let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let local_port= 5000;
        let remote_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let remote_port= 5000;

        // connection
        let local_ip= format!("{}:{}", localhost_v4.to_string(), local_port);
        let remote_ip= format!("{}:{}", remote_v4.to_string(), remote_port);

        let recive_socket = UdpSocket::bind(remote_ip.clone()).expect("couldn't bind to address");

        // frist conn test
        recive_socket.send_to(&[0; 10], remote_ip.clone()).expect("couldn't send data");

        loop {
            if std::io::ErrorKind::NotConnected == recive_socket.peer_addr().unwrap_err().kind() {
                println!("Conn err, rechecking");
                match recive_socket.take_error() {
                    Ok(Some(error)) => println!("UdpSocket error: {error:?}"),
                    Ok(None) => println!("No error"),
                    Err(error) => println!("UdpSocket.take_error failed: {error:?}"),
                }
            }else{
                // if broadcast
                // recive_socket.set_broadcast(true);

                recive_socket.connect(remote_ip.clone()).expect("connect function failed");

                //  set_nonblocking - https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.set_nonblocking

                // about timeout option
                let duration = std::time::Duration::new(1, 0);
                let dur = std::option::Option::Some(duration);
                recive_socket.set_read_timeout(dur).expect("failed to set timeout");

                let device_state = DeviceState::new();
                let mouse: MouseState = device_state.get_mouse();
                // println!("{:#?}", mouse.coords);

                // i32 to u8 (overhead) - Serialization
                let data= mouse.coords;

                let position= Position{
                    x: data.0 as u32,
                    x_bar: false,
                    y: data.1 as u32,
                    y_bar: false,
                };

                // https://docs.rs/serialize_deserialize_u8_i32/latest/serialize_deserialize_u8_i32/
                let encoded_u8: Vec<u8> = bincode::serialize(&position).unwrap();
                recive_socket.send_to(&encoded_u8, remote_ip.clone()).expect("couldn't send data");
            
                // receive
                //     let encoded_i32: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(encoded_u8);
            }
        }

        // If I can to that (screenshot copy, not save)
        // shift + cnt + print-f(gnome)
    }

    #[cfg(target_os = "windows")]
    {
        println!("Running on Windows!");
        // Add Windows-specific functionality here.
    }

    Ok(())
}