#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - desc CR"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x08 - desc SR"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    _reserved2: [u8; 0x10],
    #[doc = "0x1c - desc CFGR"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    #[doc = "0x20 - desc PR"]
    pub pr: crate::Reg<pr::PR_SPEC>,
    #[doc = "0x24 - desc ICR"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x28 - desc DRL"]
    pub drl: crate::Reg<drl::DRL_SPEC>,
    #[doc = "0x2c - desc DRR"]
    pub drr: crate::Reg<drr::DRR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "desc SR"]
pub mod sr;
#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "desc CFGR"]
pub mod cfgr;
#[doc = "PR register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "desc PR"]
pub mod pr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "desc ICR"]
pub mod icr;
#[doc = "DRL register accessor: an alias for `Reg<DRL_SPEC>`"]
pub type DRL = crate::Reg<drl::DRL_SPEC>;
#[doc = "desc DRL"]
pub mod drl;
#[doc = "DRR register accessor: an alias for `Reg<DRR_SPEC>`"]
pub type DRR = crate::Reg<drr::DRR_SPEC>;
#[doc = "desc DRR"]
pub mod drr;
