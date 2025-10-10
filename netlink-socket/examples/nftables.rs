//! This example demonstrates basic interactions with nftables:
//! creating/deleting chains, adding rules.
//!
//! Run with: `cargo run --example nftables --features=nftables`

use std::{ffi::CStr, net::Ipv4Addr};

use netlink_bindings::nftables::{self, CmpOps, PayloadBase, PushNfgenmsg, Registers, VerdictCode};
use netlink_socket::NetlinkSocket;

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() {
    let mut sock = netlink_socket::NetlinkSocket::new();

    let table = c"filter";
    let chain = c"example-chain";

    println!();
    println!("Appending new rule to {chain:?} chain");
    append_rule(&mut sock, table, chain).await;

    println!();
    println!("Printing the rules in the {chain:?} chain");
    dump_rules(&mut sock, table, chain).await;

    println!();
    println!("Deleting {chain:?} chain");
    del_chain(&mut sock, table, chain).await;
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn append_rule(sock: &mut NetlinkSocket, table: &CStr, chain: &CStr) {
    let id = get_latest_genid(sock).await;

    // Base sequence number for the messages in the chain. Might as well be random.
    let seq = sock.reserve_seq(256);

    // Nftables expects the whole transaction to come in a single write operation
    let mut c = nftables::Chained::new(seq);

    c.request()
        .op_batch_begin_do_request(&batch_header())
        .encode()
        .push_genid(id);

    // Create a separate table to not interfere with actual traffic
    c.request()
        .op_newchain_do_request(&msg_header())
        .encode()
        .push_table(table)
        .push_name(chain);

    // Add a new rule to the chain
    c.request()
        .set_create()
        .set_append() // append the rule to the end
        .op_newrule_do_request(&msg_header())
        .encode()
        .push_table(table)
        .push_chain(chain)

        // Add attribute containing rule expressions
        .nested_expressions()

        // Save source ip addr bytes to register 1
        .nested_elem()
        .nested_data_payload()
        .push_dreg(Registers::Reg1 as u32)
        .push_base(PayloadBase::NetworkHeader as u32)
        .push_offset(12) // ipv4 source addr
        .push_len(4)
        .end_nested()
        .end_nested()

        // Check if bytes in register 1 equal to the expected ip addr.
        .nested_elem()
        .nested_data_cmp()
        .push_sreg(Registers::Reg1 as u32)
        .push_op(CmpOps::Eq as u32)
        .nested_data()
        .push_value(&Ipv4Addr::new(1, 2, 3, 5).to_bits().to_be_bytes())
        .end_nested()
        .end_nested()
        .end_nested()

        // Accept the request (by setting the verdict register)
        .nested_elem()
        .nested_data_immediate()
        .push_dreg(Registers::RegVerdict as u32)
        .nested_data()
        .nested_verdict()
        .push_code(VerdictCode::Accept as u32)
        .end_nested()
        .end_nested()
        .end_nested()
        .end_nested()
        .end_nested()

        // ...
        ;

    c.request().op_batch_end_do_request(&batch_header());

    sock.request_chained(&c.finalize())
        .await
        .unwrap()
        .recv_all()
        .await
        .unwrap();
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn get_latest_genid(sock: &mut NetlinkSocket) -> u32 {
    let request = nftables::Request::new().op_getgen_do_request(&PushNfgenmsg::new());
    let mut iter = sock.request(&request).await.unwrap();
    let (_, attrs) = iter.recv_one().await.unwrap();

    attrs.get_id().unwrap()
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn dump_rules(sock: &mut NetlinkSocket, table: &CStr, chain: &CStr) {
    let mut request = nftables::Request::new().op_getrule_dump_request(&msg_header());
    request.encode().push_table(table).push_chain(chain);
    let mut iter = sock.request(&request).await.unwrap();
    while let Some(res) = iter.recv().await {
        println!("{res:#?}");
    }
}

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
async fn del_chain(sock: &mut NetlinkSocket, table: &CStr, chain: &CStr) {
    let genid = get_latest_genid(sock).await;

    let mut c = nftables::Chained::new(sock.reserve_seq(256));
    c.request()
        .op_batch_begin_do_request(&batch_header())
        .encode()
        .push_genid(genid);

    c.request()
        .op_delchain_do_request(&msg_header())
        .encode()
        .push_table(table)
        .push_name(chain);

    c.request().op_batch_end_do_request(&batch_header());

    let c = c.finalize();

    let mut iter = sock.request_chained(&c).await.unwrap();
    while let Some(res) = iter.recv().await {
        res.unwrap();
    }
}

fn batch_header() -> nftables::PushNfgenmsg {
    let mut h = nftables::PushNfgenmsg::new();
    h.set_res_id(10);
    h
}

fn msg_header() -> nftables::PushNfgenmsg {
    let mut h = nftables::PushNfgenmsg::new();
    h.set_nfgen_family(libc::AF_INET as u8); // aka ipv4
    h
}
