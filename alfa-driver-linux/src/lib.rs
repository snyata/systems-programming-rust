//! - Configured for the AWUS0306ACH Network Adaptor
//! Linux Driver for Alfa Network Adaptor
//! - Aiming to provide the ability to take it up, down and into monitor mode
//! -------------
//! Chipset:
//! Interface:
//! structs include ScanResult
//! 
//mod orion_cli;

use nix::sys::socket::{self, AddressFamily, SockType, SockFlag};
use nix::unistd::close;
use std::ffi::CString;
use log::{info, error};
use std::process::Command;
use serde::Serialize;

#[derive(Serialize)]
pub struct ScanResult {
    pub timestamp: String,
    pub id: u16,
    pub ip: String,
    pub port: i32,
    pub status: String,
}

pub fn set_interface_up(iface: &str) -> Result<(), String> {
    let sock = socket::socket(AddressFamily::Inet
        SockType::Datagram, SockFlag::empty(), None)
        .map_err(|e| e.to_string())?;
    
    let iface_cstr = CString::new(iface).unwrap();
    let req = libc::ifreq {
        ifr_name: {
            let mut buf = [0; libc::IFNAMSIZ];
            for (i,b) in iface_cstr.to_bytes().iter().enumerate() {
                buf[i] = *b as i8;
            }
            buf
        },
        ifr_ifru: libc::ifru_flags(libc::IF_UP),
    };
    
    unsafe {
        if libc::ioctl(sock, libc::SIOCSIFFLAGS, &req) < 0 {
            return Err("Failed to set to up".to_string());
        }
    }

    close(sock).map_err(|e| e.to_string())?;
    info!("Interface {} brought up", iface);
}




