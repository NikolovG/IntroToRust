use nix::libc;

fn main() {
    nix::unistd::write(libc::STDOUT_FILENO, "Hello, world!\n".as_bytes());
}
