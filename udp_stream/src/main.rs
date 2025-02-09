use std::net::UdpSocket;

fn main() {
    #[cfg(not(any(target_os = "windows")))]
    {
        println!("Running on an unwindows operating system!");

    }

    #[cfg(target_os = "windows")]
    {
        println!("Running on Windows!");
        // Add Windows-specific functionality here.
    }
}