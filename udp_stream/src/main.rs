use std::net::{self, IpAddr, Ipv4Addr, UdpSocket};

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

        loop {
            if std::io::ErrorKind::NotConnected == recive_socket.peer_addr().unwrap_err().kind() {
                println!("Conn err, rechecking");
                match recive_socket.take_error() {
                    Ok(Some(error)) => println!("UdpSocket error: {error:?}"),
                    Ok(None) => println!("No error"),
                    Err(error) => println!("UdpSocket.take_error failed: {error:?}"),
                }
            }else{
                // frist conn test
                recive_socket.send_to(&[0; 10], remote_ip.clone()).expect("couldn't send data");

                // if broadcast
                // recive_socket.set_broadcast(true);

                recive_socket.connect(remote_ip.clone()).expect("connect function failed");

                //  set_nonblocking - https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.set_nonblocking

                // about timeout option
                let duration = std::time::Duration::new(1, 0);
                let dur = std::option::Option::Some(duration);
                recive_socket.set_read_timeout(dur).expect("failed to set timeout");
            }
        }


        // check keeping conn
        // mouse pointer x,y + screenshot copy
        // shift + cnt + print-f(gnome)
    }

    #[cfg(target_os = "windows")]
    {
        println!("Running on Windows!");
        // Add Windows-specific functionality here.
    }

    Ok(())
}