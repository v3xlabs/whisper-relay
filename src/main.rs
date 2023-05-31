use std::io::{Read, Write};
use std::{
    env::temp_dir,
    os::{fd::FromRawFd, unix::net::UnixListener},
    path::Path,
};

use nix::{fcntl::OFlag, sys::stat::Mode, unistd::mkfifo};

fn main() {
    
}
