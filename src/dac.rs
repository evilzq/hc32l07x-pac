#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CR0"]
    pub cr0: crate::Reg<cr0::CR0_SPEC>,
    #[doc = "0x04 - desc SWTRIGR"]
    pub swtrigr: crate::Reg<swtrigr::SWTRIGR_SPEC>,
    #[doc = "0x08 - desc DHR12R0"]
    pub dhr12r0: crate::Reg<dhr12r0::DHR12R0_SPEC>,
    #[doc = "0x0c - desc DHR12L0"]
    pub dhr12l0: crate::Reg<dhr12l0::DHR12L0_SPEC>,
    #[doc = "0x10 - desc DHR8R0"]
    pub dhr8r0: crate::Reg<dhr8r0::DHR8R0_SPEC>,
    #[doc = "0x14 - desc DHR12R1"]
    pub dhr12r1: crate::Reg<dhr12r1::DHR12R1_SPEC>,
    #[doc = "0x18 - desc DHR12L1"]
    pub dhr12l1: crate::Reg<dhr12l1::DHR12L1_SPEC>,
    #[doc = "0x1c - desc DHR8R1"]
    pub dhr8r1: crate::Reg<dhr8r1::DHR8R1_SPEC>,
    #[doc = "0x20 - desc DHR12RD"]
    pub dhr12rd: crate::Reg<dhr12rd::DHR12RD_SPEC>,
    #[doc = "0x24 - desc DHR12LD"]
    pub dhr12ld: crate::Reg<dhr12ld::DHR12LD_SPEC>,
    #[doc = "0x28 - desc DHR8RD"]
    pub dhr8rd: crate::Reg<dhr8rd::DHR8RD_SPEC>,
    #[doc = "0x2c - desc DOR0"]
    pub dor0: crate::Reg<dor0::DOR0_SPEC>,
    #[doc = "0x30 - desc DOR1"]
    pub dor1: crate::Reg<dor1::DOR1_SPEC>,
    #[doc = "0x34 - desc SR"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x38 - desc ETRS"]
    pub etrs: crate::Reg<etrs::ETRS_SPEC>,
}
#[doc = "CR0 register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "desc CR0"]
pub mod cr0;
#[doc = "SWTRIGR register accessor: an alias for `Reg<SWTRIGR_SPEC>`"]
pub type SWTRIGR = crate::Reg<swtrigr::SWTRIGR_SPEC>;
#[doc = "desc SWTRIGR"]
pub mod swtrigr;
#[doc = "DHR12R0 register accessor: an alias for `Reg<DHR12R0_SPEC>`"]
pub type DHR12R0 = crate::Reg<dhr12r0::DHR12R0_SPEC>;
#[doc = "desc DHR12R0"]
pub mod dhr12r0;
#[doc = "DHR12L0 register accessor: an alias for `Reg<DHR12L0_SPEC>`"]
pub type DHR12L0 = crate::Reg<dhr12l0::DHR12L0_SPEC>;
#[doc = "desc DHR12L0"]
pub mod dhr12l0;
#[doc = "DHR8R0 register accessor: an alias for `Reg<DHR8R0_SPEC>`"]
pub type DHR8R0 = crate::Reg<dhr8r0::DHR8R0_SPEC>;
#[doc = "desc DHR8R0"]
pub mod dhr8r0;
#[doc = "DHR12R1 register accessor: an alias for `Reg<DHR12R1_SPEC>`"]
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1_SPEC>;
#[doc = "desc DHR12R1"]
pub mod dhr12r1;
#[doc = "DHR12L1 register accessor: an alias for `Reg<DHR12L1_SPEC>`"]
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1_SPEC>;
#[doc = "desc DHR12L1"]
pub mod dhr12l1;
#[doc = "DHR8R1 register accessor: an alias for `Reg<DHR8R1_SPEC>`"]
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1_SPEC>;
#[doc = "desc DHR8R1"]
pub mod dhr8r1;
#[doc = "DHR12RD register accessor: an alias for `Reg<DHR12RD_SPEC>`"]
pub type DHR12RD = crate::Reg<dhr12rd::DHR12RD_SPEC>;
#[doc = "desc DHR12RD"]
pub mod dhr12rd;
#[doc = "DHR12LD register accessor: an alias for `Reg<DHR12LD_SPEC>`"]
pub type DHR12LD = crate::Reg<dhr12ld::DHR12LD_SPEC>;
#[doc = "desc DHR12LD"]
pub mod dhr12ld;
#[doc = "DHR8RD register accessor: an alias for `Reg<DHR8RD_SPEC>`"]
pub type DHR8RD = crate::Reg<dhr8rd::DHR8RD_SPEC>;
#[doc = "desc DHR8RD"]
pub mod dhr8rd;
#[doc = "DOR0 register accessor: an alias for `Reg<DOR0_SPEC>`"]
pub type DOR0 = crate::Reg<dor0::DOR0_SPEC>;
#[doc = "desc DOR0"]
pub mod dor0;
#[doc = "DOR1 register accessor: an alias for `Reg<DOR1_SPEC>`"]
pub type DOR1 = crate::Reg<dor1::DOR1_SPEC>;
#[doc = "desc DOR1"]
pub mod dor1;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "desc SR"]
pub mod sr;
#[doc = "ETRS register accessor: an alias for `Reg<ETRS_SPEC>`"]
pub type ETRS = crate::Reg<etrs::ETRS_SPEC>;
#[doc = "desc ETRS"]
pub mod etrs;
