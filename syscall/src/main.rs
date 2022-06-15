use crate::libc::syscall;
use nix::libc;

fn main() {
    unsafe {
    syscall(1, libc::STDOUT_FILENO, "Hello, world!\n".as_bytes());
    }
}