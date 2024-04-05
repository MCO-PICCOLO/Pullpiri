fn main() {
    let handle = std::thread::spawn(|| {
        std::process::Command::new("/usr/bin/etcd")
            .stdout(std::process::Stdio::null())
            .output()
    });

    let _ = handle.join();
}
