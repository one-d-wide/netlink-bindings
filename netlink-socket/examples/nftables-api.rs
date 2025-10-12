//! This example demonstrates a small high-level wrapper for nftables built on
//! top of netlink-bindings.
//!
//! Run with: `cargo run --example nftables-api --features=nftables`

use std::{io, net::Ipv4Addr};

use netlink_bindings::{
    nftables::{
        self, CmpOps, PayloadBase, PushExprListAttrs, PushNfgenmsg, PushOpNewruleDoRequest,
        Registers, VerdictCode,
    },
    utils,
};
use netlink_socket2::{NetlinkSocket, ReplyError};

#[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
#[cfg_attr(feature = "tokio", tokio::main(flavor = "current_thread"))]
#[cfg_attr(feature = "smol", macro_rules_attribute::apply(smol_macros::main))]
async fn main() {
    let mut sock = netlink_socket2::NetlinkSocket::new();

    let chain = "example-api-chain";

    println!();
    println!("Appending new rule to {chain:?} chain");

    // Same as
    //   iptables -N example-api-chain
    //   iptables -A example-api-chain --src 1.2.3.4 -j ACCEPT
    let mut rules = Transaction::new(&mut sock).await.unwrap();

    rules.create_chain(chain);

    rules
        .append_rule_ipv4(chain)
        .has_source_ipv4("1.2.3.4".parse().unwrap())
        .accept();

    rules.send(&mut sock).await.unwrap();

    println!();
    println!("Running iptables -L to verify");

    print_chain(chain);

    println!();
    println!("Deleting {chain:?} chain");

    // Same as
    //   iptables -D example-api-chain
    let mut rules = Transaction::new(&mut sock).await.unwrap();

    rules.delete_chain(chain);

    rules.send(&mut sock).await.unwrap();

    println!();
    println!("Running iptables -L again");

    print_chain(chain);
}

fn print_chain(chain: &str) {
    std::process::Command::new("iptables")
        .arg("-n")
        .arg("-L")
        .arg(chain)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GenerationId(u32);

impl GenerationId {
    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
    async fn new_latest(sock: &mut NetlinkSocket) -> Result<Self, ReplyError> {
        let request = nftables::Request::new().op_getgen_do_request(&PushNfgenmsg::new());
        let mut iter = sock.request(&request).await?;
        let (_, attrs) = iter.recv_one().await?;

        Ok(GenerationId(attrs.get_id()?))
    }
}

const FILTER_TABLE: &str = "filter";

struct Transaction {
    inner: nftables::Chained<'static>,
}

impl Transaction {
    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
    async fn new(sock: &mut NetlinkSocket) -> Result<Self, ReplyError> {
        let seq = sock.reserve_seq(256);
        let genid = GenerationId::new_latest(sock).await?;

        Ok(Self::new_with_genid(seq, genid))
    }

    fn new_with_genid(seq: u32, genid: GenerationId) -> Self {
        let mut inner = nftables::Chained::new(seq);

        let mut h = nftables::PushNfgenmsg::new();
        h.set_res_id(10);

        inner
            .request()
            .op_batch_begin_do_request(&h)
            .encode()
            .push_genid(genid.0);

        Self { inner }
    }

    fn create_chain(&mut self, chain: &str) {
        let mut h = nftables::PushNfgenmsg::new();
        h.set_nfgen_family(libc::AF_INET as u8);

        // Create a separate table to not interfere with actual traffic
        self.inner
            .request()
            .op_newchain_do_request(&h)
            .encode()
            .push_table_bytes(FILTER_TABLE.as_bytes())
            .push_name_bytes(chain.as_bytes());
    }

    fn delete_chain(&mut self, chain: &str) {
        let mut h = nftables::PushNfgenmsg::new();
        h.set_nfgen_family(libc::AF_INET as u8);

        self.inner
            .request()
            .op_delchain_do_request(&h)
            .encode()
            .push_table_bytes(FILTER_TABLE.as_bytes())
            .push_name_bytes(chain.as_bytes());
    }

    fn append_rule_ipv4(&mut self, chain: &str) -> NewRuleExprs<'_> {
        self.rule_ipv4(chain, true)
    }

    fn rule_ipv4(&mut self, chain: &str, append: bool) -> NewRuleExprs<'_> {
        let mut h = nftables::PushNfgenmsg::new();
        h.set_nfgen_family(libc::AF_INET as u8); // aka ipv4

        let mut inner = self.inner.request();
        if append {
            inner = inner.set_append();
        }
        let inner = inner
            .set_create()
            .op_newrule_do_request(&h)
            .into_encoder()
            .push_table_bytes(FILTER_TABLE.as_bytes())
            .push_chain_bytes(chain.as_bytes())
            .nested_expressions();

        NewRuleExprs { inner }
    }

    #[cfg_attr(not(feature = "async"), maybe_async::maybe_async)]
    async fn send(mut self, sock: &mut NetlinkSocket) -> Result<(), ReplyError> {
        let mut h = nftables::PushNfgenmsg::new();
        h.set_res_id(10);

        self.inner.request().op_batch_end_do_request(&h);

        let c = self.inner.finalize();

        let res = sock.request_chained(&c).await?.recv_all().await;

        if let Err(err) = &res {
            if let Some(err) = err.as_io_error().raw_os_error() {
                if err == libc::ERESTART {
                    return Err(io::Error::other("Generation id is too old").into());
                }
            }
        }

        res
    }
}

type PushExprs<'a> = PushExprListAttrs<PushOpNewruleDoRequest<utils::RequestBuf<'a>>>;

struct NewRuleExprs<'a> {
    inner: PushExprs<'a>,
}

// Pretty stuff
impl<'a> NewRuleExprs<'a> {
    fn has_source_ipv4(self, ipv4: Ipv4Addr) -> Self {
        self.contains_bytes(
            &ipv4.to_bits().to_be_bytes(),
            PayloadBase::NetworkHeader,
            12, // source ipv4 offset
        )
    }

    fn accept(self) -> PushExprs<'a> {
        self.verdict(VerdictCode::Accept)
    }

    // ...
}

// Not so pretty stuff
impl<'a> NewRuleExprs<'a> {
    fn contains_bytes(mut self, bytes: &[u8], base: PayloadBase, offset: u32) -> Self {
        self.inner = self
            .inner
            // Save source ip addr bytes to register 1
            .nested_elem()
            .nested_data_payload()
            .push_dreg(Registers::Reg1 as u32)
            .push_base(base as u32)
            .push_offset(offset) // ipv4 source addr
            .push_len(bytes.len() as u32)
            .end_nested()
            .end_nested()
            // If bytes in register 1 equal to the expected ip addr
            .nested_elem()
            .nested_data_cmp()
            .push_sreg(Registers::Reg1 as u32)
            .push_op(CmpOps::Eq as u32)
            .nested_data()
            .push_value(bytes)
            .end_nested()
            .end_nested()
            .end_nested();
        self
    }

    fn verdict(self, code: VerdictCode) -> PushExprs<'a> {
        self.inner
            .nested_elem()
            .nested_data_immediate()
            .push_dreg(Registers::RegVerdict as u32)
            .nested_data()
            .nested_verdict()
            .push_code(code as u32)
            .end_nested()
            .end_nested()
            .end_nested()
            .end_nested()
    }
}
