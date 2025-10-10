
# Operation "newlink"

## Do (request)

```rust
PushOpNewlinkDoRequest::new(&mut vec)
  .push_ifname(val) // &CStr
  .push_ifname_bytes(val) // &[u8]
  .push_net_ns_pid(val) // u32
  .push_net_ns_fd(val) // u32
  .push_target_netnsid(val) // i32
  .push_link_netnsid(val) // i32
  .nested_linkinfo()
    .push_kind(val) // &CStr
    .push_kind_bytes(val) // &[u8]
    .nested_data_bond()
      .push_mode(val) // u8
      .push_active_slave(val) // u32
      .push_miimon(val) // u32
      .push_updelay(val) // u32
      .push_downdelay(val) // u32
      .push_use_carrier(val) // u8
      .push_arp_interval(val) // u32
      .array_arp_ip_target()
        .entry(val) // Ipv4Addr
      .end_array()
      .push_arp_validate(val) // u32
      .push_arp_all_targets(val) // u32
      .push_primary(val) // u32
      .push_primary_reselect(val) // u8
      .push_fail_over_mac(val) // u8
      .push_xmit_hash_policy(val) // u8
      .push_resend_igmp(val) // u32
      .push_num_peer_notif(val) // u8
      .push_all_slaves_active(val) // u8
      .push_min_links(val) // u32
      .push_lp_interval(val) // u32
      .push_packets_per_slave(val) // u32
      .push_ad_lacp_rate(val) // u8
      .push_ad_select(val) // u8
      .nested_ad_info()
        .push_aggregator(val) // u16
        .push_num_ports(val) // u16
        .push_actor_key(val) // u16
        .push_partner_key(val) // u16
        .push_partner_mac(val) // &[u8]
      .end_nested()
      .push_ad_actor_sys_prio(val) // u16
      .push_ad_user_port_key(val) // u16
      .push_ad_actor_system(val) // &[u8]
      .push_tlb_dynamic_lb(val) // u8
      .push_peer_notif_delay(val) // u32
      .push_ad_lacp_active(val) // u8
      .push_missed_max(val) // u8
      .array_ns_ip6_target()
        .entry(val) // &[u8]
      .end_array()
      .push_coupled_control(val) // u8
    .end_nested()
    .nested_data_bridge()
      .push_forward_delay(val) // u32
      .push_hello_time(val) // u32
      .push_max_age(val) // u32
      .push_ageing_time(val) // u32
      .push_stp_state(val) // u32
      .push_priority(val) // u16
      .push_vlan_filtering(val) // u8
      .push_vlan_protocol(val) // u16
      .push_group_fwd_mask(val) // u16
      .push_root_id(val) // PushIflaBridgeId
      .push_bridge_id(val) // PushIflaBridgeId
      .push_root_port(val) // u16
      .push_root_path_cost(val) // u32
      .push_topology_change(val) // u8
      .push_topology_change_detected(val) // u8
      .push_hello_timer(val) // u64
      .push_tcn_timer(val) // u64
      .push_topology_change_timer(val) // u64
      .push_gc_timer(val) // u64
      .push_group_addr(val) // &[u8]
      .push_fdb_flush(val) // &[u8]
      .push_mcast_router(val) // u8
      .push_mcast_snooping(val) // u8
      .push_mcast_query_use_ifaddr(val) // u8
      .push_mcast_querier(val) // u8
      .push_mcast_hash_elasticity(val) // u32
      .push_mcast_hash_max(val) // u32
      .push_mcast_last_member_cnt(val) // u32
      .push_mcast_startup_query_cnt(val) // u32
      .push_mcast_last_member_intvl(val) // u64
      .push_mcast_membership_intvl(val) // u64
      .push_mcast_querier_intvl(val) // u64
      .push_mcast_query_intvl(val) // u64
      .push_mcast_query_response_intvl(val) // u64
      .push_mcast_startup_query_intvl(val) // u64
      .push_nf_call_iptables(val) // u8
      .push_nf_call_ip6tables(val) // u8
      .push_nf_call_arptables(val) // u8
      .push_vlan_default_pvid(val) // u16
      .push_pad(val) // &[u8]
      .push_vlan_stats_enabled(val) // u8
      .push_mcast_stats_enabled(val) // u8
      .push_mcast_igmp_version(val) // u8
      .push_mcast_mld_version(val) // u8
      .push_vlan_stats_per_port(val) // u8
      .push_multi_boolopt(val) // PushBrBooloptMulti
      .push_mcast_querier_state(val) // &[u8]
      .push_fdb_n_learned(val) // u32
      .push_fdb_max_learned(val) // u32
    .end_nested()
    .nested_data_erspan()
      .push_link(val) // u32
      .push_iflags(val) // u16
      .push_oflags(val) // u16
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_pmtudisc(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_ignore_df(val) // u8
      .push_fwmark(val) // u32
      .push_erspan_index(val) // u32
      .push_erspan_ver(val) // u8
      .push_erspan_dir(val) // u8
      .push_erspan_hwid(val) // u16
    .end_nested()
    .nested_data_gre()
      .push_link(val) // u32
      .push_iflags(val) // u16
      .push_oflags(val) // u16
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_pmtudisc(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_ignore_df(val) // u8
      .push_fwmark(val) // u32
      .push_erspan_index(val) // u32
      .push_erspan_ver(val) // u8
      .push_erspan_dir(val) // u8
      .push_erspan_hwid(val) // u16
    .end_nested()
    .nested_data_gretap()
      .push_link(val) // u32
      .push_iflags(val) // u16
      .push_oflags(val) // u16
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_pmtudisc(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_ignore_df(val) // u8
      .push_fwmark(val) // u32
      .push_erspan_index(val) // u32
      .push_erspan_ver(val) // u8
      .push_erspan_dir(val) // u8
      .push_erspan_hwid(val) // u16
    .end_nested()
    .nested_data_ip6gre()
      .push_link(val) // u32
      .push_iflags(val) // u16
      .push_oflags(val) // u16
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_fwmark(val) // u32
      .push_erspan_index(val) // u32
      .push_erspan_ver(val) // u8
      .push_erspan_dir(val) // u8
      .push_erspan_hwid(val) // u16
    .end_nested()
    .nested_data_geneve()
      .push_id(val) // u32
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_port(val) // u16
      .push_collect_metadata(val) // ()
      .push_remote6(val) // &[u8]
      .push_udp_csum(val) // u8
      .push_udp_zero_csum6_tx(val) // u8
      .push_udp_zero_csum6_rx(val) // u8
      .push_label(val) // u32
      .push_ttl_inherit(val) // u8
      .push_df(val) // u8
      .push_inner_proto_inherit(val) // ()
      .push_port_range(val) // PushIflaGenevePortRange
    .end_nested()
    .nested_data_ipip()
      .push_link(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u16
      .push_proto(val) // u8
      .push_pmtudisc(val) // u8
      .push_6rd_prefix(val) // &[u8]
      .push_6rd_relay_prefix(val) // &[u8]
      .push_6rd_prefixlen(val) // u16
      .push_6rd_relay_prefixlen(val) // u16
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_ip6tnl()
      .push_link(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_proto(val) // u8
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_sit()
      .push_link(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u16
      .push_proto(val) // u8
      .push_pmtudisc(val) // u8
      .push_6rd_prefix(val) // &[u8]
      .push_6rd_relay_prefix(val) // &[u8]
      .push_6rd_prefixlen(val) // u16
      .push_6rd_relay_prefixlen(val) // u16
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_tun()
      .push_owner(val) // u32
      .push_group(val) // u32
      .push_type(val) // u8
      .push_pi(val) // u8
      .push_vnet_hdr(val) // u8
      .push_persist(val) // u8
      .push_multi_queue(val) // u8
      .push_num_queues(val) // u32
      .push_num_disabled_queues(val) // u32
    .end_nested()
    .nested_data_vlan()
      .push_id(val) // u16
      .push_flags(val) // PushIflaVlanFlags
      .nested_egress_qos()

        // Attribute may repeat multiple times (treat it as array)
        .push_mapping(val) // PushIflaVlanQosMapping
      .end_nested()
      .nested_ingress_qos()

        // Attribute may repeat multiple times (treat it as array)
        .push_mapping(val) // PushIflaVlanQosMapping
      .end_nested()

      // Associated type: "VlanProtocols" (enum)
      .push_protocol(val) // u16
    .end_nested()
    .nested_data_vrf()
      .push_table(val) // u32
    .end_nested()
    .nested_data_vti()
      .push_link(val) // u32
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_vti6()
      .push_link(val) // u32
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_netkit()
      .push_peer_info(val) // &[u8]
      .push_primary(val) // u8

      // Associated type: "NetkitPolicy" (enum)
      .push_policy(val) // u32

      // Associated type: "NetkitPolicy" (enum)
      .push_peer_policy(val) // u32

      // Associated type: "NetkitMode" (enum)
      .push_mode(val) // u32

      // Associated type: "NetkitScrub" (enum)
      .push_scrub(val) // u32

      // Associated type: "NetkitScrub" (enum)
      .push_peer_scrub(val) // u32
      .push_headroom(val) // u16
      .push_tailroom(val) // u16
    .end_nested()
    .nested_data_ovpn()

      // Associated type: "OvpnMode" (enum)
      .push_mode(val) // u8
    .end_nested()
    .push_xstats(val) // &[u8]
    .push_slave_kind(val) // &CStr
    .push_slave_kind_bytes(val) // &[u8]
    .nested_slave_data_bridge()
      .push_state(val) // u8
      .push_priority(val) // u16
      .push_cost(val) // u32
      .push_mode(val) // ()
      .push_guard(val) // ()
      .push_protect(val) // ()
      .push_fast_leave(val) // ()
      .push_learning(val) // ()
      .push_unicast_flood(val) // ()
      .push_proxyarp(val) // ()
      .push_learning_sync(val) // ()
      .push_proxyarp_wifi(val) // ()
      .push_root_id(val) // PushIflaBridgeId
      .push_bridge_id(val) // PushIflaBridgeId
      .push_designated_port(val) // u16
      .push_designated_cost(val) // u16
      .push_id(val) // u16
      .push_no(val) // u16
      .push_topology_change_ack(val) // u8
      .push_config_pending(val) // u8
      .push_message_age_timer(val) // u64
      .push_forward_delay_timer(val) // u64
      .push_hold_timer(val) // u64
      .push_flush(val) // ()
      .push_multicast_router(val) // u8
      .push_pad(val) // &[u8]
      .push_mcast_flood(val) // ()
      .push_mcast_to_ucast(val) // ()
      .push_vlan_tunnel(val) // ()
      .push_bcast_flood(val) // ()
      .push_group_fwd_mask(val) // u16
      .push_neigh_suppress(val) // ()
      .push_isolated(val) // ()
      .push_backup_port(val) // u32
      .push_mrp_ring_open(val) // ()
      .push_mrp_in_open(val) // ()
      .push_mcast_eht_hosts_limit(val) // u32
      .push_mcast_eht_hosts_cnt(val) // u32
      .push_locked(val) // ()
      .push_mab(val) // ()
      .push_mcast_n_groups(val) // u32
      .push_mcast_max_groups(val) // u32
      .push_neigh_vlan_suppress(val) // ()
      .push_backup_nhid(val) // u32
    .end_nested()
    .nested_slave_data_bond()
      .push_state(val) // u8
      .push_mii_status(val) // u8
      .push_link_failure_count(val) // u32
      .push_perm_hwaddr(val) // &[u8]
      .push_queue_id(val) // u16
      .push_ad_aggregator_id(val) // u16
      .push_ad_actor_oper_port_state(val) // u8
      .push_ad_partner_oper_port_state(val) // u16
      .push_prio(val) // u32
    .end_nested()
  .end_nested()
  .push_group(val) // u32
  .push_num_tx_queues(val) // u32
  .push_num_rx_queues(val) // u32
  .push_address(val) // &[u8]
  .push_broadcast(val) // &[u8]
  .push_mtu(val) // u32
  .push_txqlen(val) // u32
  .push_operstate(val) // u8
  .push_linkmode(val) // u8
  .push_gso_max_size(val) // u32
  .push_gso_max_segs(val) // u32
  .push_gro_max_size(val) // u32
  .push_gso_ipv4_max_size(val) // u32
  .push_gro_ipv4_max_size(val) // u32
  .nested_af_spec()
    .nested_inet()

      // u32 indexed by ipv4-devconf - 1 on output, on input it's a nest
      .push_conf(val) // &[u8]
    .end_nested()
    .nested_inet6()
      .push_flags(val) // u32

      // u32 indexed by ipv6-devconf - 1 on output, on input it's a nest
      .push_conf(val) // &[u8]
      .push_stats(val) // &[u8]
      .push_mcast(val) // &[u8]
      .push_cacheinfo(val) // PushIflaCacheinfo
      .push_icmp6stats(val) // &[u8]
      .push_token(val) // &[u8]
      .push_addr_gen_mode(val) // u8
      .push_ra_mtu(val) // u32
    .end_nested()
    .nested_mctp()
      .push_net(val) // u32
      .push_phys_binding(val) // u8
    .end_nested()
  .end_nested()
  ;
```

### Do (reply)

```rust
let attrs = OpNewlinkDoReply::new(buf);

// No attributes
```

# Operation "dellink"

## Do (request)

```rust
PushOpDellinkDoRequest::new(&mut vec)
  .push_ifname(val) // &CStr
  .push_ifname_bytes(val) // &[u8]
  ;
```

### Do (reply)

```rust
let attrs = OpDellinkDoReply::new(buf);

// No attributes
```

# Operation "getlink"

## Do (request)

```rust
PushOpGetlinkDoRequest::new(&mut vec)
  .push_ifname(val) // &CStr
  .push_ifname_bytes(val) // &[u8]
  .push_alt_ifname(val) // &CStr
  .push_alt_ifname_bytes(val) // &[u8]

  // Associated type: "RtextFilter" (1 bit per enumeration)
  .push_ext_mask(val) // u32
  .push_target_netnsid(val) // i32
  ;
```

### Do (reply)

```rust
let attrs = OpGetlinkDoReply::new(buf);

attrs.get_address(); // &[u8]
attrs.get_broadcast(); // &[u8]
attrs.get_ifname(); // &CStr
attrs.get_mtu(); // u32
attrs.get_link(); // u32
attrs.get_qdisc(); // &CStr
attrs.get_stats(); // PushRtnlLinkStats
attrs.get_cost(); // &CStr
attrs.get_priority(); // &CStr
attrs.get_master(); // u32
attrs.get_wireless(); // &CStr
attrs.get_protinfo(); // &CStr
attrs.get_txqlen(); // u32
attrs.get_map(); // PushRtnlLinkIfmap
attrs.get_weight(); // u32
attrs.get_operstate(); // u8
attrs.get_linkmode(); // u8
{ // Nested Linkinfo
  let attrs = attrs.get_linkinfo();
  attrs.get_kind(); // &CStr
  attrs.get_data(); // submessage
  attrs.get_xstats(); // &[u8]
  attrs.get_slave_kind(); // &CStr
  attrs.get_slave_data(); // submessage
}
attrs.get_net_ns_pid(); // u32
attrs.get_ifalias(); // &CStr
attrs.get_num_vf(); // u32
{ // Nested VfinfoList
  let attrs = attrs.get_vfinfo_list();
  { // Nested Info

    // Attribute may repeat multiple times (treat it as array)
    for entry in attrs.get_info() {
      entry.get_mac(); // PushIflaVfMac
      entry.get_vlan(); // PushIflaVfVlan
      entry.get_tx_rate(); // PushIflaVfTxRate
      entry.get_spoofchk(); // PushIflaVfSpoofchk
      entry.get_link_state(); // PushIflaVfLinkState
      entry.get_rate(); // PushIflaVfRate
      entry.get_rss_query_en(); // PushIflaVfRssQueryEn
      { // Nested Stats
        let attrs = entry.get_stats();
        attrs.get_rx_packets(); // u64
        attrs.get_tx_packets(); // u64
        attrs.get_rx_bytes(); // u64
        attrs.get_tx_bytes(); // u64
        attrs.get_broadcast(); // u64
        attrs.get_multicast(); // u64
        attrs.get_pad(); // &[u8]
        attrs.get_rx_dropped(); // u64
        attrs.get_tx_dropped(); // u64
      }
      entry.get_trust(); // PushIflaVfTrust
      entry.get_ib_node_guid(); // PushIflaVfGuid
      entry.get_ib_port_guid(); // PushIflaVfGuid
      { // Nested VlanList
        let attrs = entry.get_vlan_list();

        // Attribute may repeat multiple times (treat it as array)
        for entry in attrs.get_info() {
          entry; // PushIflaVfVlanInfo
        }
      }
      entry.get_broadcast(); // &[u8]
    }
  }
}
attrs.get_stats64(); // PushRtnlLinkStats64
{ // Nested VfPorts
  let attrs = attrs.get_vf_ports();
}
{ // Nested PortSelf
  let attrs = attrs.get_port_self();
}
{ // Nested AfSpec
  let attrs = attrs.get_af_spec();
  { // Nested Inet
    let attrs = attrs.get_inet();

    // u32 indexed by ipv4-devconf - 1 on output, on input it's a nest
    attrs.get_conf(); // &[u8]
  }
  { // Nested Inet6
    let attrs = attrs.get_inet6();
    attrs.get_flags(); // u32

    // u32 indexed by ipv6-devconf - 1 on output, on input it's a nest
    attrs.get_conf(); // &[u8]
    attrs.get_stats(); // &[u8]
    attrs.get_mcast(); // &[u8]
    attrs.get_cacheinfo(); // PushIflaCacheinfo
    attrs.get_icmp6stats(); // &[u8]
    attrs.get_token(); // &[u8]
    attrs.get_addr_gen_mode(); // u8
    attrs.get_ra_mtu(); // u32
  }
  { // Nested Mctp
    let attrs = attrs.get_mctp();
    attrs.get_net(); // u32
    attrs.get_phys_binding(); // u8
  }
}
attrs.get_group(); // u32
attrs.get_net_ns_fd(); // u32

// Associated type: "RtextFilter" (1 bit per enumeration)
attrs.get_ext_mask(); // u32
attrs.get_promiscuity(); // u32
attrs.get_num_tx_queues(); // u32
attrs.get_num_rx_queues(); // u32
attrs.get_carrier(); // u8
attrs.get_phys_port_id(); // &[u8]
attrs.get_carrier_changes(); // u32
attrs.get_phys_switch_id(); // &[u8]
attrs.get_link_netnsid(); // i32
attrs.get_phys_port_name(); // &CStr
attrs.get_proto_down(); // u8
attrs.get_gso_max_segs(); // u32
attrs.get_gso_max_size(); // u32
attrs.get_pad(); // &[u8]
{ // Nested Xdp
  let attrs = attrs.get_xdp();
  attrs.get_fd(); // i32
  attrs.get_attached(); // u8
  attrs.get_flags(); // u32
  attrs.get_prog_id(); // u32
  attrs.get_drv_prog_id(); // u32
  attrs.get_skb_prog_id(); // u32
  attrs.get_hw_prog_id(); // u32
  attrs.get_expected_fd(); // i32
}
attrs.get_event(); // u32
attrs.get_new_netnsid(); // i32
attrs.get_target_netnsid(); // i32
attrs.get_carrier_up_count(); // u32
attrs.get_carrier_down_count(); // u32
attrs.get_new_ifindex(); // i32
attrs.get_min_mtu(); // u32
attrs.get_max_mtu(); // u32
{ // Nested PropList
  let attrs = attrs.get_prop_list();
  attrs.get_alt_ifname(); // &CStr
}
attrs.get_perm_address(); // &[u8]
attrs.get_proto_down_reason(); // &CStr
attrs.get_parent_dev_name(); // &CStr
attrs.get_parent_dev_bus_name(); // &CStr
attrs.get_gro_max_size(); // u32
attrs.get_tso_max_size(); // u32
attrs.get_tso_max_segs(); // u32
attrs.get_allmulti(); // u32
attrs.get_devlink_port(); // &[u8]
attrs.get_gso_ipv4_max_size(); // u32
attrs.get_gro_ipv4_max_size(); // u32
{ // Nested DpllPin
  let attrs = attrs.get_dpll_pin();
  attrs.get_id(); // u32
}

// EDT offload horizon supported by the device (in nsec).
attrs.get_max_pacing_offload_horizon(); // u32
attrs.get_netns_immutable(); // u8
```

## Dump (request)

```rust
PushOpGetlinkDumpRequest::new(&mut vec)
  .push_target_netnsid(val) // i32

  // Associated type: "RtextFilter" (1 bit per enumeration)
  .push_ext_mask(val) // u32
  .push_master(val) // u32
  .nested_linkinfo()
    .push_kind(val) // &CStr
    .push_kind_bytes(val) // &[u8]
    .nested_data_bond()
      .push_mode(val) // u8
      .push_active_slave(val) // u32
      .push_miimon(val) // u32
      .push_updelay(val) // u32
      .push_downdelay(val) // u32
      .push_use_carrier(val) // u8
      .push_arp_interval(val) // u32
      .array_arp_ip_target()
        .entry(val) // Ipv4Addr
      .end_array()
      .push_arp_validate(val) // u32
      .push_arp_all_targets(val) // u32
      .push_primary(val) // u32
      .push_primary_reselect(val) // u8
      .push_fail_over_mac(val) // u8
      .push_xmit_hash_policy(val) // u8
      .push_resend_igmp(val) // u32
      .push_num_peer_notif(val) // u8
      .push_all_slaves_active(val) // u8
      .push_min_links(val) // u32
      .push_lp_interval(val) // u32
      .push_packets_per_slave(val) // u32
      .push_ad_lacp_rate(val) // u8
      .push_ad_select(val) // u8
      .nested_ad_info()
        .push_aggregator(val) // u16
        .push_num_ports(val) // u16
        .push_actor_key(val) // u16
        .push_partner_key(val) // u16
        .push_partner_mac(val) // &[u8]
      .end_nested()
      .push_ad_actor_sys_prio(val) // u16
      .push_ad_user_port_key(val) // u16
      .push_ad_actor_system(val) // &[u8]
      .push_tlb_dynamic_lb(val) // u8
      .push_peer_notif_delay(val) // u32
      .push_ad_lacp_active(val) // u8
      .push_missed_max(val) // u8
      .array_ns_ip6_target()
        .entry(val) // &[u8]
      .end_array()
      .push_coupled_control(val) // u8
    .end_nested()
    .nested_data_bridge()
      .push_forward_delay(val) // u32
      .push_hello_time(val) // u32
      .push_max_age(val) // u32
      .push_ageing_time(val) // u32
      .push_stp_state(val) // u32
      .push_priority(val) // u16
      .push_vlan_filtering(val) // u8
      .push_vlan_protocol(val) // u16
      .push_group_fwd_mask(val) // u16
      .push_root_id(val) // PushIflaBridgeId
      .push_bridge_id(val) // PushIflaBridgeId
      .push_root_port(val) // u16
      .push_root_path_cost(val) // u32
      .push_topology_change(val) // u8
      .push_topology_change_detected(val) // u8
      .push_hello_timer(val) // u64
      .push_tcn_timer(val) // u64
      .push_topology_change_timer(val) // u64
      .push_gc_timer(val) // u64
      .push_group_addr(val) // &[u8]
      .push_fdb_flush(val) // &[u8]
      .push_mcast_router(val) // u8
      .push_mcast_snooping(val) // u8
      .push_mcast_query_use_ifaddr(val) // u8
      .push_mcast_querier(val) // u8
      .push_mcast_hash_elasticity(val) // u32
      .push_mcast_hash_max(val) // u32
      .push_mcast_last_member_cnt(val) // u32
      .push_mcast_startup_query_cnt(val) // u32
      .push_mcast_last_member_intvl(val) // u64
      .push_mcast_membership_intvl(val) // u64
      .push_mcast_querier_intvl(val) // u64
      .push_mcast_query_intvl(val) // u64
      .push_mcast_query_response_intvl(val) // u64
      .push_mcast_startup_query_intvl(val) // u64
      .push_nf_call_iptables(val) // u8
      .push_nf_call_ip6tables(val) // u8
      .push_nf_call_arptables(val) // u8
      .push_vlan_default_pvid(val) // u16
      .push_pad(val) // &[u8]
      .push_vlan_stats_enabled(val) // u8
      .push_mcast_stats_enabled(val) // u8
      .push_mcast_igmp_version(val) // u8
      .push_mcast_mld_version(val) // u8
      .push_vlan_stats_per_port(val) // u8
      .push_multi_boolopt(val) // PushBrBooloptMulti
      .push_mcast_querier_state(val) // &[u8]
      .push_fdb_n_learned(val) // u32
      .push_fdb_max_learned(val) // u32
    .end_nested()
    .nested_data_erspan()
      .push_link(val) // u32
      .push_iflags(val) // u16
      .push_oflags(val) // u16
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_pmtudisc(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_ignore_df(val) // u8
      .push_fwmark(val) // u32
      .push_erspan_index(val) // u32
      .push_erspan_ver(val) // u8
      .push_erspan_dir(val) // u8
      .push_erspan_hwid(val) // u16
    .end_nested()
    .nested_data_gre()
      .push_link(val) // u32
      .push_iflags(val) // u16
      .push_oflags(val) // u16
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_pmtudisc(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_ignore_df(val) // u8
      .push_fwmark(val) // u32
      .push_erspan_index(val) // u32
      .push_erspan_ver(val) // u8
      .push_erspan_dir(val) // u8
      .push_erspan_hwid(val) // u16
    .end_nested()
    .nested_data_gretap()
      .push_link(val) // u32
      .push_iflags(val) // u16
      .push_oflags(val) // u16
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_pmtudisc(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_ignore_df(val) // u8
      .push_fwmark(val) // u32
      .push_erspan_index(val) // u32
      .push_erspan_ver(val) // u8
      .push_erspan_dir(val) // u8
      .push_erspan_hwid(val) // u16
    .end_nested()
    .nested_data_ip6gre()
      .push_link(val) // u32
      .push_iflags(val) // u16
      .push_oflags(val) // u16
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_fwmark(val) // u32
      .push_erspan_index(val) // u32
      .push_erspan_ver(val) // u8
      .push_erspan_dir(val) // u8
      .push_erspan_hwid(val) // u16
    .end_nested()
    .nested_data_geneve()
      .push_id(val) // u32
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_port(val) // u16
      .push_collect_metadata(val) // ()
      .push_remote6(val) // &[u8]
      .push_udp_csum(val) // u8
      .push_udp_zero_csum6_tx(val) // u8
      .push_udp_zero_csum6_rx(val) // u8
      .push_label(val) // u32
      .push_ttl_inherit(val) // u8
      .push_df(val) // u8
      .push_inner_proto_inherit(val) // ()
      .push_port_range(val) // PushIflaGenevePortRange
    .end_nested()
    .nested_data_ipip()
      .push_link(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u16
      .push_proto(val) // u8
      .push_pmtudisc(val) // u8
      .push_6rd_prefix(val) // &[u8]
      .push_6rd_relay_prefix(val) // &[u8]
      .push_6rd_prefixlen(val) // u16
      .push_6rd_relay_prefixlen(val) // u16
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_ip6tnl()
      .push_link(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_proto(val) // u8
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_sit()
      .push_link(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u16
      .push_proto(val) // u8
      .push_pmtudisc(val) // u8
      .push_6rd_prefix(val) // &[u8]
      .push_6rd_relay_prefix(val) // &[u8]
      .push_6rd_prefixlen(val) // u16
      .push_6rd_relay_prefixlen(val) // u16
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_tun()
      .push_owner(val) // u32
      .push_group(val) // u32
      .push_type(val) // u8
      .push_pi(val) // u8
      .push_vnet_hdr(val) // u8
      .push_persist(val) // u8
      .push_multi_queue(val) // u8
      .push_num_queues(val) // u32
      .push_num_disabled_queues(val) // u32
    .end_nested()
    .nested_data_vlan()
      .push_id(val) // u16
      .push_flags(val) // PushIflaVlanFlags
      .nested_egress_qos()

        // Attribute may repeat multiple times (treat it as array)
        .push_mapping(val) // PushIflaVlanQosMapping
      .end_nested()
      .nested_ingress_qos()

        // Attribute may repeat multiple times (treat it as array)
        .push_mapping(val) // PushIflaVlanQosMapping
      .end_nested()

      // Associated type: "VlanProtocols" (enum)
      .push_protocol(val) // u16
    .end_nested()
    .nested_data_vrf()
      .push_table(val) // u32
    .end_nested()
    .nested_data_vti()
      .push_link(val) // u32
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_vti6()
      .push_link(val) // u32
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_netkit()
      .push_peer_info(val) // &[u8]
      .push_primary(val) // u8

      // Associated type: "NetkitPolicy" (enum)
      .push_policy(val) // u32

      // Associated type: "NetkitPolicy" (enum)
      .push_peer_policy(val) // u32

      // Associated type: "NetkitMode" (enum)
      .push_mode(val) // u32

      // Associated type: "NetkitScrub" (enum)
      .push_scrub(val) // u32

      // Associated type: "NetkitScrub" (enum)
      .push_peer_scrub(val) // u32
      .push_headroom(val) // u16
      .push_tailroom(val) // u16
    .end_nested()
    .nested_data_ovpn()

      // Associated type: "OvpnMode" (enum)
      .push_mode(val) // u8
    .end_nested()
    .push_xstats(val) // &[u8]
    .push_slave_kind(val) // &CStr
    .push_slave_kind_bytes(val) // &[u8]
    .nested_slave_data_bridge()
      .push_state(val) // u8
      .push_priority(val) // u16
      .push_cost(val) // u32
      .push_mode(val) // ()
      .push_guard(val) // ()
      .push_protect(val) // ()
      .push_fast_leave(val) // ()
      .push_learning(val) // ()
      .push_unicast_flood(val) // ()
      .push_proxyarp(val) // ()
      .push_learning_sync(val) // ()
      .push_proxyarp_wifi(val) // ()
      .push_root_id(val) // PushIflaBridgeId
      .push_bridge_id(val) // PushIflaBridgeId
      .push_designated_port(val) // u16
      .push_designated_cost(val) // u16
      .push_id(val) // u16
      .push_no(val) // u16
      .push_topology_change_ack(val) // u8
      .push_config_pending(val) // u8
      .push_message_age_timer(val) // u64
      .push_forward_delay_timer(val) // u64
      .push_hold_timer(val) // u64
      .push_flush(val) // ()
      .push_multicast_router(val) // u8
      .push_pad(val) // &[u8]
      .push_mcast_flood(val) // ()
      .push_mcast_to_ucast(val) // ()
      .push_vlan_tunnel(val) // ()
      .push_bcast_flood(val) // ()
      .push_group_fwd_mask(val) // u16
      .push_neigh_suppress(val) // ()
      .push_isolated(val) // ()
      .push_backup_port(val) // u32
      .push_mrp_ring_open(val) // ()
      .push_mrp_in_open(val) // ()
      .push_mcast_eht_hosts_limit(val) // u32
      .push_mcast_eht_hosts_cnt(val) // u32
      .push_locked(val) // ()
      .push_mab(val) // ()
      .push_mcast_n_groups(val) // u32
      .push_mcast_max_groups(val) // u32
      .push_neigh_vlan_suppress(val) // ()
      .push_backup_nhid(val) // u32
    .end_nested()
    .nested_slave_data_bond()
      .push_state(val) // u8
      .push_mii_status(val) // u8
      .push_link_failure_count(val) // u32
      .push_perm_hwaddr(val) // &[u8]
      .push_queue_id(val) // u16
      .push_ad_aggregator_id(val) // u16
      .push_ad_actor_oper_port_state(val) // u8
      .push_ad_partner_oper_port_state(val) // u16
      .push_prio(val) // u32
    .end_nested()
  .end_nested()
  ;
```

### Dump (reply)

```rust
let attrs = OpGetlinkDumpReply::new(buf);

attrs.get_address(); // &[u8]
attrs.get_broadcast(); // &[u8]
attrs.get_ifname(); // &CStr
attrs.get_mtu(); // u32
attrs.get_link(); // u32
attrs.get_qdisc(); // &CStr
attrs.get_stats(); // PushRtnlLinkStats
attrs.get_cost(); // &CStr
attrs.get_priority(); // &CStr
attrs.get_master(); // u32
attrs.get_wireless(); // &CStr
attrs.get_protinfo(); // &CStr
attrs.get_txqlen(); // u32
attrs.get_map(); // PushRtnlLinkIfmap
attrs.get_weight(); // u32
attrs.get_operstate(); // u8
attrs.get_linkmode(); // u8
{ // Nested Linkinfo
  let attrs = attrs.get_linkinfo();
  attrs.get_kind(); // &CStr
  attrs.get_data(); // submessage
  attrs.get_xstats(); // &[u8]
  attrs.get_slave_kind(); // &CStr
  attrs.get_slave_data(); // submessage
}
attrs.get_net_ns_pid(); // u32
attrs.get_ifalias(); // &CStr
attrs.get_num_vf(); // u32
{ // Nested VfinfoList
  let attrs = attrs.get_vfinfo_list();
  { // Nested Info

    // Attribute may repeat multiple times (treat it as array)
    for entry in attrs.get_info() {
      entry.get_mac(); // PushIflaVfMac
      entry.get_vlan(); // PushIflaVfVlan
      entry.get_tx_rate(); // PushIflaVfTxRate
      entry.get_spoofchk(); // PushIflaVfSpoofchk
      entry.get_link_state(); // PushIflaVfLinkState
      entry.get_rate(); // PushIflaVfRate
      entry.get_rss_query_en(); // PushIflaVfRssQueryEn
      { // Nested Stats
        let attrs = entry.get_stats();
        attrs.get_rx_packets(); // u64
        attrs.get_tx_packets(); // u64
        attrs.get_rx_bytes(); // u64
        attrs.get_tx_bytes(); // u64
        attrs.get_broadcast(); // u64
        attrs.get_multicast(); // u64
        attrs.get_pad(); // &[u8]
        attrs.get_rx_dropped(); // u64
        attrs.get_tx_dropped(); // u64
      }
      entry.get_trust(); // PushIflaVfTrust
      entry.get_ib_node_guid(); // PushIflaVfGuid
      entry.get_ib_port_guid(); // PushIflaVfGuid
      { // Nested VlanList
        let attrs = entry.get_vlan_list();

        // Attribute may repeat multiple times (treat it as array)
        for entry in attrs.get_info() {
          entry; // PushIflaVfVlanInfo
        }
      }
      entry.get_broadcast(); // &[u8]
    }
  }
}
attrs.get_stats64(); // PushRtnlLinkStats64
{ // Nested VfPorts
  let attrs = attrs.get_vf_ports();
}
{ // Nested PortSelf
  let attrs = attrs.get_port_self();
}
{ // Nested AfSpec
  let attrs = attrs.get_af_spec();
  { // Nested Inet
    let attrs = attrs.get_inet();

    // u32 indexed by ipv4-devconf - 1 on output, on input it's a nest
    attrs.get_conf(); // &[u8]
  }
  { // Nested Inet6
    let attrs = attrs.get_inet6();
    attrs.get_flags(); // u32

    // u32 indexed by ipv6-devconf - 1 on output, on input it's a nest
    attrs.get_conf(); // &[u8]
    attrs.get_stats(); // &[u8]
    attrs.get_mcast(); // &[u8]
    attrs.get_cacheinfo(); // PushIflaCacheinfo
    attrs.get_icmp6stats(); // &[u8]
    attrs.get_token(); // &[u8]
    attrs.get_addr_gen_mode(); // u8
    attrs.get_ra_mtu(); // u32
  }
  { // Nested Mctp
    let attrs = attrs.get_mctp();
    attrs.get_net(); // u32
    attrs.get_phys_binding(); // u8
  }
}
attrs.get_group(); // u32
attrs.get_net_ns_fd(); // u32

// Associated type: "RtextFilter" (1 bit per enumeration)
attrs.get_ext_mask(); // u32
attrs.get_promiscuity(); // u32
attrs.get_num_tx_queues(); // u32
attrs.get_num_rx_queues(); // u32
attrs.get_carrier(); // u8
attrs.get_phys_port_id(); // &[u8]
attrs.get_carrier_changes(); // u32
attrs.get_phys_switch_id(); // &[u8]
attrs.get_link_netnsid(); // i32
attrs.get_phys_port_name(); // &CStr
attrs.get_proto_down(); // u8
attrs.get_gso_max_segs(); // u32
attrs.get_gso_max_size(); // u32
attrs.get_pad(); // &[u8]
{ // Nested Xdp
  let attrs = attrs.get_xdp();
  attrs.get_fd(); // i32
  attrs.get_attached(); // u8
  attrs.get_flags(); // u32
  attrs.get_prog_id(); // u32
  attrs.get_drv_prog_id(); // u32
  attrs.get_skb_prog_id(); // u32
  attrs.get_hw_prog_id(); // u32
  attrs.get_expected_fd(); // i32
}
attrs.get_event(); // u32
attrs.get_new_netnsid(); // i32
attrs.get_target_netnsid(); // i32
attrs.get_carrier_up_count(); // u32
attrs.get_carrier_down_count(); // u32
attrs.get_new_ifindex(); // i32
attrs.get_min_mtu(); // u32
attrs.get_max_mtu(); // u32
{ // Nested PropList
  let attrs = attrs.get_prop_list();
  attrs.get_alt_ifname(); // &CStr
}
attrs.get_perm_address(); // &[u8]
attrs.get_proto_down_reason(); // &CStr
attrs.get_parent_dev_name(); // &CStr
attrs.get_parent_dev_bus_name(); // &CStr
attrs.get_gro_max_size(); // u32
attrs.get_tso_max_size(); // u32
attrs.get_tso_max_segs(); // u32
attrs.get_allmulti(); // u32
attrs.get_devlink_port(); // &[u8]
attrs.get_gso_ipv4_max_size(); // u32
attrs.get_gro_ipv4_max_size(); // u32
{ // Nested DpllPin
  let attrs = attrs.get_dpll_pin();
  attrs.get_id(); // u32
}

// EDT offload horizon supported by the device (in nsec).
attrs.get_max_pacing_offload_horizon(); // u32
attrs.get_netns_immutable(); // u8
```

# Operation "setlink"

## Do (request)

```rust
PushOpSetlinkDoRequest::new(&mut vec)
  .push_address(val) // &[u8]
  .push_broadcast(val) // &[u8]
  .push_ifname(val) // &CStr
  .push_ifname_bytes(val) // &[u8]
  .push_mtu(val) // u32
  .push_link(val) // u32
  .push_qdisc(val) // &CStr
  .push_qdisc_bytes(val) // &[u8]
  .push_stats(val) // PushRtnlLinkStats
  .push_cost(val) // &CStr
  .push_cost_bytes(val) // &[u8]
  .push_priority(val) // &CStr
  .push_priority_bytes(val) // &[u8]
  .push_master(val) // u32
  .push_wireless(val) // &CStr
  .push_wireless_bytes(val) // &[u8]
  .push_protinfo(val) // &CStr
  .push_protinfo_bytes(val) // &[u8]
  .push_txqlen(val) // u32
  .push_map(val) // PushRtnlLinkIfmap
  .push_weight(val) // u32
  .push_operstate(val) // u8
  .push_linkmode(val) // u8
  .nested_linkinfo()
    .push_kind(val) // &CStr
    .push_kind_bytes(val) // &[u8]
    .nested_data_bond()
      .push_mode(val) // u8
      .push_active_slave(val) // u32
      .push_miimon(val) // u32
      .push_updelay(val) // u32
      .push_downdelay(val) // u32
      .push_use_carrier(val) // u8
      .push_arp_interval(val) // u32
      .array_arp_ip_target()
        .entry(val) // Ipv4Addr
      .end_array()
      .push_arp_validate(val) // u32
      .push_arp_all_targets(val) // u32
      .push_primary(val) // u32
      .push_primary_reselect(val) // u8
      .push_fail_over_mac(val) // u8
      .push_xmit_hash_policy(val) // u8
      .push_resend_igmp(val) // u32
      .push_num_peer_notif(val) // u8
      .push_all_slaves_active(val) // u8
      .push_min_links(val) // u32
      .push_lp_interval(val) // u32
      .push_packets_per_slave(val) // u32
      .push_ad_lacp_rate(val) // u8
      .push_ad_select(val) // u8
      .nested_ad_info()
        .push_aggregator(val) // u16
        .push_num_ports(val) // u16
        .push_actor_key(val) // u16
        .push_partner_key(val) // u16
        .push_partner_mac(val) // &[u8]
      .end_nested()
      .push_ad_actor_sys_prio(val) // u16
      .push_ad_user_port_key(val) // u16
      .push_ad_actor_system(val) // &[u8]
      .push_tlb_dynamic_lb(val) // u8
      .push_peer_notif_delay(val) // u32
      .push_ad_lacp_active(val) // u8
      .push_missed_max(val) // u8
      .array_ns_ip6_target()
        .entry(val) // &[u8]
      .end_array()
      .push_coupled_control(val) // u8
    .end_nested()
    .nested_data_bridge()
      .push_forward_delay(val) // u32
      .push_hello_time(val) // u32
      .push_max_age(val) // u32
      .push_ageing_time(val) // u32
      .push_stp_state(val) // u32
      .push_priority(val) // u16
      .push_vlan_filtering(val) // u8
      .push_vlan_protocol(val) // u16
      .push_group_fwd_mask(val) // u16
      .push_root_id(val) // PushIflaBridgeId
      .push_bridge_id(val) // PushIflaBridgeId
      .push_root_port(val) // u16
      .push_root_path_cost(val) // u32
      .push_topology_change(val) // u8
      .push_topology_change_detected(val) // u8
      .push_hello_timer(val) // u64
      .push_tcn_timer(val) // u64
      .push_topology_change_timer(val) // u64
      .push_gc_timer(val) // u64
      .push_group_addr(val) // &[u8]
      .push_fdb_flush(val) // &[u8]
      .push_mcast_router(val) // u8
      .push_mcast_snooping(val) // u8
      .push_mcast_query_use_ifaddr(val) // u8
      .push_mcast_querier(val) // u8
      .push_mcast_hash_elasticity(val) // u32
      .push_mcast_hash_max(val) // u32
      .push_mcast_last_member_cnt(val) // u32
      .push_mcast_startup_query_cnt(val) // u32
      .push_mcast_last_member_intvl(val) // u64
      .push_mcast_membership_intvl(val) // u64
      .push_mcast_querier_intvl(val) // u64
      .push_mcast_query_intvl(val) // u64
      .push_mcast_query_response_intvl(val) // u64
      .push_mcast_startup_query_intvl(val) // u64
      .push_nf_call_iptables(val) // u8
      .push_nf_call_ip6tables(val) // u8
      .push_nf_call_arptables(val) // u8
      .push_vlan_default_pvid(val) // u16
      .push_pad(val) // &[u8]
      .push_vlan_stats_enabled(val) // u8
      .push_mcast_stats_enabled(val) // u8
      .push_mcast_igmp_version(val) // u8
      .push_mcast_mld_version(val) // u8
      .push_vlan_stats_per_port(val) // u8
      .push_multi_boolopt(val) // PushBrBooloptMulti
      .push_mcast_querier_state(val) // &[u8]
      .push_fdb_n_learned(val) // u32
      .push_fdb_max_learned(val) // u32
    .end_nested()
    .nested_data_erspan()
      .push_link(val) // u32
      .push_iflags(val) // u16
      .push_oflags(val) // u16
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_pmtudisc(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_ignore_df(val) // u8
      .push_fwmark(val) // u32
      .push_erspan_index(val) // u32
      .push_erspan_ver(val) // u8
      .push_erspan_dir(val) // u8
      .push_erspan_hwid(val) // u16
    .end_nested()
    .nested_data_gre()
      .push_link(val) // u32
      .push_iflags(val) // u16
      .push_oflags(val) // u16
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_pmtudisc(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_ignore_df(val) // u8
      .push_fwmark(val) // u32
      .push_erspan_index(val) // u32
      .push_erspan_ver(val) // u8
      .push_erspan_dir(val) // u8
      .push_erspan_hwid(val) // u16
    .end_nested()
    .nested_data_gretap()
      .push_link(val) // u32
      .push_iflags(val) // u16
      .push_oflags(val) // u16
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_pmtudisc(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_ignore_df(val) // u8
      .push_fwmark(val) // u32
      .push_erspan_index(val) // u32
      .push_erspan_ver(val) // u8
      .push_erspan_dir(val) // u8
      .push_erspan_hwid(val) // u16
    .end_nested()
    .nested_data_ip6gre()
      .push_link(val) // u32
      .push_iflags(val) // u16
      .push_oflags(val) // u16
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_fwmark(val) // u32
      .push_erspan_index(val) // u32
      .push_erspan_ver(val) // u8
      .push_erspan_dir(val) // u8
      .push_erspan_hwid(val) // u16
    .end_nested()
    .nested_data_geneve()
      .push_id(val) // u32
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_port(val) // u16
      .push_collect_metadata(val) // ()
      .push_remote6(val) // &[u8]
      .push_udp_csum(val) // u8
      .push_udp_zero_csum6_tx(val) // u8
      .push_udp_zero_csum6_rx(val) // u8
      .push_label(val) // u32
      .push_ttl_inherit(val) // u8
      .push_df(val) // u8
      .push_inner_proto_inherit(val) // ()
      .push_port_range(val) // PushIflaGenevePortRange
    .end_nested()
    .nested_data_ipip()
      .push_link(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u16
      .push_proto(val) // u8
      .push_pmtudisc(val) // u8
      .push_6rd_prefix(val) // &[u8]
      .push_6rd_relay_prefix(val) // &[u8]
      .push_6rd_prefixlen(val) // u16
      .push_6rd_relay_prefixlen(val) // u16
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_ip6tnl()
      .push_link(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u32
      .push_proto(val) // u8
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_sit()
      .push_link(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_ttl(val) // u8
      .push_tos(val) // u8
      .push_encap_limit(val) // u8
      .push_flowinfo(val) // u32
      .push_flags(val) // u16
      .push_proto(val) // u8
      .push_pmtudisc(val) // u8
      .push_6rd_prefix(val) // &[u8]
      .push_6rd_relay_prefix(val) // &[u8]
      .push_6rd_prefixlen(val) // u16
      .push_6rd_relay_prefixlen(val) // u16
      .push_encap_type(val) // u16
      .push_encap_flags(val) // u16
      .push_encap_sport(val) // u16
      .push_encap_dport(val) // u16
      .push_collect_metadata(val) // ()
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_tun()
      .push_owner(val) // u32
      .push_group(val) // u32
      .push_type(val) // u8
      .push_pi(val) // u8
      .push_vnet_hdr(val) // u8
      .push_persist(val) // u8
      .push_multi_queue(val) // u8
      .push_num_queues(val) // u32
      .push_num_disabled_queues(val) // u32
    .end_nested()
    .nested_data_vlan()
      .push_id(val) // u16
      .push_flags(val) // PushIflaVlanFlags
      .nested_egress_qos()

        // Attribute may repeat multiple times (treat it as array)
        .push_mapping(val) // PushIflaVlanQosMapping
      .end_nested()
      .nested_ingress_qos()

        // Attribute may repeat multiple times (treat it as array)
        .push_mapping(val) // PushIflaVlanQosMapping
      .end_nested()

      // Associated type: "VlanProtocols" (enum)
      .push_protocol(val) // u16
    .end_nested()
    .nested_data_vrf()
      .push_table(val) // u32
    .end_nested()
    .nested_data_vti()
      .push_link(val) // u32
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_vti6()
      .push_link(val) // u32
      .push_ikey(val) // u32
      .push_okey(val) // u32
      .push_local(val) // &[u8]
      .push_remote(val) // &[u8]
      .push_fwmark(val) // u32
    .end_nested()
    .nested_data_netkit()
      .push_peer_info(val) // &[u8]
      .push_primary(val) // u8

      // Associated type: "NetkitPolicy" (enum)
      .push_policy(val) // u32

      // Associated type: "NetkitPolicy" (enum)
      .push_peer_policy(val) // u32

      // Associated type: "NetkitMode" (enum)
      .push_mode(val) // u32

      // Associated type: "NetkitScrub" (enum)
      .push_scrub(val) // u32

      // Associated type: "NetkitScrub" (enum)
      .push_peer_scrub(val) // u32
      .push_headroom(val) // u16
      .push_tailroom(val) // u16
    .end_nested()
    .nested_data_ovpn()

      // Associated type: "OvpnMode" (enum)
      .push_mode(val) // u8
    .end_nested()
    .push_xstats(val) // &[u8]
    .push_slave_kind(val) // &CStr
    .push_slave_kind_bytes(val) // &[u8]
    .nested_slave_data_bridge()
      .push_state(val) // u8
      .push_priority(val) // u16
      .push_cost(val) // u32
      .push_mode(val) // ()
      .push_guard(val) // ()
      .push_protect(val) // ()
      .push_fast_leave(val) // ()
      .push_learning(val) // ()
      .push_unicast_flood(val) // ()
      .push_proxyarp(val) // ()
      .push_learning_sync(val) // ()
      .push_proxyarp_wifi(val) // ()
      .push_root_id(val) // PushIflaBridgeId
      .push_bridge_id(val) // PushIflaBridgeId
      .push_designated_port(val) // u16
      .push_designated_cost(val) // u16
      .push_id(val) // u16
      .push_no(val) // u16
      .push_topology_change_ack(val) // u8
      .push_config_pending(val) // u8
      .push_message_age_timer(val) // u64
      .push_forward_delay_timer(val) // u64
      .push_hold_timer(val) // u64
      .push_flush(val) // ()
      .push_multicast_router(val) // u8
      .push_pad(val) // &[u8]
      .push_mcast_flood(val) // ()
      .push_mcast_to_ucast(val) // ()
      .push_vlan_tunnel(val) // ()
      .push_bcast_flood(val) // ()
      .push_group_fwd_mask(val) // u16
      .push_neigh_suppress(val) // ()
      .push_isolated(val) // ()
      .push_backup_port(val) // u32
      .push_mrp_ring_open(val) // ()
      .push_mrp_in_open(val) // ()
      .push_mcast_eht_hosts_limit(val) // u32
      .push_mcast_eht_hosts_cnt(val) // u32
      .push_locked(val) // ()
      .push_mab(val) // ()
      .push_mcast_n_groups(val) // u32
      .push_mcast_max_groups(val) // u32
      .push_neigh_vlan_suppress(val) // ()
      .push_backup_nhid(val) // u32
    .end_nested()
    .nested_slave_data_bond()
      .push_state(val) // u8
      .push_mii_status(val) // u8
      .push_link_failure_count(val) // u32
      .push_perm_hwaddr(val) // &[u8]
      .push_queue_id(val) // u16
      .push_ad_aggregator_id(val) // u16
      .push_ad_actor_oper_port_state(val) // u8
      .push_ad_partner_oper_port_state(val) // u16
      .push_prio(val) // u32
    .end_nested()
  .end_nested()
  .push_net_ns_pid(val) // u32
  .push_ifalias(val) // &CStr
  .push_ifalias_bytes(val) // &[u8]
  .push_num_vf(val) // u32
  .nested_vfinfo_list()

    // Attribute may repeat multiple times (treat it as array)
    .nested_info()
      .push_mac(val) // PushIflaVfMac
      .push_vlan(val) // PushIflaVfVlan
      .push_tx_rate(val) // PushIflaVfTxRate
      .push_spoofchk(val) // PushIflaVfSpoofchk
      .push_link_state(val) // PushIflaVfLinkState
      .push_rate(val) // PushIflaVfRate
      .push_rss_query_en(val) // PushIflaVfRssQueryEn
      .nested_stats()
        .push_rx_packets(val) // u64
        .push_tx_packets(val) // u64
        .push_rx_bytes(val) // u64
        .push_tx_bytes(val) // u64
        .push_broadcast(val) // u64
        .push_multicast(val) // u64
        .push_pad(val) // &[u8]
        .push_rx_dropped(val) // u64
        .push_tx_dropped(val) // u64
      .end_nested()
      .push_trust(val) // PushIflaVfTrust
      .push_ib_node_guid(val) // PushIflaVfGuid
      .push_ib_port_guid(val) // PushIflaVfGuid
      .nested_vlan_list()

        // Attribute may repeat multiple times (treat it as array)
        .push_info(val) // PushIflaVfVlanInfo
      .end_nested()
      .push_broadcast(val) // &[u8]
    .end_nested()
  .end_nested()
  .push_stats64(val) // PushRtnlLinkStats64
  .nested_vf_ports()
  .end_nested()
  .nested_port_self()
  .end_nested()
  .nested_af_spec()
    .nested_inet()

      // u32 indexed by ipv4-devconf - 1 on output, on input it's a nest
      .push_conf(val) // &[u8]
    .end_nested()
    .nested_inet6()
      .push_flags(val) // u32

      // u32 indexed by ipv6-devconf - 1 on output, on input it's a nest
      .push_conf(val) // &[u8]
      .push_stats(val) // &[u8]
      .push_mcast(val) // &[u8]
      .push_cacheinfo(val) // PushIflaCacheinfo
      .push_icmp6stats(val) // &[u8]
      .push_token(val) // &[u8]
      .push_addr_gen_mode(val) // u8
      .push_ra_mtu(val) // u32
    .end_nested()
    .nested_mctp()
      .push_net(val) // u32
      .push_phys_binding(val) // u8
    .end_nested()
  .end_nested()
  .push_group(val) // u32
  .push_net_ns_fd(val) // u32

  // Associated type: "RtextFilter" (1 bit per enumeration)
  .push_ext_mask(val) // u32
  .push_promiscuity(val) // u32
  .push_num_tx_queues(val) // u32
  .push_num_rx_queues(val) // u32
  .push_carrier(val) // u8
  .push_phys_port_id(val) // &[u8]
  .push_carrier_changes(val) // u32
  .push_phys_switch_id(val) // &[u8]
  .push_link_netnsid(val) // i32
  .push_phys_port_name(val) // &CStr
  .push_phys_port_name_bytes(val) // &[u8]
  .push_proto_down(val) // u8
  .push_gso_max_segs(val) // u32
  .push_gso_max_size(val) // u32
  .push_pad(val) // &[u8]
  .nested_xdp()
    .push_fd(val) // i32
    .push_attached(val) // u8
    .push_flags(val) // u32
    .push_prog_id(val) // u32
    .push_drv_prog_id(val) // u32
    .push_skb_prog_id(val) // u32
    .push_hw_prog_id(val) // u32
    .push_expected_fd(val) // i32
  .end_nested()
  .push_event(val) // u32
  .push_new_netnsid(val) // i32
  .push_target_netnsid(val) // i32
  .push_carrier_up_count(val) // u32
  .push_carrier_down_count(val) // u32
  .push_new_ifindex(val) // i32
  .push_min_mtu(val) // u32
  .push_max_mtu(val) // u32
  .nested_prop_list()
    .push_alt_ifname(val) // &CStr
    .push_alt_ifname_bytes(val) // &[u8]
  .end_nested()
  .push_perm_address(val) // &[u8]
  .push_proto_down_reason(val) // &CStr
  .push_proto_down_reason_bytes(val) // &[u8]
  .push_parent_dev_name(val) // &CStr
  .push_parent_dev_name_bytes(val) // &[u8]
  .push_parent_dev_bus_name(val) // &CStr
  .push_parent_dev_bus_name_bytes(val) // &[u8]
  .push_gro_max_size(val) // u32
  .push_tso_max_size(val) // u32
  .push_tso_max_segs(val) // u32
  .push_allmulti(val) // u32
  .push_devlink_port(val) // &[u8]
  .push_gso_ipv4_max_size(val) // u32
  .push_gro_ipv4_max_size(val) // u32
  .nested_dpll_pin()
    .push_id(val) // u32
  .end_nested()

  // EDT offload horizon supported by the device (in nsec).
  .push_max_pacing_offload_horizon(val) // u32
  .push_netns_immutable(val) // u8
  ;
```

### Do (reply)

```rust
let attrs = OpSetlinkDoReply::new(buf);

// No attributes
```

# Operation "getstats"

## Do (request)

```rust
PushOpGetstatsDoRequest::new(&mut vec)
  ;
```

### Do (reply)

```rust
let attrs = OpGetstatsDoReply::new(buf);

attrs.get_link_64(); // PushRtnlLinkStats64
attrs.get_link_xstats(); // &[u8]
attrs.get_link_xstats_slave(); // &[u8]
{ // Nested LinkOffloadXstats
  let attrs = attrs.get_link_offload_xstats();
  attrs.get_cpu_hit(); // &[u8]

  for entry in attrs.get_hw_s_info() {
    entry.get_request(); // u8
    entry.get_used(); // u8
  }
  attrs.get_l3_stats(); // &[u8]
}
attrs.get_af_spec(); // &[u8]
```

## Dump (request)

```rust
PushOpGetstatsDumpRequest::new(&mut vec)
  ;
```

### Dump (reply)

```rust
let attrs = OpGetstatsDumpReply::new(buf);

attrs.get_link_64(); // PushRtnlLinkStats64
attrs.get_link_xstats(); // &[u8]
attrs.get_link_xstats_slave(); // &[u8]
{ // Nested LinkOffloadXstats
  let attrs = attrs.get_link_offload_xstats();
  attrs.get_cpu_hit(); // &[u8]

  for entry in attrs.get_hw_s_info() {
    entry.get_request(); // u8
    entry.get_used(); // u8
  }
  attrs.get_l3_stats(); // &[u8]
}
attrs.get_af_spec(); // &[u8]
```
