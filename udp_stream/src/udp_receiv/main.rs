use std::net::UdpSocket;

fn main() {
    let socket= UdpSocket::bind("0.0.0.0:4000")
        .unwrap();

    println!("server listening on {}", socket.local_addr().unwrap());

    let mut buffer= [0; 1024];

    loop{
        let (size, source) = socket.recv_from(
            &mut buffer
        ).unwrap();

        let request= String::from_utf8_lossy(&buffer[..size]);
        
        println!("Received {} from {}", request, source);

        let res= "Hello from server..!";

        socket.send_to(res.as_bytes(), source).unwrap();
    }
    #[cfg(target_os = "windows")]
    {
        println!("Running on Windows!");
        // Add Windows-specific functionality here.
    }

    #[cfg(target_os = "linux")]
    {
        println!("Running on Linux!");
        // Add Linux-specific functionality here.
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        println!("Running on an unsupported operating system!");
    }
}

