#[cfg(any(target_os = "macos", target_os = "openbsd", target_os = "freebsd", target_os = "netbsd", target_os = "ios"))]
pub mod unix;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub mod linux;

use std::net::{IpAddr, Ipv4Addr};
use crate::interface::{self, MacAddr, Interface};

/// Structure of default Gateway information
#[derive(Clone, Debug)]
pub struct Gateway {
    pub mac_addr: MacAddr,
    pub ip_addr: IpAddr,
}

impl Gateway {
    pub fn new() -> Gateway {
        Gateway {
            mac_addr: MacAddr::zero(),
            ip_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
        }
    }
}

/// Get default Gateway
pub fn get_default_gateway() -> Result<Gateway, String> {
    let local_ip: IpAddr = match interface::get_local_ipaddr(){
        Some(local_ip) => local_ip,
        None => return Err(String::from("Local IP address not found")),
    };
    let interfaces: Vec<Interface> = interface::get_interfaces();
    for iface in interfaces {
        match local_ip {
            IpAddr::V4(local_ipv4) => {
                if iface.ipv4.contains(&local_ipv4) {
                    if let Some(gateway) = iface.gateway {
                        return Ok(gateway);
                    }
                }
            },
            IpAddr::V6(local_ipv6) => {
                if iface.ipv6.contains(&local_ipv6) {
                    if let Some(gateway) = iface.gateway {
                        return Ok(gateway);
                    }
                }
            },
        }
    }
    Err(String::from("Default Gateway not found"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default_gateway() {
        println!("{:?}", get_default_gateway());
    }
}