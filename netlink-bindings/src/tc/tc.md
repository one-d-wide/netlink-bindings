
# Operation "newqdisc"

## Do (request)

```rust
PushOpNewqdiscDoRequest::new(&mut vec)
  .push_kind(val) // &CStr
  .sub_nested_options_basic()
    .push_classid(val) // u32
    .nested_ematches()
      .push_tree_hdr(val) // PushTcfEmatchTreeHdr
      .push_tree_list(val) // &[u8]
    .end_nested()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_pcnt(val) // PushTcBasicPcnt
    .push_pad(val) // &[u8]
  .end_nested()
  .sub_nested_options_bpf()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_classid(val) // u32
    .push_ops_len(val) // u16
    .push_ops(val) // &[u8]
    .push_fd(val) // u32
    .push_name(val) // &CStr
    .push_flags(val) // u32
    .push_flags_gen(val) // u32
    .push_tag(val) // &[u8]
    .push_id(val) // u32
  .end_nested()
  .sub_nested_options_bfifo(fixed_header) // PushTcFifoQopt
  .sub_nested_options_cake()
    .push_pad(val) // &[u8]
    .push_base_rate64(val) // u64
    .push_diffserv_mode(val) // u32
    .push_atm(val) // u32
    .push_flow_mode(val) // u32
    .push_overhead(val) // u32
    .push_rtt(val) // u32
    .push_target(val) // u32
    .push_autorate(val) // u32
    .push_memory(val) // u32
    .push_nat(val) // u32
    .push_raw(val) // u32
    .push_wash(val) // u32
    .push_mpu(val) // u32
    .push_ingress(val) // u32
    .push_ack_filter(val) // u32
    .push_split_gso(val) // u32
    .push_fwmark(val) // u32
  .end_nested()
  .sub_nested_options_cbs()
    .push_parms(val) // PushTcCbsQopt
  .end_nested()
  .sub_nested_options_cgroup()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_ematches(val) // &[u8]
  .end_nested()
  .sub_nested_options_choke()
    .push_parms(val) // PushTcRedQopt
    .push_stab(val) // &[u8]
    .push_max_p(val) // u32
  .end_nested()
  .sub_nested_options_clsact()
  .sub_nested_options_codel()
    .push_target(val) // u32
    .push_limit(val) // u32
    .push_interval(val) // u32
    .push_ecn(val) // u32
    .push_ce_threshold(val) // u32
  .end_nested()
  .sub_nested_options_drr()
    .push_quantum(val) // u32
  .end_nested()
  .sub_nested_options_dualpi2()

    // Limit of total number of packets in queue
    .push_limit(val) // u32

    // Memory limit of total number of packets in queue
    .push_memory_limit(val) // u32

    // Classic target delay in microseconds
    .push_target(val) // u32

    // Drop probability update interval time in microseconds
    .push_tupdate(val) // u32

    // Integral gain factor in Hz for PI controller
    .push_alpha(val) // u32

    // Proportional gain factor in Hz for PI controller
    .push_beta(val) // u32

    // L4S step marking threshold in packets
    .push_step_thresh_pkts(val) // u32

    // L4S Step marking threshold in microseconds
    .push_step_thresh_us(val) // u32

    // Packets enqueued to the L-queue can apply the step threshold when the queue length of L-queue is larger than this value. (0 is recommended)
    .push_min_qlen_step(val) // u32

    // Probability coupling factor between Classic and L4S (2 is recommended)
    .push_coupling(val) // u8

    // Control the overload strategy (drop to preserve latency or let the queue overflow)
    // Associated type: "Dualpi2DropOverload" (enum)
    .push_drop_overload(val) // u8

    // Decide where the Classic packets are PI-based dropped or marked
    // Associated type: "Dualpi2DropEarly" (enum)
    .push_drop_early(val) // u8

    // Classic WRR weight in percentage (from 0 to 100)
    .push_c_protection(val) // u8

    // Configure the L-queue ECN classifier
    // Associated type: "Dualpi2EcnMask" (enum)
    .push_ecn_mask(val) // u8

    // Split aggregated skb or not
    // Associated type: "Dualpi2SplitGso" (enum)
    .push_split_gso(val) // u8
  .end_nested()
  .sub_nested_options_etf()
    .push_parms(val) // PushTcEtfQopt
  .end_nested()
  .sub_nested_options_ets()
    .push_nbands(val) // u8
    .push_nstrict(val) // u8
    .nested_quanta()
      .push_nbands(val) // u8
      .push_nstrict(val) // u8
      .nested_quanta()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_quanta_band(val) // u32
      .nested_priomap()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_priomap_band(val) // u8
    .end_nested()

    // Attribute may repeat multiple times (treat it as array)
    .push_quanta_band(val) // u32
    .nested_priomap()
      .push_nbands(val) // u8
      .push_nstrict(val) // u8
      .nested_quanta()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_quanta_band(val) // u32
      .nested_priomap()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_priomap_band(val) // u8
    .end_nested()

    // Attribute may repeat multiple times (treat it as array)
    .push_priomap_band(val) // u8
  .end_nested()
  .sub_nested_options_flow()
    .push_keys(val) // u32
    .push_mode(val) // u32
    .push_baseclass(val) // u32
    .push_rshift(val) // u32
    .push_addend(val) // u32
    .push_mask(val) // u32
    .push_xor(val) // u32
    .push_divisor(val) // u32
    .push_act(val) // &[u8]
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_ematches(val) // &[u8]
    .push_perturb(val) // u32
  .end_nested()
  .sub_nested_options_flower()
    .push_classid(val) // u32
    .push_indev(val) // &CStr
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_key_eth_dst(val) // &[u8]
    .push_key_eth_dst_mask(val) // &[u8]
    .push_key_eth_src(val) // &[u8]
    .push_key_eth_src_mask(val) // &[u8]
    .push_key_eth_type(val) // u16
    .push_key_ip_proto(val) // u8
    .push_key_ipv4_src(val) // Ipv4Addr
    .push_key_ipv4_src_mask(val) // Ipv4Addr
    .push_key_ipv4_dst(val) // Ipv4Addr
    .push_key_ipv4_dst_mask(val) // Ipv4Addr
    .push_key_ipv6_src(val) // &[u8]
    .push_key_ipv6_src_mask(val) // &[u8]
    .push_key_ipv6_dst(val) // &[u8]
    .push_key_ipv6_dst_mask(val) // &[u8]
    .push_key_tcp_src(val) // u16
    .push_key_tcp_dst(val) // u16
    .push_key_udp_src(val) // u16
    .push_key_udp_dst(val) // u16

    // Associated type: "ClsFlags" (1 bit per enumeration)
    .push_flags(val) // u32
    .push_key_vlan_id(val) // u16
    .push_key_vlan_prio(val) // u8
    .push_key_vlan_eth_type(val) // u16
    .push_key_enc_key_id(val) // u32
    .push_key_enc_ipv4_src(val) // Ipv4Addr
    .push_key_enc_ipv4_src_mask(val) // Ipv4Addr
    .push_key_enc_ipv4_dst(val) // Ipv4Addr
    .push_key_enc_ipv4_dst_mask(val) // Ipv4Addr
    .push_key_enc_ipv6_src(val) // &[u8]
    .push_key_enc_ipv6_src_mask(val) // &[u8]
    .push_key_enc_ipv6_dst(val) // &[u8]
    .push_key_enc_ipv6_dst_mask(val) // &[u8]
    .push_key_tcp_src_mask(val) // u16
    .push_key_tcp_dst_mask(val) // u16
    .push_key_udp_src_mask(val) // u16
    .push_key_udp_dst_mask(val) // u16
    .push_key_sctp_src_mask(val) // u16
    .push_key_sctp_dst_mask(val) // u16
    .push_key_sctp_src(val) // u16
    .push_key_sctp_dst(val) // u16
    .push_key_enc_udp_src_port(val) // u16
    .push_key_enc_udp_src_port_mask(val) // u16
    .push_key_enc_udp_dst_port(val) // u16
    .push_key_enc_udp_dst_port_mask(val) // u16

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_flags(val) // u32

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_flags_mask(val) // u32
    .push_key_icmpv4_code(val) // u8
    .push_key_icmpv4_code_mask(val) // u8
    .push_key_icmpv4_type(val) // u8
    .push_key_icmpv4_type_mask(val) // u8
    .push_key_icmpv6_code(val) // u8
    .push_key_icmpv6_code_mask(val) // u8
    .push_key_icmpv6_type(val) // u8
    .push_key_icmpv6_type_mask(val) // u8
    .push_key_arp_sip(val) // u32
    .push_key_arp_sip_mask(val) // u32
    .push_key_arp_tip(val) // u32
    .push_key_arp_tip_mask(val) // u32
    .push_key_arp_op(val) // u8
    .push_key_arp_op_mask(val) // u8
    .push_key_arp_sha(val) // &[u8]
    .push_key_arp_sha_mask(val) // &[u8]
    .push_key_arp_tha(val) // &[u8]
    .push_key_arp_tha_mask(val) // &[u8]
    .push_key_mpls_ttl(val) // u8
    .push_key_mpls_bos(val) // u8
    .push_key_mpls_tc(val) // u8
    .push_key_mpls_label(val) // u32
    .push_key_tcp_flags(val) // u16
    .push_key_tcp_flags_mask(val) // u16
    .push_key_ip_tos(val) // u8
    .push_key_ip_tos_mask(val) // u8
    .push_key_ip_ttl(val) // u8
    .push_key_ip_ttl_mask(val) // u8
    .push_key_cvlan_id(val) // u16
    .push_key_cvlan_prio(val) // u8
    .push_key_cvlan_eth_type(val) // u16
    .push_key_enc_ip_tos(val) // u8
    .push_key_enc_ip_tos_mask(val) // u8
    .push_key_enc_ip_ttl(val) // u8
    .push_key_enc_ip_ttl_mask(val) // u8
    .nested_key_enc_opts()
      .nested_geneve()
        .push_class(val) // u16
        .push_type(val) // u8
        .push_data(val) // &[u8]
      .end_nested()
      .nested_vxlan()
        .push_gbp(val) // u32
      .end_nested()
      .nested_erspan()
        .push_ver(val) // u8
        .push_index(val) // u32
        .push_dir(val) // u8
        .push_hwid(val) // u8
      .end_nested()
      .nested_gtp()
        .push_pdu_type(val) // u8
        .push_qfi(val) // u8
      .end_nested()
    .end_nested()
    .nested_key_enc_opts_mask()
      .nested_geneve()
        .push_class(val) // u16
        .push_type(val) // u8
        .push_data(val) // &[u8]
      .end_nested()
      .nested_vxlan()
        .push_gbp(val) // u32
      .end_nested()
      .nested_erspan()
        .push_ver(val) // u8
        .push_index(val) // u32
        .push_dir(val) // u8
        .push_hwid(val) // u8
      .end_nested()
      .nested_gtp()
        .push_pdu_type(val) // u8
        .push_qfi(val) // u8
      .end_nested()
    .end_nested()
    .push_in_hw_count(val) // u32
    .push_key_port_src_min(val) // u16
    .push_key_port_src_max(val) // u16
    .push_key_port_dst_min(val) // u16
    .push_key_port_dst_max(val) // u16
    .push_key_ct_state(val) // u16
    .push_key_ct_state_mask(val) // u16
    .push_key_ct_zone(val) // u16
    .push_key_ct_zone_mask(val) // u16
    .push_key_ct_mark(val) // u32
    .push_key_ct_mark_mask(val) // u32
    .push_key_ct_labels(val) // &[u8]
    .push_key_ct_labels_mask(val) // &[u8]
    .nested_key_mpls_opts()
      .push_lse_depth(val) // u8
      .push_lse_ttl(val) // u8
      .push_lse_bos(val) // u8
      .push_lse_tc(val) // u8
      .push_lse_label(val) // u32
    .end_nested()
    .push_key_hash(val) // u32
    .push_key_hash_mask(val) // u32
    .push_key_num_of_vlans(val) // u8
    .push_key_pppoe_sid(val) // u16
    .push_key_ppp_proto(val) // u16
    .push_key_l2tpv3_sid(val) // u32
    .push_l2_miss(val) // u8
    .nested_key_cfm()
      .push_md_level(val) // u8
      .push_opcode(val) // u8
    .end_nested()
    .push_key_spi(val) // u32
    .push_key_spi_mask(val) // u32

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_enc_flags(val) // u32

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_enc_flags_mask(val) // u32
  .end_nested()
  .sub_nested_options_fq()

    // Limit of total number of packets in queue
    .push_plimit(val) // u32

    // Limit of packets per flow
    .push_flow_plimit(val) // u32

    // RR quantum
    .push_quantum(val) // u32

    // RR quantum for new flow
    .push_initial_quantum(val) // u32

    // Enable / disable rate limiting
    .push_rate_enable(val) // u32

    // Obsolete, do not use
    .push_flow_default_rate(val) // u32

    // Per flow max rate
    .push_flow_max_rate(val) // u32

    // log2(number of buckets)
    .push_buckets_log(val) // u32

    // Flow credit refill delay in usec
    .push_flow_refill_delay(val) // u32

    // Mask applied to orphaned skb hashes
    .push_orphan_mask(val) // u32

    // Per packet delay under this rate
    .push_low_rate_threshold(val) // u32

    // DCTCP-like CE marking threshold
    .push_ce_threshold(val) // u32
    .push_timer_slack(val) // u32

    // Time horizon in usec
    .push_horizon(val) // u32

    // Drop packets beyond horizon, or cap their EDT
    .push_horizon_drop(val) // u8
    .push_priomap(val) // PushTcPrioQopt

    // Weights for each band
    .push_weights(val) // &[u8]
  .end_nested()
  .sub_nested_options_fq_codel()
    .push_target(val) // u32
    .push_limit(val) // u32
    .push_interval(val) // u32
    .push_ecn(val) // u32
    .push_flows(val) // u32
    .push_quantum(val) // u32
    .push_ce_threshold(val) // u32
    .push_drop_batch_size(val) // u32
    .push_memory_limit(val) // u32
    .push_ce_threshold_selector(val) // u8
    .push_ce_threshold_mask(val) // u8
  .end_nested()
  .sub_nested_options_fq_pie()
    .push_limit(val) // u32
    .push_flows(val) // u32
    .push_target(val) // u32
    .push_tupdate(val) // u32
    .push_alpha(val) // u32
    .push_beta(val) // u32
    .push_quantum(val) // u32
    .push_memory_limit(val) // u32
    .push_ecn_prob(val) // u32
    .push_ecn(val) // u32
    .push_bytemode(val) // u32
    .push_dq_rate_estimator(val) // u32
  .end_nested()
  .sub_nested_options_fw()
    .push_classid(val) // u32
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_indev(val) // &CStr
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_mask(val) // u32
  .end_nested()
  .sub_nested_options_gred()
    .push_parms(val) // &[u8]
    .push_stab(val) // &[u8]
    .push_dps(val) // PushTcGredSopt
    .push_max_p(val) // &[u8]
    .push_limit(val) // u32
    .nested_vq_list()

      // Attribute may repeat multiple times (treat it as array)
      .nested_entry()
        .push_pad(val) // &[u8]
        .push_dp(val) // u32
        .push_stat_bytes(val) // u64
        .push_stat_packets(val) // u32
        .push_stat_backlog(val) // u32
        .push_stat_prob_drop(val) // u32
        .push_stat_prob_mark(val) // u32
        .push_stat_forced_drop(val) // u32
        .push_stat_forced_mark(val) // u32
        .push_stat_pdrop(val) // u32
        .push_stat_other(val) // u32
        .push_flags(val) // u32
      .end_nested()
    .end_nested()
  .end_nested()
  .sub_nested_options_hfsc(fixed_header) // PushTcHfscQopt
  .sub_nested_options_hhf()
    .push_backlog_limit(val) // u32
    .push_quantum(val) // u32
    .push_hh_flows_limit(val) // u32
    .push_reset_timeout(val) // u32
    .push_admit_bytes(val) // u32
    .push_evict_timeout(val) // u32
    .push_non_hh_weight(val) // u32
  .end_nested()
  .sub_nested_options_htb()
    .push_parms(val) // PushTcHtbOpt
    .push_init(val) // PushTcHtbGlob
    .push_ctab(val) // &[u8]
    .push_rtab(val) // &[u8]
    .push_direct_qlen(val) // u32
    .push_rate64(val) // u64
    .push_ceil64(val) // u64
    .push_pad(val) // &[u8]
    .push_offload(val) // ()
  .end_nested()
  .sub_nested_options_ingress()
  .sub_nested_options_matchall()
    .push_classid(val) // u32
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_flags(val) // u32
    .push_pcnt(val) // PushTcMatchallPcnt
    .push_pad(val) // &[u8]
  .end_nested()
  .sub_nested_options_mq()
  .sub_nested_options_mqprio(fixed_header) // PushTcMqprioQopt
  .sub_nested_options_multiq(fixed_header) // PushTcMultiqQopt
  .sub_nested_options_netem(fixed_header) // PushTcNetemQopt
    .push_corr(val) // PushTcNetemCorr
    .push_delay_dist(val) // &[u8]
    .push_reorder(val) // PushTcNetemReorder
    .push_corrupt(val) // PushTcNetemCorrupt
    .nested_loss()

      // General Intuitive - 4 state model
      .push_gi(val) // PushTcNetemGimodel

      // Gilbert Elliot models
      .push_ge(val) // PushTcNetemGemodel
    .end_nested()
    .push_rate(val) // PushTcNetemRate
    .push_ecn(val) // u32
    .push_rate64(val) // u64
    .push_pad(val) // u32
    .push_latency64(val) // i64
    .push_jitter64(val) // i64
    .push_slot(val) // PushTcNetemSlot
    .push_slot_dist(val) // &[u8]
    .push_prng_seed(val) // u64
  .end_nested()
  .sub_nested_options_pfifo(fixed_header) // PushTcFifoQopt
  .sub_nested_options_pfifo_fast(fixed_header) // PushTcPrioQopt
  .sub_nested_options_pfifo_head_drop(fixed_header) // PushTcFifoQopt
  .sub_nested_options_pie()
    .push_target(val) // u32
    .push_limit(val) // u32
    .push_tupdate(val) // u32
    .push_alpha(val) // u32
    .push_beta(val) // u32
    .push_ecn(val) // u32
    .push_bytemode(val) // u32
    .push_dq_rate_estimator(val) // u32
  .end_nested()
  .sub_nested_options_plug(fixed_header) // PushTcPlugQopt
  .sub_nested_options_prio(fixed_header) // PushTcPrioQopt
  .sub_nested_options_qfq()
    .push_weight(val) // u32
    .push_lmax(val) // u32
  .end_nested()
  .sub_nested_options_red()
    .push_parms(val) // PushTcRedQopt
    .push_stab(val) // &[u8]
    .push_max_p(val) // u32
    .push_flags(val) // PushBuiltinBitfield32
    .push_early_drop_block(val) // u32
    .push_mark_block(val) // u32
  .end_nested()
  .sub_nested_options_route()
    .push_classid(val) // u32
    .push_to(val) // u32
    .push_from(val) // u32
    .push_iif(val) // u32
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
  .end_nested()
  .sub_nested_options_sfb(fixed_header) // PushTcSfbQopt
  .sub_nested_options_sfq(fixed_header) // PushTcSfqQoptV1
  .sub_nested_options_taprio()
    .push_priomap(val) // PushTcMqprioQopt
    .nested_sched_entry_list()

      // Attribute may repeat multiple times (treat it as array)
      .nested_entry()
        .push_index(val) // u32
        .push_cmd(val) // u8
        .push_gate_mask(val) // u32
        .push_interval(val) // u32
      .end_nested()
    .end_nested()
    .push_sched_base_time(val) // i64
    .nested_sched_single_entry()
      .push_index(val) // u32
      .push_cmd(val) // u8
      .push_gate_mask(val) // u32
      .push_interval(val) // u32
    .end_nested()
    .push_sched_clockid(val) // i32
    .push_pad(val) // &[u8]
    .push_admin_sched(val) // &[u8]
    .push_sched_cycle_time(val) // i64
    .push_sched_cycle_time_extension(val) // i64
    .push_flags(val) // u32
    .push_txtime_delay(val) // u32
    .nested_tc_entry()
      .push_index(val) // u32
      .push_max_sdu(val) // u32
      .push_fp(val) // u32
    .end_nested()
  .end_nested()
  .sub_nested_options_tbf()
    .push_parms(val) // PushTcTbfQopt
    .push_rtab(val) // &[u8]
    .push_ptab(val) // &[u8]
    .push_rate64(val) // u64
    .push_prate64(val) // u64
    .push_burst(val) // u32
    .push_pburst(val) // u32
    .push_pad(val) // &[u8]
  .end_nested()
  .sub_nested_options_u32()
    .push_classid(val) // u32
    .push_hash(val) // u32
    .push_link(val) // u32
    .push_divisor(val) // u32
    .push_sel(val) // PushTcU32Sel
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_indev(val) // &CStr
    .push_pcnt(val) // PushTcU32Pcnt
    .push_mark(val) // PushTcU32Mark
    .push_flags(val) // u32
    .push_pad(val) // &[u8]
  .end_nested()
  .push_rate(val) // PushGnetEstimator
  .push_chain(val) // u32
  .push_ingress_block(val) // u32
  .push_egress_block(val) // u32
  ;
```

### Do (reply)

```rust
let attrs = OpNewqdiscDoReply::new(buf);

// No attributes
```

# Operation "delqdisc"

## Do (request)

```rust
PushOpDelqdiscDoRequest::new(&mut vec)
  ;
```

### Do (reply)

```rust
let attrs = OpDelqdiscDoReply::new(buf);

// No attributes
```

# Operation "getqdisc"

## Do (request)

```rust
PushOpGetqdiscDoRequest::new(&mut vec)
  .push_dump_invisible(val) // ()
  ;
```

### Do (reply)

```rust
let attrs = OpGetqdiscDoReply::new(buf);

attrs.get_kind(); // &CStr
attrs.get_options(); // submessage
attrs.get_stats(); // PushTcStats
attrs.get_xstats(); // submessage
attrs.get_rate(); // PushGnetEstimator
attrs.get_fcnt(); // u32
{ // Nested Stats2
  let attrs = attrs.get_stats2();
  attrs.get_basic(); // PushGnetStatsBasic
  attrs.get_rate_est(); // PushGnetStatsRateEst
  attrs.get_queue(); // PushGnetStatsQueue
  attrs.get_app(); // submessage
  attrs.get_rate_est64(); // PushGnetStatsRateEst64
  attrs.get_pad(); // &[u8]
  attrs.get_basic_hw(); // PushGnetStatsBasic
  attrs.get_pkt64(); // u64
}
{ // Nested Stab
  let attrs = attrs.get_stab();
  attrs.get_base(); // PushTcSizespec
  attrs.get_data(); // &[u8]
}
attrs.get_chain(); // u32
attrs.get_ingress_block(); // u32
attrs.get_egress_block(); // u32
```

## Dump (request)

```rust
PushOpGetqdiscDumpRequest::new(&mut vec)
  .push_dump_invisible(val) // ()
  ;
```

### Dump (reply)

```rust
let attrs = OpGetqdiscDumpReply::new(buf);

attrs.get_kind(); // &CStr
attrs.get_options(); // submessage
attrs.get_stats(); // PushTcStats
attrs.get_xstats(); // submessage
attrs.get_rate(); // PushGnetEstimator
attrs.get_fcnt(); // u32
{ // Nested Stats2
  let attrs = attrs.get_stats2();
  attrs.get_basic(); // PushGnetStatsBasic
  attrs.get_rate_est(); // PushGnetStatsRateEst
  attrs.get_queue(); // PushGnetStatsQueue
  attrs.get_app(); // submessage
  attrs.get_rate_est64(); // PushGnetStatsRateEst64
  attrs.get_pad(); // &[u8]
  attrs.get_basic_hw(); // PushGnetStatsBasic
  attrs.get_pkt64(); // u64
}
{ // Nested Stab
  let attrs = attrs.get_stab();
  attrs.get_base(); // PushTcSizespec
  attrs.get_data(); // &[u8]
}
attrs.get_chain(); // u32
attrs.get_ingress_block(); // u32
attrs.get_egress_block(); // u32
```

# Operation "newtclass"

## Do (request)

```rust
PushOpNewtclassDoRequest::new(&mut vec)
  .push_kind(val) // &CStr
  .sub_nested_options_basic()
    .push_classid(val) // u32
    .nested_ematches()
      .push_tree_hdr(val) // PushTcfEmatchTreeHdr
      .push_tree_list(val) // &[u8]
    .end_nested()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_pcnt(val) // PushTcBasicPcnt
    .push_pad(val) // &[u8]
  .end_nested()
  .sub_nested_options_bpf()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_classid(val) // u32
    .push_ops_len(val) // u16
    .push_ops(val) // &[u8]
    .push_fd(val) // u32
    .push_name(val) // &CStr
    .push_flags(val) // u32
    .push_flags_gen(val) // u32
    .push_tag(val) // &[u8]
    .push_id(val) // u32
  .end_nested()
  .sub_nested_options_bfifo(fixed_header) // PushTcFifoQopt
  .sub_nested_options_cake()
    .push_pad(val) // &[u8]
    .push_base_rate64(val) // u64
    .push_diffserv_mode(val) // u32
    .push_atm(val) // u32
    .push_flow_mode(val) // u32
    .push_overhead(val) // u32
    .push_rtt(val) // u32
    .push_target(val) // u32
    .push_autorate(val) // u32
    .push_memory(val) // u32
    .push_nat(val) // u32
    .push_raw(val) // u32
    .push_wash(val) // u32
    .push_mpu(val) // u32
    .push_ingress(val) // u32
    .push_ack_filter(val) // u32
    .push_split_gso(val) // u32
    .push_fwmark(val) // u32
  .end_nested()
  .sub_nested_options_cbs()
    .push_parms(val) // PushTcCbsQopt
  .end_nested()
  .sub_nested_options_cgroup()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_ematches(val) // &[u8]
  .end_nested()
  .sub_nested_options_choke()
    .push_parms(val) // PushTcRedQopt
    .push_stab(val) // &[u8]
    .push_max_p(val) // u32
  .end_nested()
  .sub_nested_options_clsact()
  .sub_nested_options_codel()
    .push_target(val) // u32
    .push_limit(val) // u32
    .push_interval(val) // u32
    .push_ecn(val) // u32
    .push_ce_threshold(val) // u32
  .end_nested()
  .sub_nested_options_drr()
    .push_quantum(val) // u32
  .end_nested()
  .sub_nested_options_dualpi2()

    // Limit of total number of packets in queue
    .push_limit(val) // u32

    // Memory limit of total number of packets in queue
    .push_memory_limit(val) // u32

    // Classic target delay in microseconds
    .push_target(val) // u32

    // Drop probability update interval time in microseconds
    .push_tupdate(val) // u32

    // Integral gain factor in Hz for PI controller
    .push_alpha(val) // u32

    // Proportional gain factor in Hz for PI controller
    .push_beta(val) // u32

    // L4S step marking threshold in packets
    .push_step_thresh_pkts(val) // u32

    // L4S Step marking threshold in microseconds
    .push_step_thresh_us(val) // u32

    // Packets enqueued to the L-queue can apply the step threshold when the queue length of L-queue is larger than this value. (0 is recommended)
    .push_min_qlen_step(val) // u32

    // Probability coupling factor between Classic and L4S (2 is recommended)
    .push_coupling(val) // u8

    // Control the overload strategy (drop to preserve latency or let the queue overflow)
    // Associated type: "Dualpi2DropOverload" (enum)
    .push_drop_overload(val) // u8

    // Decide where the Classic packets are PI-based dropped or marked
    // Associated type: "Dualpi2DropEarly" (enum)
    .push_drop_early(val) // u8

    // Classic WRR weight in percentage (from 0 to 100)
    .push_c_protection(val) // u8

    // Configure the L-queue ECN classifier
    // Associated type: "Dualpi2EcnMask" (enum)
    .push_ecn_mask(val) // u8

    // Split aggregated skb or not
    // Associated type: "Dualpi2SplitGso" (enum)
    .push_split_gso(val) // u8
  .end_nested()
  .sub_nested_options_etf()
    .push_parms(val) // PushTcEtfQopt
  .end_nested()
  .sub_nested_options_ets()
    .push_nbands(val) // u8
    .push_nstrict(val) // u8
    .nested_quanta()
      .push_nbands(val) // u8
      .push_nstrict(val) // u8
      .nested_quanta()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_quanta_band(val) // u32
      .nested_priomap()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_priomap_band(val) // u8
    .end_nested()

    // Attribute may repeat multiple times (treat it as array)
    .push_quanta_band(val) // u32
    .nested_priomap()
      .push_nbands(val) // u8
      .push_nstrict(val) // u8
      .nested_quanta()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_quanta_band(val) // u32
      .nested_priomap()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_priomap_band(val) // u8
    .end_nested()

    // Attribute may repeat multiple times (treat it as array)
    .push_priomap_band(val) // u8
  .end_nested()
  .sub_nested_options_flow()
    .push_keys(val) // u32
    .push_mode(val) // u32
    .push_baseclass(val) // u32
    .push_rshift(val) // u32
    .push_addend(val) // u32
    .push_mask(val) // u32
    .push_xor(val) // u32
    .push_divisor(val) // u32
    .push_act(val) // &[u8]
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_ematches(val) // &[u8]
    .push_perturb(val) // u32
  .end_nested()
  .sub_nested_options_flower()
    .push_classid(val) // u32
    .push_indev(val) // &CStr
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_key_eth_dst(val) // &[u8]
    .push_key_eth_dst_mask(val) // &[u8]
    .push_key_eth_src(val) // &[u8]
    .push_key_eth_src_mask(val) // &[u8]
    .push_key_eth_type(val) // u16
    .push_key_ip_proto(val) // u8
    .push_key_ipv4_src(val) // Ipv4Addr
    .push_key_ipv4_src_mask(val) // Ipv4Addr
    .push_key_ipv4_dst(val) // Ipv4Addr
    .push_key_ipv4_dst_mask(val) // Ipv4Addr
    .push_key_ipv6_src(val) // &[u8]
    .push_key_ipv6_src_mask(val) // &[u8]
    .push_key_ipv6_dst(val) // &[u8]
    .push_key_ipv6_dst_mask(val) // &[u8]
    .push_key_tcp_src(val) // u16
    .push_key_tcp_dst(val) // u16
    .push_key_udp_src(val) // u16
    .push_key_udp_dst(val) // u16

    // Associated type: "ClsFlags" (1 bit per enumeration)
    .push_flags(val) // u32
    .push_key_vlan_id(val) // u16
    .push_key_vlan_prio(val) // u8
    .push_key_vlan_eth_type(val) // u16
    .push_key_enc_key_id(val) // u32
    .push_key_enc_ipv4_src(val) // Ipv4Addr
    .push_key_enc_ipv4_src_mask(val) // Ipv4Addr
    .push_key_enc_ipv4_dst(val) // Ipv4Addr
    .push_key_enc_ipv4_dst_mask(val) // Ipv4Addr
    .push_key_enc_ipv6_src(val) // &[u8]
    .push_key_enc_ipv6_src_mask(val) // &[u8]
    .push_key_enc_ipv6_dst(val) // &[u8]
    .push_key_enc_ipv6_dst_mask(val) // &[u8]
    .push_key_tcp_src_mask(val) // u16
    .push_key_tcp_dst_mask(val) // u16
    .push_key_udp_src_mask(val) // u16
    .push_key_udp_dst_mask(val) // u16
    .push_key_sctp_src_mask(val) // u16
    .push_key_sctp_dst_mask(val) // u16
    .push_key_sctp_src(val) // u16
    .push_key_sctp_dst(val) // u16
    .push_key_enc_udp_src_port(val) // u16
    .push_key_enc_udp_src_port_mask(val) // u16
    .push_key_enc_udp_dst_port(val) // u16
    .push_key_enc_udp_dst_port_mask(val) // u16

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_flags(val) // u32

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_flags_mask(val) // u32
    .push_key_icmpv4_code(val) // u8
    .push_key_icmpv4_code_mask(val) // u8
    .push_key_icmpv4_type(val) // u8
    .push_key_icmpv4_type_mask(val) // u8
    .push_key_icmpv6_code(val) // u8
    .push_key_icmpv6_code_mask(val) // u8
    .push_key_icmpv6_type(val) // u8
    .push_key_icmpv6_type_mask(val) // u8
    .push_key_arp_sip(val) // u32
    .push_key_arp_sip_mask(val) // u32
    .push_key_arp_tip(val) // u32
    .push_key_arp_tip_mask(val) // u32
    .push_key_arp_op(val) // u8
    .push_key_arp_op_mask(val) // u8
    .push_key_arp_sha(val) // &[u8]
    .push_key_arp_sha_mask(val) // &[u8]
    .push_key_arp_tha(val) // &[u8]
    .push_key_arp_tha_mask(val) // &[u8]
    .push_key_mpls_ttl(val) // u8
    .push_key_mpls_bos(val) // u8
    .push_key_mpls_tc(val) // u8
    .push_key_mpls_label(val) // u32
    .push_key_tcp_flags(val) // u16
    .push_key_tcp_flags_mask(val) // u16
    .push_key_ip_tos(val) // u8
    .push_key_ip_tos_mask(val) // u8
    .push_key_ip_ttl(val) // u8
    .push_key_ip_ttl_mask(val) // u8
    .push_key_cvlan_id(val) // u16
    .push_key_cvlan_prio(val) // u8
    .push_key_cvlan_eth_type(val) // u16
    .push_key_enc_ip_tos(val) // u8
    .push_key_enc_ip_tos_mask(val) // u8
    .push_key_enc_ip_ttl(val) // u8
    .push_key_enc_ip_ttl_mask(val) // u8
    .nested_key_enc_opts()
      .nested_geneve()
        .push_class(val) // u16
        .push_type(val) // u8
        .push_data(val) // &[u8]
      .end_nested()
      .nested_vxlan()
        .push_gbp(val) // u32
      .end_nested()
      .nested_erspan()
        .push_ver(val) // u8
        .push_index(val) // u32
        .push_dir(val) // u8
        .push_hwid(val) // u8
      .end_nested()
      .nested_gtp()
        .push_pdu_type(val) // u8
        .push_qfi(val) // u8
      .end_nested()
    .end_nested()
    .nested_key_enc_opts_mask()
      .nested_geneve()
        .push_class(val) // u16
        .push_type(val) // u8
        .push_data(val) // &[u8]
      .end_nested()
      .nested_vxlan()
        .push_gbp(val) // u32
      .end_nested()
      .nested_erspan()
        .push_ver(val) // u8
        .push_index(val) // u32
        .push_dir(val) // u8
        .push_hwid(val) // u8
      .end_nested()
      .nested_gtp()
        .push_pdu_type(val) // u8
        .push_qfi(val) // u8
      .end_nested()
    .end_nested()
    .push_in_hw_count(val) // u32
    .push_key_port_src_min(val) // u16
    .push_key_port_src_max(val) // u16
    .push_key_port_dst_min(val) // u16
    .push_key_port_dst_max(val) // u16
    .push_key_ct_state(val) // u16
    .push_key_ct_state_mask(val) // u16
    .push_key_ct_zone(val) // u16
    .push_key_ct_zone_mask(val) // u16
    .push_key_ct_mark(val) // u32
    .push_key_ct_mark_mask(val) // u32
    .push_key_ct_labels(val) // &[u8]
    .push_key_ct_labels_mask(val) // &[u8]
    .nested_key_mpls_opts()
      .push_lse_depth(val) // u8
      .push_lse_ttl(val) // u8
      .push_lse_bos(val) // u8
      .push_lse_tc(val) // u8
      .push_lse_label(val) // u32
    .end_nested()
    .push_key_hash(val) // u32
    .push_key_hash_mask(val) // u32
    .push_key_num_of_vlans(val) // u8
    .push_key_pppoe_sid(val) // u16
    .push_key_ppp_proto(val) // u16
    .push_key_l2tpv3_sid(val) // u32
    .push_l2_miss(val) // u8
    .nested_key_cfm()
      .push_md_level(val) // u8
      .push_opcode(val) // u8
    .end_nested()
    .push_key_spi(val) // u32
    .push_key_spi_mask(val) // u32

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_enc_flags(val) // u32

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_enc_flags_mask(val) // u32
  .end_nested()
  .sub_nested_options_fq()

    // Limit of total number of packets in queue
    .push_plimit(val) // u32

    // Limit of packets per flow
    .push_flow_plimit(val) // u32

    // RR quantum
    .push_quantum(val) // u32

    // RR quantum for new flow
    .push_initial_quantum(val) // u32

    // Enable / disable rate limiting
    .push_rate_enable(val) // u32

    // Obsolete, do not use
    .push_flow_default_rate(val) // u32

    // Per flow max rate
    .push_flow_max_rate(val) // u32

    // log2(number of buckets)
    .push_buckets_log(val) // u32

    // Flow credit refill delay in usec
    .push_flow_refill_delay(val) // u32

    // Mask applied to orphaned skb hashes
    .push_orphan_mask(val) // u32

    // Per packet delay under this rate
    .push_low_rate_threshold(val) // u32

    // DCTCP-like CE marking threshold
    .push_ce_threshold(val) // u32
    .push_timer_slack(val) // u32

    // Time horizon in usec
    .push_horizon(val) // u32

    // Drop packets beyond horizon, or cap their EDT
    .push_horizon_drop(val) // u8
    .push_priomap(val) // PushTcPrioQopt

    // Weights for each band
    .push_weights(val) // &[u8]
  .end_nested()
  .sub_nested_options_fq_codel()
    .push_target(val) // u32
    .push_limit(val) // u32
    .push_interval(val) // u32
    .push_ecn(val) // u32
    .push_flows(val) // u32
    .push_quantum(val) // u32
    .push_ce_threshold(val) // u32
    .push_drop_batch_size(val) // u32
    .push_memory_limit(val) // u32
    .push_ce_threshold_selector(val) // u8
    .push_ce_threshold_mask(val) // u8
  .end_nested()
  .sub_nested_options_fq_pie()
    .push_limit(val) // u32
    .push_flows(val) // u32
    .push_target(val) // u32
    .push_tupdate(val) // u32
    .push_alpha(val) // u32
    .push_beta(val) // u32
    .push_quantum(val) // u32
    .push_memory_limit(val) // u32
    .push_ecn_prob(val) // u32
    .push_ecn(val) // u32
    .push_bytemode(val) // u32
    .push_dq_rate_estimator(val) // u32
  .end_nested()
  .sub_nested_options_fw()
    .push_classid(val) // u32
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_indev(val) // &CStr
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_mask(val) // u32
  .end_nested()
  .sub_nested_options_gred()
    .push_parms(val) // &[u8]
    .push_stab(val) // &[u8]
    .push_dps(val) // PushTcGredSopt
    .push_max_p(val) // &[u8]
    .push_limit(val) // u32
    .nested_vq_list()

      // Attribute may repeat multiple times (treat it as array)
      .nested_entry()
        .push_pad(val) // &[u8]
        .push_dp(val) // u32
        .push_stat_bytes(val) // u64
        .push_stat_packets(val) // u32
        .push_stat_backlog(val) // u32
        .push_stat_prob_drop(val) // u32
        .push_stat_prob_mark(val) // u32
        .push_stat_forced_drop(val) // u32
        .push_stat_forced_mark(val) // u32
        .push_stat_pdrop(val) // u32
        .push_stat_other(val) // u32
        .push_flags(val) // u32
      .end_nested()
    .end_nested()
  .end_nested()
  .sub_nested_options_hfsc(fixed_header) // PushTcHfscQopt
  .sub_nested_options_hhf()
    .push_backlog_limit(val) // u32
    .push_quantum(val) // u32
    .push_hh_flows_limit(val) // u32
    .push_reset_timeout(val) // u32
    .push_admit_bytes(val) // u32
    .push_evict_timeout(val) // u32
    .push_non_hh_weight(val) // u32
  .end_nested()
  .sub_nested_options_htb()
    .push_parms(val) // PushTcHtbOpt
    .push_init(val) // PushTcHtbGlob
    .push_ctab(val) // &[u8]
    .push_rtab(val) // &[u8]
    .push_direct_qlen(val) // u32
    .push_rate64(val) // u64
    .push_ceil64(val) // u64
    .push_pad(val) // &[u8]
    .push_offload(val) // ()
  .end_nested()
  .sub_nested_options_ingress()
  .sub_nested_options_matchall()
    .push_classid(val) // u32
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_flags(val) // u32
    .push_pcnt(val) // PushTcMatchallPcnt
    .push_pad(val) // &[u8]
  .end_nested()
  .sub_nested_options_mq()
  .sub_nested_options_mqprio(fixed_header) // PushTcMqprioQopt
  .sub_nested_options_multiq(fixed_header) // PushTcMultiqQopt
  .sub_nested_options_netem(fixed_header) // PushTcNetemQopt
    .push_corr(val) // PushTcNetemCorr
    .push_delay_dist(val) // &[u8]
    .push_reorder(val) // PushTcNetemReorder
    .push_corrupt(val) // PushTcNetemCorrupt
    .nested_loss()

      // General Intuitive - 4 state model
      .push_gi(val) // PushTcNetemGimodel

      // Gilbert Elliot models
      .push_ge(val) // PushTcNetemGemodel
    .end_nested()
    .push_rate(val) // PushTcNetemRate
    .push_ecn(val) // u32
    .push_rate64(val) // u64
    .push_pad(val) // u32
    .push_latency64(val) // i64
    .push_jitter64(val) // i64
    .push_slot(val) // PushTcNetemSlot
    .push_slot_dist(val) // &[u8]
    .push_prng_seed(val) // u64
  .end_nested()
  .sub_nested_options_pfifo(fixed_header) // PushTcFifoQopt
  .sub_nested_options_pfifo_fast(fixed_header) // PushTcPrioQopt
  .sub_nested_options_pfifo_head_drop(fixed_header) // PushTcFifoQopt
  .sub_nested_options_pie()
    .push_target(val) // u32
    .push_limit(val) // u32
    .push_tupdate(val) // u32
    .push_alpha(val) // u32
    .push_beta(val) // u32
    .push_ecn(val) // u32
    .push_bytemode(val) // u32
    .push_dq_rate_estimator(val) // u32
  .end_nested()
  .sub_nested_options_plug(fixed_header) // PushTcPlugQopt
  .sub_nested_options_prio(fixed_header) // PushTcPrioQopt
  .sub_nested_options_qfq()
    .push_weight(val) // u32
    .push_lmax(val) // u32
  .end_nested()
  .sub_nested_options_red()
    .push_parms(val) // PushTcRedQopt
    .push_stab(val) // &[u8]
    .push_max_p(val) // u32
    .push_flags(val) // PushBuiltinBitfield32
    .push_early_drop_block(val) // u32
    .push_mark_block(val) // u32
  .end_nested()
  .sub_nested_options_route()
    .push_classid(val) // u32
    .push_to(val) // u32
    .push_from(val) // u32
    .push_iif(val) // u32
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
  .end_nested()
  .sub_nested_options_sfb(fixed_header) // PushTcSfbQopt
  .sub_nested_options_sfq(fixed_header) // PushTcSfqQoptV1
  .sub_nested_options_taprio()
    .push_priomap(val) // PushTcMqprioQopt
    .nested_sched_entry_list()

      // Attribute may repeat multiple times (treat it as array)
      .nested_entry()
        .push_index(val) // u32
        .push_cmd(val) // u8
        .push_gate_mask(val) // u32
        .push_interval(val) // u32
      .end_nested()
    .end_nested()
    .push_sched_base_time(val) // i64
    .nested_sched_single_entry()
      .push_index(val) // u32
      .push_cmd(val) // u8
      .push_gate_mask(val) // u32
      .push_interval(val) // u32
    .end_nested()
    .push_sched_clockid(val) // i32
    .push_pad(val) // &[u8]
    .push_admin_sched(val) // &[u8]
    .push_sched_cycle_time(val) // i64
    .push_sched_cycle_time_extension(val) // i64
    .push_flags(val) // u32
    .push_txtime_delay(val) // u32
    .nested_tc_entry()
      .push_index(val) // u32
      .push_max_sdu(val) // u32
      .push_fp(val) // u32
    .end_nested()
  .end_nested()
  .sub_nested_options_tbf()
    .push_parms(val) // PushTcTbfQopt
    .push_rtab(val) // &[u8]
    .push_ptab(val) // &[u8]
    .push_rate64(val) // u64
    .push_prate64(val) // u64
    .push_burst(val) // u32
    .push_pburst(val) // u32
    .push_pad(val) // &[u8]
  .end_nested()
  .sub_nested_options_u32()
    .push_classid(val) // u32
    .push_hash(val) // u32
    .push_link(val) // u32
    .push_divisor(val) // u32
    .push_sel(val) // PushTcU32Sel
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_indev(val) // &CStr
    .push_pcnt(val) // PushTcU32Pcnt
    .push_mark(val) // PushTcU32Mark
    .push_flags(val) // u32
    .push_pad(val) // &[u8]
  .end_nested()
  .push_rate(val) // PushGnetEstimator
  .push_chain(val) // u32
  .push_ingress_block(val) // u32
  .push_egress_block(val) // u32
  ;
```

### Do (reply)

```rust
let attrs = OpNewtclassDoReply::new(buf);

// No attributes
```

# Operation "deltclass"

## Do (request)

```rust
PushOpDeltclassDoRequest::new(&mut vec)
  ;
```

### Do (reply)

```rust
let attrs = OpDeltclassDoReply::new(buf);

// No attributes
```

# Operation "gettclass"

## Do (request)

```rust
PushOpGettclassDoRequest::new(&mut vec)
  ;
```

### Do (reply)

```rust
let attrs = OpGettclassDoReply::new(buf);

attrs.get_kind(); // &CStr
attrs.get_options(); // submessage
attrs.get_stats(); // PushTcStats
attrs.get_xstats(); // submessage
attrs.get_rate(); // PushGnetEstimator
attrs.get_fcnt(); // u32
{ // Nested Stats2
  let attrs = attrs.get_stats2();
  attrs.get_basic(); // PushGnetStatsBasic
  attrs.get_rate_est(); // PushGnetStatsRateEst
  attrs.get_queue(); // PushGnetStatsQueue
  attrs.get_app(); // submessage
  attrs.get_rate_est64(); // PushGnetStatsRateEst64
  attrs.get_pad(); // &[u8]
  attrs.get_basic_hw(); // PushGnetStatsBasic
  attrs.get_pkt64(); // u64
}
{ // Nested Stab
  let attrs = attrs.get_stab();
  attrs.get_base(); // PushTcSizespec
  attrs.get_data(); // &[u8]
}
attrs.get_chain(); // u32
attrs.get_ingress_block(); // u32
attrs.get_egress_block(); // u32
```

# Operation "newtfilter"

## Do (request)

```rust
PushOpNewtfilterDoRequest::new(&mut vec)
  .push_kind(val) // &CStr
  .sub_nested_options_basic()
    .push_classid(val) // u32
    .nested_ematches()
      .push_tree_hdr(val) // PushTcfEmatchTreeHdr
      .push_tree_list(val) // &[u8]
    .end_nested()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_pcnt(val) // PushTcBasicPcnt
    .push_pad(val) // &[u8]
  .end_nested()
  .sub_nested_options_bpf()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_classid(val) // u32
    .push_ops_len(val) // u16
    .push_ops(val) // &[u8]
    .push_fd(val) // u32
    .push_name(val) // &CStr
    .push_flags(val) // u32
    .push_flags_gen(val) // u32
    .push_tag(val) // &[u8]
    .push_id(val) // u32
  .end_nested()
  .sub_nested_options_bfifo(fixed_header) // PushTcFifoQopt
  .sub_nested_options_cake()
    .push_pad(val) // &[u8]
    .push_base_rate64(val) // u64
    .push_diffserv_mode(val) // u32
    .push_atm(val) // u32
    .push_flow_mode(val) // u32
    .push_overhead(val) // u32
    .push_rtt(val) // u32
    .push_target(val) // u32
    .push_autorate(val) // u32
    .push_memory(val) // u32
    .push_nat(val) // u32
    .push_raw(val) // u32
    .push_wash(val) // u32
    .push_mpu(val) // u32
    .push_ingress(val) // u32
    .push_ack_filter(val) // u32
    .push_split_gso(val) // u32
    .push_fwmark(val) // u32
  .end_nested()
  .sub_nested_options_cbs()
    .push_parms(val) // PushTcCbsQopt
  .end_nested()
  .sub_nested_options_cgroup()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_ematches(val) // &[u8]
  .end_nested()
  .sub_nested_options_choke()
    .push_parms(val) // PushTcRedQopt
    .push_stab(val) // &[u8]
    .push_max_p(val) // u32
  .end_nested()
  .sub_nested_options_clsact()
  .sub_nested_options_codel()
    .push_target(val) // u32
    .push_limit(val) // u32
    .push_interval(val) // u32
    .push_ecn(val) // u32
    .push_ce_threshold(val) // u32
  .end_nested()
  .sub_nested_options_drr()
    .push_quantum(val) // u32
  .end_nested()
  .sub_nested_options_dualpi2()

    // Limit of total number of packets in queue
    .push_limit(val) // u32

    // Memory limit of total number of packets in queue
    .push_memory_limit(val) // u32

    // Classic target delay in microseconds
    .push_target(val) // u32

    // Drop probability update interval time in microseconds
    .push_tupdate(val) // u32

    // Integral gain factor in Hz for PI controller
    .push_alpha(val) // u32

    // Proportional gain factor in Hz for PI controller
    .push_beta(val) // u32

    // L4S step marking threshold in packets
    .push_step_thresh_pkts(val) // u32

    // L4S Step marking threshold in microseconds
    .push_step_thresh_us(val) // u32

    // Packets enqueued to the L-queue can apply the step threshold when the queue length of L-queue is larger than this value. (0 is recommended)
    .push_min_qlen_step(val) // u32

    // Probability coupling factor between Classic and L4S (2 is recommended)
    .push_coupling(val) // u8

    // Control the overload strategy (drop to preserve latency or let the queue overflow)
    // Associated type: "Dualpi2DropOverload" (enum)
    .push_drop_overload(val) // u8

    // Decide where the Classic packets are PI-based dropped or marked
    // Associated type: "Dualpi2DropEarly" (enum)
    .push_drop_early(val) // u8

    // Classic WRR weight in percentage (from 0 to 100)
    .push_c_protection(val) // u8

    // Configure the L-queue ECN classifier
    // Associated type: "Dualpi2EcnMask" (enum)
    .push_ecn_mask(val) // u8

    // Split aggregated skb or not
    // Associated type: "Dualpi2SplitGso" (enum)
    .push_split_gso(val) // u8
  .end_nested()
  .sub_nested_options_etf()
    .push_parms(val) // PushTcEtfQopt
  .end_nested()
  .sub_nested_options_ets()
    .push_nbands(val) // u8
    .push_nstrict(val) // u8
    .nested_quanta()
      .push_nbands(val) // u8
      .push_nstrict(val) // u8
      .nested_quanta()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_quanta_band(val) // u32
      .nested_priomap()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_priomap_band(val) // u8
    .end_nested()

    // Attribute may repeat multiple times (treat it as array)
    .push_quanta_band(val) // u32
    .nested_priomap()
      .push_nbands(val) // u8
      .push_nstrict(val) // u8
      .nested_quanta()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_quanta_band(val) // u32
      .nested_priomap()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_priomap_band(val) // u8
    .end_nested()

    // Attribute may repeat multiple times (treat it as array)
    .push_priomap_band(val) // u8
  .end_nested()
  .sub_nested_options_flow()
    .push_keys(val) // u32
    .push_mode(val) // u32
    .push_baseclass(val) // u32
    .push_rshift(val) // u32
    .push_addend(val) // u32
    .push_mask(val) // u32
    .push_xor(val) // u32
    .push_divisor(val) // u32
    .push_act(val) // &[u8]
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_ematches(val) // &[u8]
    .push_perturb(val) // u32
  .end_nested()
  .sub_nested_options_flower()
    .push_classid(val) // u32
    .push_indev(val) // &CStr
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_key_eth_dst(val) // &[u8]
    .push_key_eth_dst_mask(val) // &[u8]
    .push_key_eth_src(val) // &[u8]
    .push_key_eth_src_mask(val) // &[u8]
    .push_key_eth_type(val) // u16
    .push_key_ip_proto(val) // u8
    .push_key_ipv4_src(val) // Ipv4Addr
    .push_key_ipv4_src_mask(val) // Ipv4Addr
    .push_key_ipv4_dst(val) // Ipv4Addr
    .push_key_ipv4_dst_mask(val) // Ipv4Addr
    .push_key_ipv6_src(val) // &[u8]
    .push_key_ipv6_src_mask(val) // &[u8]
    .push_key_ipv6_dst(val) // &[u8]
    .push_key_ipv6_dst_mask(val) // &[u8]
    .push_key_tcp_src(val) // u16
    .push_key_tcp_dst(val) // u16
    .push_key_udp_src(val) // u16
    .push_key_udp_dst(val) // u16

    // Associated type: "ClsFlags" (1 bit per enumeration)
    .push_flags(val) // u32
    .push_key_vlan_id(val) // u16
    .push_key_vlan_prio(val) // u8
    .push_key_vlan_eth_type(val) // u16
    .push_key_enc_key_id(val) // u32
    .push_key_enc_ipv4_src(val) // Ipv4Addr
    .push_key_enc_ipv4_src_mask(val) // Ipv4Addr
    .push_key_enc_ipv4_dst(val) // Ipv4Addr
    .push_key_enc_ipv4_dst_mask(val) // Ipv4Addr
    .push_key_enc_ipv6_src(val) // &[u8]
    .push_key_enc_ipv6_src_mask(val) // &[u8]
    .push_key_enc_ipv6_dst(val) // &[u8]
    .push_key_enc_ipv6_dst_mask(val) // &[u8]
    .push_key_tcp_src_mask(val) // u16
    .push_key_tcp_dst_mask(val) // u16
    .push_key_udp_src_mask(val) // u16
    .push_key_udp_dst_mask(val) // u16
    .push_key_sctp_src_mask(val) // u16
    .push_key_sctp_dst_mask(val) // u16
    .push_key_sctp_src(val) // u16
    .push_key_sctp_dst(val) // u16
    .push_key_enc_udp_src_port(val) // u16
    .push_key_enc_udp_src_port_mask(val) // u16
    .push_key_enc_udp_dst_port(val) // u16
    .push_key_enc_udp_dst_port_mask(val) // u16

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_flags(val) // u32

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_flags_mask(val) // u32
    .push_key_icmpv4_code(val) // u8
    .push_key_icmpv4_code_mask(val) // u8
    .push_key_icmpv4_type(val) // u8
    .push_key_icmpv4_type_mask(val) // u8
    .push_key_icmpv6_code(val) // u8
    .push_key_icmpv6_code_mask(val) // u8
    .push_key_icmpv6_type(val) // u8
    .push_key_icmpv6_type_mask(val) // u8
    .push_key_arp_sip(val) // u32
    .push_key_arp_sip_mask(val) // u32
    .push_key_arp_tip(val) // u32
    .push_key_arp_tip_mask(val) // u32
    .push_key_arp_op(val) // u8
    .push_key_arp_op_mask(val) // u8
    .push_key_arp_sha(val) // &[u8]
    .push_key_arp_sha_mask(val) // &[u8]
    .push_key_arp_tha(val) // &[u8]
    .push_key_arp_tha_mask(val) // &[u8]
    .push_key_mpls_ttl(val) // u8
    .push_key_mpls_bos(val) // u8
    .push_key_mpls_tc(val) // u8
    .push_key_mpls_label(val) // u32
    .push_key_tcp_flags(val) // u16
    .push_key_tcp_flags_mask(val) // u16
    .push_key_ip_tos(val) // u8
    .push_key_ip_tos_mask(val) // u8
    .push_key_ip_ttl(val) // u8
    .push_key_ip_ttl_mask(val) // u8
    .push_key_cvlan_id(val) // u16
    .push_key_cvlan_prio(val) // u8
    .push_key_cvlan_eth_type(val) // u16
    .push_key_enc_ip_tos(val) // u8
    .push_key_enc_ip_tos_mask(val) // u8
    .push_key_enc_ip_ttl(val) // u8
    .push_key_enc_ip_ttl_mask(val) // u8
    .nested_key_enc_opts()
      .nested_geneve()
        .push_class(val) // u16
        .push_type(val) // u8
        .push_data(val) // &[u8]
      .end_nested()
      .nested_vxlan()
        .push_gbp(val) // u32
      .end_nested()
      .nested_erspan()
        .push_ver(val) // u8
        .push_index(val) // u32
        .push_dir(val) // u8
        .push_hwid(val) // u8
      .end_nested()
      .nested_gtp()
        .push_pdu_type(val) // u8
        .push_qfi(val) // u8
      .end_nested()
    .end_nested()
    .nested_key_enc_opts_mask()
      .nested_geneve()
        .push_class(val) // u16
        .push_type(val) // u8
        .push_data(val) // &[u8]
      .end_nested()
      .nested_vxlan()
        .push_gbp(val) // u32
      .end_nested()
      .nested_erspan()
        .push_ver(val) // u8
        .push_index(val) // u32
        .push_dir(val) // u8
        .push_hwid(val) // u8
      .end_nested()
      .nested_gtp()
        .push_pdu_type(val) // u8
        .push_qfi(val) // u8
      .end_nested()
    .end_nested()
    .push_in_hw_count(val) // u32
    .push_key_port_src_min(val) // u16
    .push_key_port_src_max(val) // u16
    .push_key_port_dst_min(val) // u16
    .push_key_port_dst_max(val) // u16
    .push_key_ct_state(val) // u16
    .push_key_ct_state_mask(val) // u16
    .push_key_ct_zone(val) // u16
    .push_key_ct_zone_mask(val) // u16
    .push_key_ct_mark(val) // u32
    .push_key_ct_mark_mask(val) // u32
    .push_key_ct_labels(val) // &[u8]
    .push_key_ct_labels_mask(val) // &[u8]
    .nested_key_mpls_opts()
      .push_lse_depth(val) // u8
      .push_lse_ttl(val) // u8
      .push_lse_bos(val) // u8
      .push_lse_tc(val) // u8
      .push_lse_label(val) // u32
    .end_nested()
    .push_key_hash(val) // u32
    .push_key_hash_mask(val) // u32
    .push_key_num_of_vlans(val) // u8
    .push_key_pppoe_sid(val) // u16
    .push_key_ppp_proto(val) // u16
    .push_key_l2tpv3_sid(val) // u32
    .push_l2_miss(val) // u8
    .nested_key_cfm()
      .push_md_level(val) // u8
      .push_opcode(val) // u8
    .end_nested()
    .push_key_spi(val) // u32
    .push_key_spi_mask(val) // u32

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_enc_flags(val) // u32

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_enc_flags_mask(val) // u32
  .end_nested()
  .sub_nested_options_fq()

    // Limit of total number of packets in queue
    .push_plimit(val) // u32

    // Limit of packets per flow
    .push_flow_plimit(val) // u32

    // RR quantum
    .push_quantum(val) // u32

    // RR quantum for new flow
    .push_initial_quantum(val) // u32

    // Enable / disable rate limiting
    .push_rate_enable(val) // u32

    // Obsolete, do not use
    .push_flow_default_rate(val) // u32

    // Per flow max rate
    .push_flow_max_rate(val) // u32

    // log2(number of buckets)
    .push_buckets_log(val) // u32

    // Flow credit refill delay in usec
    .push_flow_refill_delay(val) // u32

    // Mask applied to orphaned skb hashes
    .push_orphan_mask(val) // u32

    // Per packet delay under this rate
    .push_low_rate_threshold(val) // u32

    // DCTCP-like CE marking threshold
    .push_ce_threshold(val) // u32
    .push_timer_slack(val) // u32

    // Time horizon in usec
    .push_horizon(val) // u32

    // Drop packets beyond horizon, or cap their EDT
    .push_horizon_drop(val) // u8
    .push_priomap(val) // PushTcPrioQopt

    // Weights for each band
    .push_weights(val) // &[u8]
  .end_nested()
  .sub_nested_options_fq_codel()
    .push_target(val) // u32
    .push_limit(val) // u32
    .push_interval(val) // u32
    .push_ecn(val) // u32
    .push_flows(val) // u32
    .push_quantum(val) // u32
    .push_ce_threshold(val) // u32
    .push_drop_batch_size(val) // u32
    .push_memory_limit(val) // u32
    .push_ce_threshold_selector(val) // u8
    .push_ce_threshold_mask(val) // u8
  .end_nested()
  .sub_nested_options_fq_pie()
    .push_limit(val) // u32
    .push_flows(val) // u32
    .push_target(val) // u32
    .push_tupdate(val) // u32
    .push_alpha(val) // u32
    .push_beta(val) // u32
    .push_quantum(val) // u32
    .push_memory_limit(val) // u32
    .push_ecn_prob(val) // u32
    .push_ecn(val) // u32
    .push_bytemode(val) // u32
    .push_dq_rate_estimator(val) // u32
  .end_nested()
  .sub_nested_options_fw()
    .push_classid(val) // u32
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_indev(val) // &CStr
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_mask(val) // u32
  .end_nested()
  .sub_nested_options_gred()
    .push_parms(val) // &[u8]
    .push_stab(val) // &[u8]
    .push_dps(val) // PushTcGredSopt
    .push_max_p(val) // &[u8]
    .push_limit(val) // u32
    .nested_vq_list()

      // Attribute may repeat multiple times (treat it as array)
      .nested_entry()
        .push_pad(val) // &[u8]
        .push_dp(val) // u32
        .push_stat_bytes(val) // u64
        .push_stat_packets(val) // u32
        .push_stat_backlog(val) // u32
        .push_stat_prob_drop(val) // u32
        .push_stat_prob_mark(val) // u32
        .push_stat_forced_drop(val) // u32
        .push_stat_forced_mark(val) // u32
        .push_stat_pdrop(val) // u32
        .push_stat_other(val) // u32
        .push_flags(val) // u32
      .end_nested()
    .end_nested()
  .end_nested()
  .sub_nested_options_hfsc(fixed_header) // PushTcHfscQopt
  .sub_nested_options_hhf()
    .push_backlog_limit(val) // u32
    .push_quantum(val) // u32
    .push_hh_flows_limit(val) // u32
    .push_reset_timeout(val) // u32
    .push_admit_bytes(val) // u32
    .push_evict_timeout(val) // u32
    .push_non_hh_weight(val) // u32
  .end_nested()
  .sub_nested_options_htb()
    .push_parms(val) // PushTcHtbOpt
    .push_init(val) // PushTcHtbGlob
    .push_ctab(val) // &[u8]
    .push_rtab(val) // &[u8]
    .push_direct_qlen(val) // u32
    .push_rate64(val) // u64
    .push_ceil64(val) // u64
    .push_pad(val) // &[u8]
    .push_offload(val) // ()
  .end_nested()
  .sub_nested_options_ingress()
  .sub_nested_options_matchall()
    .push_classid(val) // u32
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_flags(val) // u32
    .push_pcnt(val) // PushTcMatchallPcnt
    .push_pad(val) // &[u8]
  .end_nested()
  .sub_nested_options_mq()
  .sub_nested_options_mqprio(fixed_header) // PushTcMqprioQopt
  .sub_nested_options_multiq(fixed_header) // PushTcMultiqQopt
  .sub_nested_options_netem(fixed_header) // PushTcNetemQopt
    .push_corr(val) // PushTcNetemCorr
    .push_delay_dist(val) // &[u8]
    .push_reorder(val) // PushTcNetemReorder
    .push_corrupt(val) // PushTcNetemCorrupt
    .nested_loss()

      // General Intuitive - 4 state model
      .push_gi(val) // PushTcNetemGimodel

      // Gilbert Elliot models
      .push_ge(val) // PushTcNetemGemodel
    .end_nested()
    .push_rate(val) // PushTcNetemRate
    .push_ecn(val) // u32
    .push_rate64(val) // u64
    .push_pad(val) // u32
    .push_latency64(val) // i64
    .push_jitter64(val) // i64
    .push_slot(val) // PushTcNetemSlot
    .push_slot_dist(val) // &[u8]
    .push_prng_seed(val) // u64
  .end_nested()
  .sub_nested_options_pfifo(fixed_header) // PushTcFifoQopt
  .sub_nested_options_pfifo_fast(fixed_header) // PushTcPrioQopt
  .sub_nested_options_pfifo_head_drop(fixed_header) // PushTcFifoQopt
  .sub_nested_options_pie()
    .push_target(val) // u32
    .push_limit(val) // u32
    .push_tupdate(val) // u32
    .push_alpha(val) // u32
    .push_beta(val) // u32
    .push_ecn(val) // u32
    .push_bytemode(val) // u32
    .push_dq_rate_estimator(val) // u32
  .end_nested()
  .sub_nested_options_plug(fixed_header) // PushTcPlugQopt
  .sub_nested_options_prio(fixed_header) // PushTcPrioQopt
  .sub_nested_options_qfq()
    .push_weight(val) // u32
    .push_lmax(val) // u32
  .end_nested()
  .sub_nested_options_red()
    .push_parms(val) // PushTcRedQopt
    .push_stab(val) // &[u8]
    .push_max_p(val) // u32
    .push_flags(val) // PushBuiltinBitfield32
    .push_early_drop_block(val) // u32
    .push_mark_block(val) // u32
  .end_nested()
  .sub_nested_options_route()
    .push_classid(val) // u32
    .push_to(val) // u32
    .push_from(val) // u32
    .push_iif(val) // u32
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
  .end_nested()
  .sub_nested_options_sfb(fixed_header) // PushTcSfbQopt
  .sub_nested_options_sfq(fixed_header) // PushTcSfqQoptV1
  .sub_nested_options_taprio()
    .push_priomap(val) // PushTcMqprioQopt
    .nested_sched_entry_list()

      // Attribute may repeat multiple times (treat it as array)
      .nested_entry()
        .push_index(val) // u32
        .push_cmd(val) // u8
        .push_gate_mask(val) // u32
        .push_interval(val) // u32
      .end_nested()
    .end_nested()
    .push_sched_base_time(val) // i64
    .nested_sched_single_entry()
      .push_index(val) // u32
      .push_cmd(val) // u8
      .push_gate_mask(val) // u32
      .push_interval(val) // u32
    .end_nested()
    .push_sched_clockid(val) // i32
    .push_pad(val) // &[u8]
    .push_admin_sched(val) // &[u8]
    .push_sched_cycle_time(val) // i64
    .push_sched_cycle_time_extension(val) // i64
    .push_flags(val) // u32
    .push_txtime_delay(val) // u32
    .nested_tc_entry()
      .push_index(val) // u32
      .push_max_sdu(val) // u32
      .push_fp(val) // u32
    .end_nested()
  .end_nested()
  .sub_nested_options_tbf()
    .push_parms(val) // PushTcTbfQopt
    .push_rtab(val) // &[u8]
    .push_ptab(val) // &[u8]
    .push_rate64(val) // u64
    .push_prate64(val) // u64
    .push_burst(val) // u32
    .push_pburst(val) // u32
    .push_pad(val) // &[u8]
  .end_nested()
  .sub_nested_options_u32()
    .push_classid(val) // u32
    .push_hash(val) // u32
    .push_link(val) // u32
    .push_divisor(val) // u32
    .push_sel(val) // PushTcU32Sel
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_indev(val) // &CStr
    .push_pcnt(val) // PushTcU32Pcnt
    .push_mark(val) // PushTcU32Mark
    .push_flags(val) // u32
    .push_pad(val) // &[u8]
  .end_nested()
  .push_rate(val) // PushGnetEstimator
  .push_chain(val) // u32
  .push_ingress_block(val) // u32
  .push_egress_block(val) // u32
  ;
```

### Do (reply)

```rust
let attrs = OpNewtfilterDoReply::new(buf);

// No attributes
```

# Operation "deltfilter"

## Do (request)

```rust
PushOpDeltfilterDoRequest::new(&mut vec)
  .push_chain(val) // u32
  .push_kind(val) // &CStr
  ;
```

### Do (reply)

```rust
let attrs = OpDeltfilterDoReply::new(buf);

// No attributes
```

# Operation "gettfilter"

## Do (request)

```rust
PushOpGettfilterDoRequest::new(&mut vec)
  .push_chain(val) // u32
  .push_kind(val) // &CStr
  ;
```

### Do (reply)

```rust
let attrs = OpGettfilterDoReply::new(buf);

attrs.get_kind(); // &CStr
attrs.get_options(); // submessage
attrs.get_stats(); // PushTcStats
attrs.get_xstats(); // submessage
attrs.get_rate(); // PushGnetEstimator
attrs.get_fcnt(); // u32
{ // Nested Stats2
  let attrs = attrs.get_stats2();
  attrs.get_basic(); // PushGnetStatsBasic
  attrs.get_rate_est(); // PushGnetStatsRateEst
  attrs.get_queue(); // PushGnetStatsQueue
  attrs.get_app(); // submessage
  attrs.get_rate_est64(); // PushGnetStatsRateEst64
  attrs.get_pad(); // &[u8]
  attrs.get_basic_hw(); // PushGnetStatsBasic
  attrs.get_pkt64(); // u64
}
{ // Nested Stab
  let attrs = attrs.get_stab();
  attrs.get_base(); // PushTcSizespec
  attrs.get_data(); // &[u8]
}
attrs.get_chain(); // u32
attrs.get_ingress_block(); // u32
attrs.get_egress_block(); // u32
```

## Dump (request)

```rust
PushOpGettfilterDumpRequest::new(&mut vec)
  .push_chain(val) // u32
  .push_dump_flags(val) // PushBuiltinBitfield32
  ;
```

### Dump (reply)

```rust
let attrs = OpGettfilterDumpReply::new(buf);

attrs.get_kind(); // &CStr
attrs.get_options(); // submessage
attrs.get_stats(); // PushTcStats
attrs.get_xstats(); // submessage
attrs.get_rate(); // PushGnetEstimator
attrs.get_fcnt(); // u32
{ // Nested Stats2
  let attrs = attrs.get_stats2();
  attrs.get_basic(); // PushGnetStatsBasic
  attrs.get_rate_est(); // PushGnetStatsRateEst
  attrs.get_queue(); // PushGnetStatsQueue
  attrs.get_app(); // submessage
  attrs.get_rate_est64(); // PushGnetStatsRateEst64
  attrs.get_pad(); // &[u8]
  attrs.get_basic_hw(); // PushGnetStatsBasic
  attrs.get_pkt64(); // u64
}
{ // Nested Stab
  let attrs = attrs.get_stab();
  attrs.get_base(); // PushTcSizespec
  attrs.get_data(); // &[u8]
}
attrs.get_chain(); // u32
attrs.get_ingress_block(); // u32
attrs.get_egress_block(); // u32
```

# Operation "newchain"

## Do (request)

```rust
PushOpNewchainDoRequest::new(&mut vec)
  .push_kind(val) // &CStr
  .sub_nested_options_basic()
    .push_classid(val) // u32
    .nested_ematches()
      .push_tree_hdr(val) // PushTcfEmatchTreeHdr
      .push_tree_list(val) // &[u8]
    .end_nested()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_pcnt(val) // PushTcBasicPcnt
    .push_pad(val) // &[u8]
  .end_nested()
  .sub_nested_options_bpf()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_classid(val) // u32
    .push_ops_len(val) // u16
    .push_ops(val) // &[u8]
    .push_fd(val) // u32
    .push_name(val) // &CStr
    .push_flags(val) // u32
    .push_flags_gen(val) // u32
    .push_tag(val) // &[u8]
    .push_id(val) // u32
  .end_nested()
  .sub_nested_options_bfifo(fixed_header) // PushTcFifoQopt
  .sub_nested_options_cake()
    .push_pad(val) // &[u8]
    .push_base_rate64(val) // u64
    .push_diffserv_mode(val) // u32
    .push_atm(val) // u32
    .push_flow_mode(val) // u32
    .push_overhead(val) // u32
    .push_rtt(val) // u32
    .push_target(val) // u32
    .push_autorate(val) // u32
    .push_memory(val) // u32
    .push_nat(val) // u32
    .push_raw(val) // u32
    .push_wash(val) // u32
    .push_mpu(val) // u32
    .push_ingress(val) // u32
    .push_ack_filter(val) // u32
    .push_split_gso(val) // u32
    .push_fwmark(val) // u32
  .end_nested()
  .sub_nested_options_cbs()
    .push_parms(val) // PushTcCbsQopt
  .end_nested()
  .sub_nested_options_cgroup()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_ematches(val) // &[u8]
  .end_nested()
  .sub_nested_options_choke()
    .push_parms(val) // PushTcRedQopt
    .push_stab(val) // &[u8]
    .push_max_p(val) // u32
  .end_nested()
  .sub_nested_options_clsact()
  .sub_nested_options_codel()
    .push_target(val) // u32
    .push_limit(val) // u32
    .push_interval(val) // u32
    .push_ecn(val) // u32
    .push_ce_threshold(val) // u32
  .end_nested()
  .sub_nested_options_drr()
    .push_quantum(val) // u32
  .end_nested()
  .sub_nested_options_dualpi2()

    // Limit of total number of packets in queue
    .push_limit(val) // u32

    // Memory limit of total number of packets in queue
    .push_memory_limit(val) // u32

    // Classic target delay in microseconds
    .push_target(val) // u32

    // Drop probability update interval time in microseconds
    .push_tupdate(val) // u32

    // Integral gain factor in Hz for PI controller
    .push_alpha(val) // u32

    // Proportional gain factor in Hz for PI controller
    .push_beta(val) // u32

    // L4S step marking threshold in packets
    .push_step_thresh_pkts(val) // u32

    // L4S Step marking threshold in microseconds
    .push_step_thresh_us(val) // u32

    // Packets enqueued to the L-queue can apply the step threshold when the queue length of L-queue is larger than this value. (0 is recommended)
    .push_min_qlen_step(val) // u32

    // Probability coupling factor between Classic and L4S (2 is recommended)
    .push_coupling(val) // u8

    // Control the overload strategy (drop to preserve latency or let the queue overflow)
    // Associated type: "Dualpi2DropOverload" (enum)
    .push_drop_overload(val) // u8

    // Decide where the Classic packets are PI-based dropped or marked
    // Associated type: "Dualpi2DropEarly" (enum)
    .push_drop_early(val) // u8

    // Classic WRR weight in percentage (from 0 to 100)
    .push_c_protection(val) // u8

    // Configure the L-queue ECN classifier
    // Associated type: "Dualpi2EcnMask" (enum)
    .push_ecn_mask(val) // u8

    // Split aggregated skb or not
    // Associated type: "Dualpi2SplitGso" (enum)
    .push_split_gso(val) // u8
  .end_nested()
  .sub_nested_options_etf()
    .push_parms(val) // PushTcEtfQopt
  .end_nested()
  .sub_nested_options_ets()
    .push_nbands(val) // u8
    .push_nstrict(val) // u8
    .nested_quanta()
      .push_nbands(val) // u8
      .push_nstrict(val) // u8
      .nested_quanta()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_quanta_band(val) // u32
      .nested_priomap()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_priomap_band(val) // u8
    .end_nested()

    // Attribute may repeat multiple times (treat it as array)
    .push_quanta_band(val) // u32
    .nested_priomap()
      .push_nbands(val) // u8
      .push_nstrict(val) // u8
      .nested_quanta()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_quanta_band(val) // u32
      .nested_priomap()
        // ...
      .end_nested()

      // Attribute may repeat multiple times (treat it as array)
      .push_priomap_band(val) // u8
    .end_nested()

    // Attribute may repeat multiple times (treat it as array)
    .push_priomap_band(val) // u8
  .end_nested()
  .sub_nested_options_flow()
    .push_keys(val) // u32
    .push_mode(val) // u32
    .push_baseclass(val) // u32
    .push_rshift(val) // u32
    .push_addend(val) // u32
    .push_mask(val) // u32
    .push_xor(val) // u32
    .push_divisor(val) // u32
    .push_act(val) // &[u8]
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_ematches(val) // &[u8]
    .push_perturb(val) // u32
  .end_nested()
  .sub_nested_options_flower()
    .push_classid(val) // u32
    .push_indev(val) // &CStr
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_key_eth_dst(val) // &[u8]
    .push_key_eth_dst_mask(val) // &[u8]
    .push_key_eth_src(val) // &[u8]
    .push_key_eth_src_mask(val) // &[u8]
    .push_key_eth_type(val) // u16
    .push_key_ip_proto(val) // u8
    .push_key_ipv4_src(val) // Ipv4Addr
    .push_key_ipv4_src_mask(val) // Ipv4Addr
    .push_key_ipv4_dst(val) // Ipv4Addr
    .push_key_ipv4_dst_mask(val) // Ipv4Addr
    .push_key_ipv6_src(val) // &[u8]
    .push_key_ipv6_src_mask(val) // &[u8]
    .push_key_ipv6_dst(val) // &[u8]
    .push_key_ipv6_dst_mask(val) // &[u8]
    .push_key_tcp_src(val) // u16
    .push_key_tcp_dst(val) // u16
    .push_key_udp_src(val) // u16
    .push_key_udp_dst(val) // u16

    // Associated type: "ClsFlags" (1 bit per enumeration)
    .push_flags(val) // u32
    .push_key_vlan_id(val) // u16
    .push_key_vlan_prio(val) // u8
    .push_key_vlan_eth_type(val) // u16
    .push_key_enc_key_id(val) // u32
    .push_key_enc_ipv4_src(val) // Ipv4Addr
    .push_key_enc_ipv4_src_mask(val) // Ipv4Addr
    .push_key_enc_ipv4_dst(val) // Ipv4Addr
    .push_key_enc_ipv4_dst_mask(val) // Ipv4Addr
    .push_key_enc_ipv6_src(val) // &[u8]
    .push_key_enc_ipv6_src_mask(val) // &[u8]
    .push_key_enc_ipv6_dst(val) // &[u8]
    .push_key_enc_ipv6_dst_mask(val) // &[u8]
    .push_key_tcp_src_mask(val) // u16
    .push_key_tcp_dst_mask(val) // u16
    .push_key_udp_src_mask(val) // u16
    .push_key_udp_dst_mask(val) // u16
    .push_key_sctp_src_mask(val) // u16
    .push_key_sctp_dst_mask(val) // u16
    .push_key_sctp_src(val) // u16
    .push_key_sctp_dst(val) // u16
    .push_key_enc_udp_src_port(val) // u16
    .push_key_enc_udp_src_port_mask(val) // u16
    .push_key_enc_udp_dst_port(val) // u16
    .push_key_enc_udp_dst_port_mask(val) // u16

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_flags(val) // u32

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_flags_mask(val) // u32
    .push_key_icmpv4_code(val) // u8
    .push_key_icmpv4_code_mask(val) // u8
    .push_key_icmpv4_type(val) // u8
    .push_key_icmpv4_type_mask(val) // u8
    .push_key_icmpv6_code(val) // u8
    .push_key_icmpv6_code_mask(val) // u8
    .push_key_icmpv6_type(val) // u8
    .push_key_icmpv6_type_mask(val) // u8
    .push_key_arp_sip(val) // u32
    .push_key_arp_sip_mask(val) // u32
    .push_key_arp_tip(val) // u32
    .push_key_arp_tip_mask(val) // u32
    .push_key_arp_op(val) // u8
    .push_key_arp_op_mask(val) // u8
    .push_key_arp_sha(val) // &[u8]
    .push_key_arp_sha_mask(val) // &[u8]
    .push_key_arp_tha(val) // &[u8]
    .push_key_arp_tha_mask(val) // &[u8]
    .push_key_mpls_ttl(val) // u8
    .push_key_mpls_bos(val) // u8
    .push_key_mpls_tc(val) // u8
    .push_key_mpls_label(val) // u32
    .push_key_tcp_flags(val) // u16
    .push_key_tcp_flags_mask(val) // u16
    .push_key_ip_tos(val) // u8
    .push_key_ip_tos_mask(val) // u8
    .push_key_ip_ttl(val) // u8
    .push_key_ip_ttl_mask(val) // u8
    .push_key_cvlan_id(val) // u16
    .push_key_cvlan_prio(val) // u8
    .push_key_cvlan_eth_type(val) // u16
    .push_key_enc_ip_tos(val) // u8
    .push_key_enc_ip_tos_mask(val) // u8
    .push_key_enc_ip_ttl(val) // u8
    .push_key_enc_ip_ttl_mask(val) // u8
    .nested_key_enc_opts()
      .nested_geneve()
        .push_class(val) // u16
        .push_type(val) // u8
        .push_data(val) // &[u8]
      .end_nested()
      .nested_vxlan()
        .push_gbp(val) // u32
      .end_nested()
      .nested_erspan()
        .push_ver(val) // u8
        .push_index(val) // u32
        .push_dir(val) // u8
        .push_hwid(val) // u8
      .end_nested()
      .nested_gtp()
        .push_pdu_type(val) // u8
        .push_qfi(val) // u8
      .end_nested()
    .end_nested()
    .nested_key_enc_opts_mask()
      .nested_geneve()
        .push_class(val) // u16
        .push_type(val) // u8
        .push_data(val) // &[u8]
      .end_nested()
      .nested_vxlan()
        .push_gbp(val) // u32
      .end_nested()
      .nested_erspan()
        .push_ver(val) // u8
        .push_index(val) // u32
        .push_dir(val) // u8
        .push_hwid(val) // u8
      .end_nested()
      .nested_gtp()
        .push_pdu_type(val) // u8
        .push_qfi(val) // u8
      .end_nested()
    .end_nested()
    .push_in_hw_count(val) // u32
    .push_key_port_src_min(val) // u16
    .push_key_port_src_max(val) // u16
    .push_key_port_dst_min(val) // u16
    .push_key_port_dst_max(val) // u16
    .push_key_ct_state(val) // u16
    .push_key_ct_state_mask(val) // u16
    .push_key_ct_zone(val) // u16
    .push_key_ct_zone_mask(val) // u16
    .push_key_ct_mark(val) // u32
    .push_key_ct_mark_mask(val) // u32
    .push_key_ct_labels(val) // &[u8]
    .push_key_ct_labels_mask(val) // &[u8]
    .nested_key_mpls_opts()
      .push_lse_depth(val) // u8
      .push_lse_ttl(val) // u8
      .push_lse_bos(val) // u8
      .push_lse_tc(val) // u8
      .push_lse_label(val) // u32
    .end_nested()
    .push_key_hash(val) // u32
    .push_key_hash_mask(val) // u32
    .push_key_num_of_vlans(val) // u8
    .push_key_pppoe_sid(val) // u16
    .push_key_ppp_proto(val) // u16
    .push_key_l2tpv3_sid(val) // u32
    .push_l2_miss(val) // u8
    .nested_key_cfm()
      .push_md_level(val) // u8
      .push_opcode(val) // u8
    .end_nested()
    .push_key_spi(val) // u32
    .push_key_spi_mask(val) // u32

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_enc_flags(val) // u32

    // Associated type: "FlowerKeyCtrlFlags" (1 bit per enumeration)
    .push_key_enc_flags_mask(val) // u32
  .end_nested()
  .sub_nested_options_fq()

    // Limit of total number of packets in queue
    .push_plimit(val) // u32

    // Limit of packets per flow
    .push_flow_plimit(val) // u32

    // RR quantum
    .push_quantum(val) // u32

    // RR quantum for new flow
    .push_initial_quantum(val) // u32

    // Enable / disable rate limiting
    .push_rate_enable(val) // u32

    // Obsolete, do not use
    .push_flow_default_rate(val) // u32

    // Per flow max rate
    .push_flow_max_rate(val) // u32

    // log2(number of buckets)
    .push_buckets_log(val) // u32

    // Flow credit refill delay in usec
    .push_flow_refill_delay(val) // u32

    // Mask applied to orphaned skb hashes
    .push_orphan_mask(val) // u32

    // Per packet delay under this rate
    .push_low_rate_threshold(val) // u32

    // DCTCP-like CE marking threshold
    .push_ce_threshold(val) // u32
    .push_timer_slack(val) // u32

    // Time horizon in usec
    .push_horizon(val) // u32

    // Drop packets beyond horizon, or cap their EDT
    .push_horizon_drop(val) // u8
    .push_priomap(val) // PushTcPrioQopt

    // Weights for each band
    .push_weights(val) // &[u8]
  .end_nested()
  .sub_nested_options_fq_codel()
    .push_target(val) // u32
    .push_limit(val) // u32
    .push_interval(val) // u32
    .push_ecn(val) // u32
    .push_flows(val) // u32
    .push_quantum(val) // u32
    .push_ce_threshold(val) // u32
    .push_drop_batch_size(val) // u32
    .push_memory_limit(val) // u32
    .push_ce_threshold_selector(val) // u8
    .push_ce_threshold_mask(val) // u8
  .end_nested()
  .sub_nested_options_fq_pie()
    .push_limit(val) // u32
    .push_flows(val) // u32
    .push_target(val) // u32
    .push_tupdate(val) // u32
    .push_alpha(val) // u32
    .push_beta(val) // u32
    .push_quantum(val) // u32
    .push_memory_limit(val) // u32
    .push_ecn_prob(val) // u32
    .push_ecn(val) // u32
    .push_bytemode(val) // u32
    .push_dq_rate_estimator(val) // u32
  .end_nested()
  .sub_nested_options_fw()
    .push_classid(val) // u32
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .push_indev(val) // &CStr
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_mask(val) // u32
  .end_nested()
  .sub_nested_options_gred()
    .push_parms(val) // &[u8]
    .push_stab(val) // &[u8]
    .push_dps(val) // PushTcGredSopt
    .push_max_p(val) // &[u8]
    .push_limit(val) // u32
    .nested_vq_list()

      // Attribute may repeat multiple times (treat it as array)
      .nested_entry()
        .push_pad(val) // &[u8]
        .push_dp(val) // u32
        .push_stat_bytes(val) // u64
        .push_stat_packets(val) // u32
        .push_stat_backlog(val) // u32
        .push_stat_prob_drop(val) // u32
        .push_stat_prob_mark(val) // u32
        .push_stat_forced_drop(val) // u32
        .push_stat_forced_mark(val) // u32
        .push_stat_pdrop(val) // u32
        .push_stat_other(val) // u32
        .push_flags(val) // u32
      .end_nested()
    .end_nested()
  .end_nested()
  .sub_nested_options_hfsc(fixed_header) // PushTcHfscQopt
  .sub_nested_options_hhf()
    .push_backlog_limit(val) // u32
    .push_quantum(val) // u32
    .push_hh_flows_limit(val) // u32
    .push_reset_timeout(val) // u32
    .push_admit_bytes(val) // u32
    .push_evict_timeout(val) // u32
    .push_non_hh_weight(val) // u32
  .end_nested()
  .sub_nested_options_htb()
    .push_parms(val) // PushTcHtbOpt
    .push_init(val) // PushTcHtbGlob
    .push_ctab(val) // &[u8]
    .push_rtab(val) // &[u8]
    .push_direct_qlen(val) // u32
    .push_rate64(val) // u64
    .push_ceil64(val) // u64
    .push_pad(val) // &[u8]
    .push_offload(val) // ()
  .end_nested()
  .sub_nested_options_ingress()
  .sub_nested_options_matchall()
    .push_classid(val) // u32
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_flags(val) // u32
    .push_pcnt(val) // PushTcMatchallPcnt
    .push_pad(val) // &[u8]
  .end_nested()
  .sub_nested_options_mq()
  .sub_nested_options_mqprio(fixed_header) // PushTcMqprioQopt
  .sub_nested_options_multiq(fixed_header) // PushTcMultiqQopt
  .sub_nested_options_netem(fixed_header) // PushTcNetemQopt
    .push_corr(val) // PushTcNetemCorr
    .push_delay_dist(val) // &[u8]
    .push_reorder(val) // PushTcNetemReorder
    .push_corrupt(val) // PushTcNetemCorrupt
    .nested_loss()

      // General Intuitive - 4 state model
      .push_gi(val) // PushTcNetemGimodel

      // Gilbert Elliot models
      .push_ge(val) // PushTcNetemGemodel
    .end_nested()
    .push_rate(val) // PushTcNetemRate
    .push_ecn(val) // u32
    .push_rate64(val) // u64
    .push_pad(val) // u32
    .push_latency64(val) // i64
    .push_jitter64(val) // i64
    .push_slot(val) // PushTcNetemSlot
    .push_slot_dist(val) // &[u8]
    .push_prng_seed(val) // u64
  .end_nested()
  .sub_nested_options_pfifo(fixed_header) // PushTcFifoQopt
  .sub_nested_options_pfifo_fast(fixed_header) // PushTcPrioQopt
  .sub_nested_options_pfifo_head_drop(fixed_header) // PushTcFifoQopt
  .sub_nested_options_pie()
    .push_target(val) // u32
    .push_limit(val) // u32
    .push_tupdate(val) // u32
    .push_alpha(val) // u32
    .push_beta(val) // u32
    .push_ecn(val) // u32
    .push_bytemode(val) // u32
    .push_dq_rate_estimator(val) // u32
  .end_nested()
  .sub_nested_options_plug(fixed_header) // PushTcPlugQopt
  .sub_nested_options_prio(fixed_header) // PushTcPrioQopt
  .sub_nested_options_qfq()
    .push_weight(val) // u32
    .push_lmax(val) // u32
  .end_nested()
  .sub_nested_options_red()
    .push_parms(val) // PushTcRedQopt
    .push_stab(val) // &[u8]
    .push_max_p(val) // u32
    .push_flags(val) // PushBuiltinBitfield32
    .push_early_drop_block(val) // u32
    .push_mark_block(val) // u32
  .end_nested()
  .sub_nested_options_route()
    .push_classid(val) // u32
    .push_to(val) // u32
    .push_from(val) // u32
    .push_iif(val) // u32
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
  .end_nested()
  .sub_nested_options_sfb(fixed_header) // PushTcSfbQopt
  .sub_nested_options_sfq(fixed_header) // PushTcSfqQoptV1
  .sub_nested_options_taprio()
    .push_priomap(val) // PushTcMqprioQopt
    .nested_sched_entry_list()

      // Attribute may repeat multiple times (treat it as array)
      .nested_entry()
        .push_index(val) // u32
        .push_cmd(val) // u8
        .push_gate_mask(val) // u32
        .push_interval(val) // u32
      .end_nested()
    .end_nested()
    .push_sched_base_time(val) // i64
    .nested_sched_single_entry()
      .push_index(val) // u32
      .push_cmd(val) // u8
      .push_gate_mask(val) // u32
      .push_interval(val) // u32
    .end_nested()
    .push_sched_clockid(val) // i32
    .push_pad(val) // &[u8]
    .push_admin_sched(val) // &[u8]
    .push_sched_cycle_time(val) // i64
    .push_sched_cycle_time_extension(val) // i64
    .push_flags(val) // u32
    .push_txtime_delay(val) // u32
    .nested_tc_entry()
      .push_index(val) // u32
      .push_max_sdu(val) // u32
      .push_fp(val) // u32
    .end_nested()
  .end_nested()
  .sub_nested_options_tbf()
    .push_parms(val) // PushTcTbfQopt
    .push_rtab(val) // &[u8]
    .push_ptab(val) // &[u8]
    .push_rate64(val) // u64
    .push_prate64(val) // u64
    .push_burst(val) // u32
    .push_pburst(val) // u32
    .push_pad(val) // &[u8]
  .end_nested()
  .sub_nested_options_u32()
    .push_classid(val) // u32
    .push_hash(val) // u32
    .push_link(val) // u32
    .push_divisor(val) // u32
    .push_sel(val) // PushTcU32Sel
    .nested_police()
      .push_tbf(val) // PushTcPolice
      .push_rate(val) // &[u8]
      .push_peakrate(val) // &[u8]
      .push_avrate(val) // u32
      .push_result(val) // u32
      .push_tm(val) // PushTcfT
      .push_pad(val) // &[u8]
      .push_rate64(val) // u64
      .push_peakrate64(val) // u64
      .push_pktrate64(val) // u64
      .push_pktburst64(val) // u64
    .end_nested()
    .array_act()
      .entry_nested()
        .push_kind(val) // &CStr
        .sub_nested_options_bpf()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_ops_len(val) // u16
          .push_ops(val) // &[u8]
          .push_fd(val) // u32
          .push_name(val) // &CStr
          .push_pad(val) // &[u8]
          .push_tag(val) // &[u8]
          .push_id(val) // &[u8]
        .end_nested()
        .sub_nested_options_connmark()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_csum()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_ct()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_action(val) // u16
          .push_zone(val) // u16
          .push_mark(val) // u32
          .push_mark_mask(val) // u32
          .push_labels(val) // &[u8]
          .push_labels_mask(val) // &[u8]
          .push_nat_ipv4_min(val) // u32
          .push_nat_ipv4_max(val) // u32
          .push_nat_ipv6_min(val) // &[u8]
          .push_nat_ipv6_max(val) // &[u8]
          .push_nat_port_min(val) // u16
          .push_nat_port_max(val) // u16
          .push_pad(val) // &[u8]
          .push_helper_name(val) // &CStr
          .push_helper_family(val) // u8
          .push_helper_proto(val) // u8
        .end_nested()
        .sub_nested_options_ctinfo()
          .push_pad(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_act(val) // &[u8]
          .push_zone(val) // u16
          .push_parms_dscp_mask(val) // u32
          .push_parms_dscp_statemask(val) // u32
          .push_parms_cpmark_mask(val) // u32
          .push_stats_dscp_set(val) // u64
          .push_stats_dscp_error(val) // u64
          .push_stats_cpmark_set(val) // u64
        .end_nested()
        .sub_nested_options_gact()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_prob(val) // PushTcGactP
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_gate()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_priority(val) // i32
          .push_entry_list(val) // &[u8]
          .push_base_time(val) // u64
          .push_cycle_time(val) // u64
          .push_cycle_time_ext(val) // u64
          .push_flags(val) // u32
          .push_clockid(val) // i32
        .end_nested()
        .sub_nested_options_ife()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_type(val) // u16
          .push_metalst(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_mirred()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_pad(val) // &[u8]
          .push_blockid(val) // &[u8]
        .end_nested()
        .sub_nested_options_mpls()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcMpls
          .push_pad(val) // &[u8]
          .push_proto(val) // u16
          .push_label(val) // u32
          .push_tc(val) // u8
          .push_ttl(val) // u8
          .push_bos(val) // u8
        .end_nested()
        .sub_nested_options_nat()
          .push_parms(val) // &[u8]
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_pedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcPeditSel
          .push_pad(val) // &[u8]
          .push_parms_ex(val) // &[u8]
          .push_keys_ex(val) // &[u8]
          .push_key_ex(val) // &[u8]
        .end_nested()
        .sub_nested_options_police()
          .push_tbf(val) // PushTcPolice
          .push_rate(val) // &[u8]
          .push_peakrate(val) // &[u8]
          .push_avrate(val) // u32
          .push_result(val) // u32
          .push_tm(val) // PushTcfT
          .push_pad(val) // &[u8]
          .push_rate64(val) // u64
          .push_peakrate64(val) // u64
          .push_pktrate64(val) // u64
          .push_pktburst64(val) // u64
        .end_nested()
        .sub_nested_options_sample()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcGact
          .push_rate(val) // u32
          .push_trunc_size(val) // u32
          .push_psample_group(val) // u32
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_simple()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_data(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_skbedit()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_priority(val) // u32
          .push_queue_mapping(val) // u16
          .push_mark(val) // u32
          .push_pad(val) // &[u8]
          .push_ptype(val) // u16
          .push_mask(val) // u32
          .push_flags(val) // u64
          .push_queue_mapping_max(val) // u16
        .end_nested()
        .sub_nested_options_skbmod()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_dmac(val) // &[u8]
          .push_smac(val) // &[u8]
          .push_etype(val) // &[u8]
          .push_pad(val) // &[u8]
        .end_nested()
        .sub_nested_options_tunnel_key()
          .push_tm(val) // PushTcfT
          .push_parms(val) // &[u8]
          .push_enc_ipv4_src(val) // u32
          .push_enc_ipv4_dst(val) // u32
          .push_enc_ipv6_src(val) // &[u8]
          .push_enc_ipv6_dst(val) // &[u8]
          .push_enc_key_id(val) // u64
          .push_pad(val) // &[u8]
          .push_enc_dst_port(val) // u16
          .push_no_csum(val) // u8
          .push_enc_opts(val) // &[u8]
          .push_enc_tos(val) // u8
          .push_enc_ttl(val) // u8
          .push_no_frag(val) // ()
        .end_nested()
        .sub_nested_options_vlan()
          .push_tm(val) // PushTcfT
          .push_parms(val) // PushTcVlan
          .push_push_vlan_id(val) // u16
          .push_push_vlan_protocol(val) // u16
          .push_pad(val) // &[u8]
          .push_push_vlan_priority(val) // u8
          .push_push_eth_dst(val) // &[u8]
          .push_push_eth_src(val) // &[u8]
        .end_nested()
        .push_index(val) // u32
        .nested_stats()
          .push_basic(val) // PushGnetStatsBasic
          .push_rate_est(val) // PushGnetStatsRateEst
          .push_queue(val) // PushGnetStatsQueue
          .sub_nested_app_cake()
            .push_pad(val) // &[u8]
            .push_capacity_estimate64(val) // u64
            .push_memory_limit(val) // u32
            .push_memory_used(val) // u32
            .push_avg_netoff(val) // u32
            .push_min_netlen(val) // u32
            .push_max_netlen(val) // u32
            .push_min_adjlen(val) // u32
            .push_max_adjlen(val) // u32
            .array_tin_stats()
              .entry_nested()
                .push_pad(val) // &[u8]
                .push_sent_packets(val) // u32
                .push_sent_bytes64(val) // u64
                .push_dropped_packets(val) // u32
                .push_dropped_bytes64(val) // u64
                .push_acks_dropped_packets(val) // u32
                .push_acks_dropped_bytes64(val) // u64
                .push_ecn_marked_packets(val) // u32
                .push_ecn_marked_bytes64(val) // u64
                .push_backlog_packets(val) // u32
                .push_backlog_bytes(val) // u32
                .push_threshold_rate64(val) // u64
                .push_target_us(val) // u32
                .push_interval_us(val) // u32
                .push_way_indirect_hits(val) // u32
                .push_way_misses(val) // u32
                .push_way_collisions(val) // u32
                .push_peak_delay_us(val) // u32
                .push_avg_delay_us(val) // u32
                .push_base_delay_us(val) // u32
                .push_sparse_flows(val) // u32
                .push_bulk_flows(val) // u32
                .push_unresponsive_flows(val) // u32
                .push_max_skblen(val) // u32
                .push_flow_quantum(val) // u32
              .end_nested()
            .end_array()
            .push_deficit(val) // i32
            .push_cobalt_count(val) // u32
            .push_dropping(val) // u32
            .push_drop_next_us(val) // i32
            .push_p_drop(val) // u32
            .push_blue_timer_us(val) // i32
          .end_nested()
          .sub_nested_app_choke(fixed_header) // PushTcChokeXstats
          .sub_nested_app_codel(fixed_header) // PushTcCodelXstats
          .sub_nested_app_dualpi2(fixed_header) // PushTcDualpi2Xstats
          .sub_nested_app_fq(fixed_header) // PushTcFqQdStats
          .sub_nested_app_fq_codel(fixed_header) // PushTcFqCodelXstats
          .sub_nested_app_fq_pie(fixed_header) // PushTcFqPieXstats
          .sub_nested_app_hhf(fixed_header) // PushTcHhfXstats
          .sub_nested_app_pie(fixed_header) // PushTcPieXstats
          .sub_nested_app_red(fixed_header) // PushTcRedXstats
          .sub_nested_app_sfb(fixed_header) // PushTcSfbXstats
          .sub_nested_app_sfq(fixed_header) // PushTcSfqXstats
          .push_rate_est64(val) // PushGnetStatsRateEst64
          .push_pad(val) // &[u8]
          .push_basic_hw(val) // PushGnetStatsBasic
          .push_pkt64(val) // u64
        .end_nested()
        .push_pad(val) // &[u8]
        .push_cookie(val) // &[u8]
        .push_flags(val) // PushBuiltinBitfield32
        .push_hw_stats(val) // PushBuiltinBitfield32
        .push_used_hw_stats(val) // PushBuiltinBitfield32
        .push_in_hw_count(val) // u32
      .end_nested()
    .end_array()
    .push_indev(val) // &CStr
    .push_pcnt(val) // PushTcU32Pcnt
    .push_mark(val) // PushTcU32Mark
    .push_flags(val) // u32
    .push_pad(val) // &[u8]
  .end_nested()
  .push_rate(val) // PushGnetEstimator
  .push_chain(val) // u32
  .push_ingress_block(val) // u32
  .push_egress_block(val) // u32
  ;
```

### Do (reply)

```rust
let attrs = OpNewchainDoReply::new(buf);

// No attributes
```

# Operation "delchain"

## Do (request)

```rust
PushOpDelchainDoRequest::new(&mut vec)
  .push_chain(val) // u32
  ;
```

### Do (reply)

```rust
let attrs = OpDelchainDoReply::new(buf);

// No attributes
```

# Operation "getchain"

## Do (request)

```rust
PushOpGetchainDoRequest::new(&mut vec)
  .push_chain(val) // u32
  ;
```

### Do (reply)

```rust
let attrs = OpGetchainDoReply::new(buf);

attrs.get_kind(); // &CStr
attrs.get_options(); // submessage
attrs.get_stats(); // PushTcStats
attrs.get_xstats(); // submessage
attrs.get_rate(); // PushGnetEstimator
attrs.get_fcnt(); // u32
{ // Nested Stats2
  let attrs = attrs.get_stats2();
  attrs.get_basic(); // PushGnetStatsBasic
  attrs.get_rate_est(); // PushGnetStatsRateEst
  attrs.get_queue(); // PushGnetStatsQueue
  attrs.get_app(); // submessage
  attrs.get_rate_est64(); // PushGnetStatsRateEst64
  attrs.get_pad(); // &[u8]
  attrs.get_basic_hw(); // PushGnetStatsBasic
  attrs.get_pkt64(); // u64
}
{ // Nested Stab
  let attrs = attrs.get_stab();
  attrs.get_base(); // PushTcSizespec
  attrs.get_data(); // &[u8]
}
attrs.get_chain(); // u32
attrs.get_ingress_block(); // u32
attrs.get_egress_block(); // u32
```
