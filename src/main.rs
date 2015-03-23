#![feature(core)]
#![feature(plugin)]
#![plugin(regex_macros)]

/*
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

extern crate regex;

use std::env;

struct CIDR {
    address: u32,
    netmask: u32,
}

impl CIDR {

    fn from_string(s: String) -> Result<CIDR, &'static str> {
        let re = regex!(r"^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})/(\d{1,2})$");

        let captures = try!(re.captures(&s).ok_or("invalid cidr"));

        let mut e: Vec<u32> = Vec::new();

        let limits = [0, 255, 255, 255, 255, 32];

        for x in (1 .. 6) {
            let n = captures.at(x).unwrap().parse::<u32>().unwrap();
            if n > limits[x] {
                return  Err("invalid cidr");
            }
            e.push(n);
        }

        Ok(CIDR::from_octets(e[0], e[1], e[2], e[3], e[4]))
    }

    fn from_octets(a0: u32, a1: u32, a2: u32, a3: u32, netmask: u32) -> CIDR {
        CIDR {
            address: a0 << 24 | a1 << 16 | a2 << 8 | a3,
            netmask: 0xffffffff << (32 - netmask),
        }
    }

    fn network(&self) -> CIDR {
        CIDR { address: self.address & self.netmask, netmask: 0 }
    }

    fn netmask(&self) -> CIDR {
        CIDR { address: self.netmask, netmask: 0 }
    }

    fn broadcast(&self) -> CIDR {
        CIDR { address: self.address | !self.netmask, netmask: 0 }
    }

    fn host_min(&self) -> CIDR {
        CIDR { address: (self.address & self.netmask) + 1, netmask: 0 }
    }

    fn host_max(&self) -> CIDR {
        CIDR { address: (self.address | !self.netmask) - 1, netmask: 0 }
    }

    fn num_hosts(&self) -> u32 {
        let min = self.host_min();
        let max = self.host_max();

        max.address - min.address + 1
    }

    // helper
    fn to_string(ip: u32) -> String {
        let a = ip >> 24 & 0xff;
        let b = ip >> 16 & 0xff;
        let c = ip >> 8 & 0xff;
        let d = ip & 0xff;
        format!("{}.{}.{}.{}", a, b, c, d)
    }

    fn to_string_ip(&self) -> String {
        CIDR::to_string(self.address)
    }

    #[allow(dead_code)]
    fn to_string_ip_subnet(&self) -> String {
        format!("{}/{}", CIDR::to_string(self.address), CIDR::to_string(self.netmask))
    }

    #[allow(dead_code)]
    fn to_string_cidr(&self) -> String {
        let mut c = 32;
        let mut mask = self.netmask;

        while mask & 1 == 0 {
            mask >>= 1;
            c -= 1;
        }

        format!("{}/{}", CIDR::to_string(self.address), c)
    }

    #[allow(dead_code)]
    fn to_string_hex(&self) -> String {
        format!("{:08x}", self.address)
    }

    fn to_string_binary(&self) -> String {
        let o = self.to_octets();
        format!("{:08b}.{:08b}.{:08b}.{:08b}", o[0], o[1], o[2], o[3])
        //format!("{:032b}", self.address)
    }

    #[allow(dead_code)]
    fn to_octets(&self) -> Vec<u32> {
        vec![self.address >> 24 & 0xff, self.address >> 16 & 0xff, self.address >> 8 & 0xff, self.address & 0xff]
    }

    fn reverse_dns(&self) -> String {
        let o = self.to_octets();
        format!("{}.{}.{}.{}.in-addr.arpa", o[3], o[2], o[1], o[0])
    }
}

fn process(ip: CIDR) {
    println!("Address:   {:16} {}", ip.to_string_ip(), ip.to_string_binary());
    println!("Netmask:   {:16} {}", ip.netmask().to_string_ip(), ip.netmask().to_string_binary());
    println!("---------------------------------------------------------------");
    println!("Network:   {:16} {}", ip.network().to_string_ip(), ip.network().to_string_binary());
    println!("HostMin:   {:16} {}", ip.host_min().to_string_ip(), ip.host_min().to_string_binary());
    println!("HostMax:   {:16} {}", ip.host_max().to_string_ip(), ip.host_max().to_string_binary());
    println!("Broadcast: {:16} {}", ip.broadcast().to_string_ip(), ip.broadcast().to_string_binary());
    println!("NumHosts:  {:<16} rDNS: {}", ip.num_hosts(), ip.reverse_dns());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <cidr>", args[0]);
        return;
    }

    match CIDR::from_string(args[1].to_string()) {
        Ok(ip) => process(ip),
        Err(e) => println!("error: {}", e),
    }
}
