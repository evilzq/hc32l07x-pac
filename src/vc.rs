#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - desc CR"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x14 - desc VC0_CR"]
    pub vc0_cr: crate::Reg<vc0_cr::VC0_CR_SPEC>,
    #[doc = "0x18 - desc VC1_CR"]
    pub vc1_cr: crate::Reg<vc1_cr::VC1_CR_SPEC>,
    #[doc = "0x1c - desc VC0_OUT_CFG"]
    pub vc0_out_cfg: crate::Reg<vc0_out_cfg::VC0_OUT_CFG_SPEC>,
    #[doc = "0x20 - desc VC1_OUT_CFG"]
    pub vc1_out_cfg: crate::Reg<vc1_out_cfg::VC1_OUT_CFG_SPEC>,
    #[doc = "0x24 - desc IFR"]
    pub ifr: crate::Reg<ifr::IFR_SPEC>,
    _reserved6: [u8; 0x0128],
    #[doc = "0x150 - desc VC2_CR"]
    pub vc2_cr: crate::Reg<vc2_cr::VC2_CR_SPEC>,
    #[doc = "0x154 - desc VC2_OUT_CFG"]
    pub vc2_out_cfg: crate::Reg<vc2_out_cfg::VC2_OUT_CFG_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "VC0_CR register accessor: an alias for `Reg<VC0_CR_SPEC>`"]
pub type VC0_CR = crate::Reg<vc0_cr::VC0_CR_SPEC>;
#[doc = "desc VC0_CR"]
pub mod vc0_cr;
#[doc = "VC1_CR register accessor: an alias for `Reg<VC1_CR_SPEC>`"]
pub type VC1_CR = crate::Reg<vc1_cr::VC1_CR_SPEC>;
#[doc = "desc VC1_CR"]
pub mod vc1_cr;
#[doc = "VC0_OUT_CFG register accessor: an alias for `Reg<VC0_OUT_CFG_SPEC>`"]
pub type VC0_OUT_CFG = crate::Reg<vc0_out_cfg::VC0_OUT_CFG_SPEC>;
#[doc = "desc VC0_OUT_CFG"]
pub mod vc0_out_cfg;
#[doc = "VC1_OUT_CFG register accessor: an alias for `Reg<VC1_OUT_CFG_SPEC>`"]
pub type VC1_OUT_CFG = crate::Reg<vc1_out_cfg::VC1_OUT_CFG_SPEC>;
#[doc = "desc VC1_OUT_CFG"]
pub mod vc1_out_cfg;
#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "desc IFR"]
pub mod ifr;
#[doc = "VC2_CR register accessor: an alias for `Reg<VC2_CR_SPEC>`"]
pub type VC2_CR = crate::Reg<vc2_cr::VC2_CR_SPEC>;
#[doc = "desc VC2_CR"]
pub mod vc2_cr;
#[doc = "VC2_OUT_CFG register accessor: an alias for `Reg<VC2_OUT_CFG_SPEC>`"]
pub type VC2_OUT_CFG = crate::Reg<vc2_out_cfg::VC2_OUT_CFG_SPEC>;
#[doc = "desc VC2_OUT_CFG"]
pub mod vc2_out_cfg;
