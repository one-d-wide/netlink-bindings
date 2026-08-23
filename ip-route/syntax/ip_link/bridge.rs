use super::group;

group! {
    const TYPE_BRIDGE;
    ~ repeat: star,
    ~ init => {
        let mut bridge = attrs
            .nested_linkinfo()
            .nested_data_bridge();
        let mut bridge_boolopt = BrBooloptMulti::default();
        let mut bridge_boolopt_set = |b: BrBooloptId, val: bool| {
            bridge_boolopt.optmask |= b as u32;
            if val {
                bridge_boolopt.optval |= b as u32;
            } else {
                bridge_boolopt.optval &= !(b as u32);
            }
        };
    }
    ~ fini => {
        if bridge_boolopt != BrBooloptMulti::default() {
            bridge = bridge.push_multi_boolopt(bridge_boolopt);
        }
        attrs = bridge
            .end_nested()
            .end_nested();
    }
    "forward_delay", val: u32 => {
        bridge = bridge.push_forward_delay(val);
    }
    "hello_time", val: u32 => {
        bridge = bridge.push_hello_time(val);
    }
    "max_age", val: u32 => {
        bridge = bridge.push_max_age(val);
    }
    "ageing_time", val: u32 => {
        bridge = bridge.push_ageing_time(val);
    }
    "priority", val: u16 => {
        bridge = bridge.push_priority(val);
    }
    "stp_state", val: u32 => {
        bridge = bridge.push_stp_state(val);
    }
    "stp_mode", "auto" => {
        bridge = bridge.push_stp_mode(0);
    }
    "stp_mode", "user" => {
        bridge = bridge.push_stp_mode(1);
    }
    "stp_mode", "kernel" => {
        bridge = bridge.push_stp_mode(2);
    }
    "stp_mode", val: u32 => {
        bridge = bridge.push_stp_mode(val);
    }
    "vlan_filtering", val: u8 => {
        bridge = bridge.push_vlan_filtering(val);
    }
    "vlan_protocol", val: u16 => {
        bridge = bridge.push_vlan_protocol(val);
    }
    "vlan_default_pvid", val: u16 => {
        bridge = bridge.push_vlan_default_pvid(val);
    }
    "vlan_stats_enabled", val: u8 => {
        bridge = bridge.push_vlan_stats_enabled(val);
    }
    "vlan_stats_per_port", val: u8 => {
        bridge = bridge.push_vlan_stats_per_port(val);
    }
    "group_fwd_mask", val: u16 => {
        bridge = bridge.push_group_fwd_mask(val);
    }
    "group_address", val: &[u8] as "mac" => {
        bridge = bridge.push_group_addr(val);
    }
    "no_linklocal_learn", val: bool => {
        bridge_boolopt_set(BrBooloptId::NoLlLearn, val);
    }
    "mst_enabled", val: bool => {
        bridge_boolopt_set(BrBooloptId::MstEnable, val);
    }
    "fdb_local_vlan_0", val: bool => {
        bridge_boolopt_set(BrBooloptId::FdbLocalVlan0, val);
    }
    "mcast_vlan_snooping", val: bool => {
        bridge_boolopt_set(BrBooloptId::McastVlanSnooping, val);
    }
    "mdb_offload_fail_notification", val: bool => {
        bridge_boolopt_set(BrBooloptId::MdbOffloadFailNotification, val);
    }
    "fdb_max_learned", val: u32 => {
        bridge = bridge.push_fdb_max_learned(val);
    }
    "mcast_router", val: u8 => {
        bridge = bridge.push_mcast_router(val);
    }
    "mcast_snooping", val: u8 => {
        bridge = bridge.push_mcast_snooping(val);
    }
    "mcast_query_use_ifaddr", val: u8 => {
        bridge = bridge.push_mcast_query_use_ifaddr(val);
    }
    "mcast_querier", val: u8 => {
        bridge = bridge.push_mcast_querier(val);
    }
    "mcast_hash_elasticity", val: u32 => {
        bridge = bridge.push_mcast_hash_elasticity(val);
    }
    "mcast_hash_max", val: u32 => {
        bridge = bridge.push_mcast_hash_max(val);
    }
    "mcast_last_member_count", val: u32 => {
        bridge = bridge.push_mcast_last_member_cnt(val);
    }
    "mcast_startup_query_count", val: u32 => {
        bridge = bridge.push_mcast_startup_query_cnt(val);
    }
    "mcast_last_member_interval", val: u64 => {
        bridge = bridge.push_mcast_last_member_intvl(val);
    }
    "mcast_membership_interval", val: u64 => {
        bridge = bridge.push_mcast_membership_intvl(val);
    }
    "mcast_querier_interval", val: u64 => {
        bridge = bridge.push_mcast_querier_intvl(val);
    }
    "mcast_query_interval", val: u64 => {
        bridge = bridge.push_mcast_query_intvl(val);
    }
    "mcast_query_response_interval", val: u64 => {
        bridge = bridge.push_mcast_query_response_intvl(val);
    }
    "mcast_startup_query_interval", val: u64 => {
        bridge = bridge.push_mcast_startup_query_intvl(val);
    }
    "mcast_stats_enabled", val: u8 => {
        bridge = bridge.push_mcast_stats_enabled(val);
    }
    "mcast_igmp_version", val: u8 => {
        bridge = bridge.push_mcast_igmp_version(val);
    }
    "mcast_mld_version", val: u8 => {
        bridge = bridge.push_mcast_mld_version(val);
    }
    "nf_call_iptables", val: u8 => {
        bridge = bridge.push_nf_call_iptables(val);
    }
    "nf_call_ip6tables", val: u8 => {
        bridge = bridge.push_nf_call_ip6tables(val);
    }
    "nf_call_arptables", val: u8 => {
        bridge = bridge.push_nf_call_arptables(val);
    }
}
