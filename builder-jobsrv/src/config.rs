// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::net;

pub struct Config {
    pub port: u16,
    pub listen_addr: net::Ipv4Addr,
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: 9636,
            listen_addr: net::Ipv4Addr::new(0, 0, 0, 0),
        }
    }
}
