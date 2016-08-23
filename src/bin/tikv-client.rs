// Copyright 2016 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(plugin)]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate tikv;
extern crate getopts;
extern crate protobuf;
extern crate kvproto;
extern crate rocksdb;
extern crate toml;
#[macro_use]
extern crate log;

use std::{env, str, u64};
use std::sync::Arc;
use std::fs::{self, File};
use std::collections::HashMap;
use std::io::Read;
use getopts::{Options, Matches};
use protobuf::Message;
use tikv::pd::{new_rpc_client, RpcClient, PdClient};
use kvproto::kvrpcpb::{Request, Response};

/// # Command line client for TiKV
///
/// Useful for simple KV operations like get, set, CAS

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("i", "cluster-id", "cluster ID for raft", "");
    opts.optopt("p", "pd", "placement driver endpoints", "");
    opts.optopt("k", "key", "key", "");
    opts.optopt("v", "value", "value", "");
    opts.optopt("e", "existing-value", "existing value for CAS requests", "");
    let matches = opts.parse(&args[1..]).expect("opts parse failed");
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let pd_endpoints = matches.opt_str("pd").unwrap();
    let cluster_id_str = matches.opt_str("cluster-id").unwrap();
    let cluster_id = u64::from_str_radix(&cluster_id_str, 10).expect("invalid cluster id");
    let pd_client = new_rpc_client(&pd_endpoints, cluster_id).unwrap();

    let op = matches.opt_str("op").unwrap();
    let key = matches.opt_str("key").unwrap();
    let value = matches.opt_str("value");
    let existing_value = matches.opt_str("existing-value");

    let req = match op.as_ref() {
        "get" => get_req(key.to_owned()),
        "set" => set_req(key.to_owned(), value.unwrap()),
        "cas" => cas_req(key.to_owned(), existing_value.unwrap(), value.unwrap()),
        _ => panic!("bad op, must be get/set/cas"),
    };

    let region = pd_client.get_region(key.as_bytes()).unwrap();
    let peers = region.get_peers();
    let peer = peers.iter().nth(0).unwrap();
    let store = pd_client.get_store(peer.get_store_id()).unwrap();
    let address = store.get_address();
}

fn get_req(key: String) -> Request {
    Request::default()
}

fn set_req(key: String, value: String) -> Request {
    Request::default()
}

fn cas_req(key: String, existing_value: String, value: String) -> Request {
    Request::default()
}
