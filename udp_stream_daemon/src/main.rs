fn main() {
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
