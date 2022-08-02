#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc RBUF0"]
    pub rbuf0: crate::Reg<rbuf0::RBUF0_SPEC>,
    #[doc = "0x04 - desc RBUF1"]
    pub rbuf1: crate::Reg<rbuf1::RBUF1_SPEC>,
    #[doc = "0x08 - desc RBUF2"]
    pub rbuf2: crate::Reg<rbuf2::RBUF2_SPEC>,
    #[doc = "0x0c - desc RBUF3"]
    pub rbuf3: crate::Reg<rbuf3::RBUF3_SPEC>,
    _reserved4: [u8; 0x40],
    #[doc = "0x50 - desc TBUF0"]
    pub tbuf0: crate::Reg<tbuf0::TBUF0_SPEC>,
    #[doc = "0x54 - desc TBUF1"]
    pub tbuf1: crate::Reg<tbuf1::TBUF1_SPEC>,
    #[doc = "0x58 - desc TBUF2"]
    pub tbuf2: crate::Reg<tbuf2::TBUF2_SPEC>,
    #[doc = "0x5c - desc TBUF3"]
    pub tbuf3: crate::Reg<tbuf3::TBUF3_SPEC>,
    _reserved8: [u8; 0x40],
    #[doc = "0xa0 - desc CFG_STAT"]
    pub cfg_stat: crate::Reg<cfg_stat::CFG_STAT_SPEC>,
    #[doc = "0xa1 - desc TCMD"]
    pub tcmd: crate::Reg<tcmd::TCMD_SPEC>,
    #[doc = "0xa2 - desc TCTRL"]
    pub tctrl: crate::Reg<tctrl::TCTRL_SPEC>,
    #[doc = "0xa3 - desc RCTRL"]
    pub rctrl: crate::Reg<rctrl::RCTRL_SPEC>,
    #[doc = "0xa4 - desc RTIE"]
    pub rtie: crate::Reg<rtie::RTIE_SPEC>,
    #[doc = "0xa5 - desc RTIF"]
    pub rtif: crate::Reg<rtif::RTIF_SPEC>,
    #[doc = "0xa6 - desc ERRINT"]
    pub errint: crate::Reg<errint::ERRINT_SPEC>,
    #[doc = "0xa7 - desc LIMIT"]
    pub limit: crate::Reg<limit::LIMIT_SPEC>,
    #[doc = "0xa8 - desc BT"]
    pub bt: crate::Reg<bt::BT_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0xb0 - desc EALCAP"]
    pub ealcap: crate::Reg<ealcap::EALCAP_SPEC>,
    _reserved18: [u8; 0x01],
    #[doc = "0xb2 - desc RECNT"]
    pub recnt: crate::Reg<recnt::RECNT_SPEC>,
    #[doc = "0xb3 - desc TECNT"]
    pub tecnt: crate::Reg<tecnt::TECNT_SPEC>,
    #[doc = "0xb4 - desc ACFCTRL"]
    pub acfctrl: crate::Reg<acfctrl::ACFCTRL_SPEC>,
    _reserved21: [u8; 0x01],
    #[doc = "0xb6 - desc ACFEN"]
    pub acfen: crate::Reg<acfen::ACFEN_SPEC>,
    _reserved22: [u8; 0x01],
    #[doc = "0xb8 - desc ACF"]
    pub acf: crate::Reg<acf::ACF_SPEC>,
    _reserved23: [u8; 0x02],
    #[doc = "0xbe - desc TBSLOT"]
    pub tbslot: crate::Reg<tbslot::TBSLOT_SPEC>,
    #[doc = "0xbf - desc TTCFG"]
    pub ttcfg: crate::Reg<ttcfg::TTCFG_SPEC>,
    #[doc = "0xc0 - desc REF_MSG"]
    pub ref_msg: crate::Reg<ref_msg::REF_MSG_SPEC>,
    #[doc = "0xc4 - desc TRG_CFG"]
    pub trg_cfg: crate::Reg<trg_cfg::TRG_CFG_SPEC>,
    #[doc = "0xc6 - desc TT_TRIG"]
    pub tt_trig: crate::Reg<tt_trig::TT_TRIG_SPEC>,
    #[doc = "0xc8 - desc TT_WTRIG"]
    pub tt_wtrig: crate::Reg<tt_wtrig::TT_WTRIG_SPEC>,
}
#[doc = "RBUF0 register accessor: an alias for `Reg<RBUF0_SPEC>`"]
pub type RBUF0 = crate::Reg<rbuf0::RBUF0_SPEC>;
#[doc = "desc RBUF0"]
pub mod rbuf0;
#[doc = "RBUF1 register accessor: an alias for `Reg<RBUF1_SPEC>`"]
pub type RBUF1 = crate::Reg<rbuf1::RBUF1_SPEC>;
#[doc = "desc RBUF1"]
pub mod rbuf1;
#[doc = "RBUF2 register accessor: an alias for `Reg<RBUF2_SPEC>`"]
pub type RBUF2 = crate::Reg<rbuf2::RBUF2_SPEC>;
#[doc = "desc RBUF2"]
pub mod rbuf2;
#[doc = "RBUF3 register accessor: an alias for `Reg<RBUF3_SPEC>`"]
pub type RBUF3 = crate::Reg<rbuf3::RBUF3_SPEC>;
#[doc = "desc RBUF3"]
pub mod rbuf3;
#[doc = "TBUF0 register accessor: an alias for `Reg<TBUF0_SPEC>`"]
pub type TBUF0 = crate::Reg<tbuf0::TBUF0_SPEC>;
#[doc = "desc TBUF0"]
pub mod tbuf0;
#[doc = "TBUF1 register accessor: an alias for `Reg<TBUF1_SPEC>`"]
pub type TBUF1 = crate::Reg<tbuf1::TBUF1_SPEC>;
#[doc = "desc TBUF1"]
pub mod tbuf1;
#[doc = "TBUF2 register accessor: an alias for `Reg<TBUF2_SPEC>`"]
pub type TBUF2 = crate::Reg<tbuf2::TBUF2_SPEC>;
#[doc = "desc TBUF2"]
pub mod tbuf2;
#[doc = "TBUF3 register accessor: an alias for `Reg<TBUF3_SPEC>`"]
pub type TBUF3 = crate::Reg<tbuf3::TBUF3_SPEC>;
#[doc = "desc TBUF3"]
pub mod tbuf3;
#[doc = "CFG_STAT register accessor: an alias for `Reg<CFG_STAT_SPEC>`"]
pub type CFG_STAT = crate::Reg<cfg_stat::CFG_STAT_SPEC>;
#[doc = "desc CFG_STAT"]
pub mod cfg_stat;
#[doc = "TCMD register accessor: an alias for `Reg<TCMD_SPEC>`"]
pub type TCMD = crate::Reg<tcmd::TCMD_SPEC>;
#[doc = "desc TCMD"]
pub mod tcmd;
#[doc = "TCTRL register accessor: an alias for `Reg<TCTRL_SPEC>`"]
pub type TCTRL = crate::Reg<tctrl::TCTRL_SPEC>;
#[doc = "desc TCTRL"]
pub mod tctrl;
#[doc = "RCTRL register accessor: an alias for `Reg<RCTRL_SPEC>`"]
pub type RCTRL = crate::Reg<rctrl::RCTRL_SPEC>;
#[doc = "desc RCTRL"]
pub mod rctrl;
#[doc = "RTIE register accessor: an alias for `Reg<RTIE_SPEC>`"]
pub type RTIE = crate::Reg<rtie::RTIE_SPEC>;
#[doc = "desc RTIE"]
pub mod rtie;
#[doc = "RTIF register accessor: an alias for `Reg<RTIF_SPEC>`"]
pub type RTIF = crate::Reg<rtif::RTIF_SPEC>;
#[doc = "desc RTIF"]
pub mod rtif;
#[doc = "ERRINT register accessor: an alias for `Reg<ERRINT_SPEC>`"]
pub type ERRINT = crate::Reg<errint::ERRINT_SPEC>;
#[doc = "desc ERRINT"]
pub mod errint;
#[doc = "LIMIT register accessor: an alias for `Reg<LIMIT_SPEC>`"]
pub type LIMIT = crate::Reg<limit::LIMIT_SPEC>;
#[doc = "desc LIMIT"]
pub mod limit;
#[doc = "BT register accessor: an alias for `Reg<BT_SPEC>`"]
pub type BT = crate::Reg<bt::BT_SPEC>;
#[doc = "desc BT"]
pub mod bt;
#[doc = "EALCAP register accessor: an alias for `Reg<EALCAP_SPEC>`"]
pub type EALCAP = crate::Reg<ealcap::EALCAP_SPEC>;
#[doc = "desc EALCAP"]
pub mod ealcap;
#[doc = "RECNT register accessor: an alias for `Reg<RECNT_SPEC>`"]
pub type RECNT = crate::Reg<recnt::RECNT_SPEC>;
#[doc = "desc RECNT"]
pub mod recnt;
#[doc = "TECNT register accessor: an alias for `Reg<TECNT_SPEC>`"]
pub type TECNT = crate::Reg<tecnt::TECNT_SPEC>;
#[doc = "desc TECNT"]
pub mod tecnt;
#[doc = "ACFCTRL register accessor: an alias for `Reg<ACFCTRL_SPEC>`"]
pub type ACFCTRL = crate::Reg<acfctrl::ACFCTRL_SPEC>;
#[doc = "desc ACFCTRL"]
pub mod acfctrl;
#[doc = "ACFEN register accessor: an alias for `Reg<ACFEN_SPEC>`"]
pub type ACFEN = crate::Reg<acfen::ACFEN_SPEC>;
#[doc = "desc ACFEN"]
pub mod acfen;
#[doc = "ACF register accessor: an alias for `Reg<ACF_SPEC>`"]
pub type ACF = crate::Reg<acf::ACF_SPEC>;
#[doc = "desc ACF"]
pub mod acf;
#[doc = "TBSLOT register accessor: an alias for `Reg<TBSLOT_SPEC>`"]
pub type TBSLOT = crate::Reg<tbslot::TBSLOT_SPEC>;
#[doc = "desc TBSLOT"]
pub mod tbslot;
#[doc = "TTCFG register accessor: an alias for `Reg<TTCFG_SPEC>`"]
pub type TTCFG = crate::Reg<ttcfg::TTCFG_SPEC>;
#[doc = "desc TTCFG"]
pub mod ttcfg;
#[doc = "REF_MSG register accessor: an alias for `Reg<REF_MSG_SPEC>`"]
pub type REF_MSG = crate::Reg<ref_msg::REF_MSG_SPEC>;
#[doc = "desc REF_MSG"]
pub mod ref_msg;
#[doc = "TRG_CFG register accessor: an alias for `Reg<TRG_CFG_SPEC>`"]
pub type TRG_CFG = crate::Reg<trg_cfg::TRG_CFG_SPEC>;
#[doc = "desc TRG_CFG"]
pub mod trg_cfg;
#[doc = "TT_TRIG register accessor: an alias for `Reg<TT_TRIG_SPEC>`"]
pub type TT_TRIG = crate::Reg<tt_trig::TT_TRIG_SPEC>;
#[doc = "desc TT_TRIG"]
pub mod tt_trig;
#[doc = "TT_WTRIG register accessor: an alias for `Reg<TT_WTRIG_SPEC>`"]
pub type TT_WTRIG = crate::Reg<tt_wtrig::TT_WTRIG_SPEC>;
#[doc = "desc TT_WTRIG"]
pub mod tt_wtrig;
