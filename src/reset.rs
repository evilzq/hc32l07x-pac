#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc RESET_FLAG"]
    pub reset_flag: crate::Reg<reset_flag::RESET_FLAG_SPEC>,
    _reserved1: [u8; 0x08],
    #[doc = "0x0c - desc PERI_RESET0"]
    pub peri_reset0: crate::Reg<peri_reset0::PERI_RESET0_SPEC>,
    #[doc = "0x10 - desc PERI_RESET1"]
    pub peri_reset1: crate::Reg<peri_reset1::PERI_RESET1_SPEC>,
}
#[doc = "RESET_FLAG register accessor: an alias for `Reg<RESET_FLAG_SPEC>`"]
pub type RESET_FLAG = crate::Reg<reset_flag::RESET_FLAG_SPEC>;
#[doc = "desc RESET_FLAG"]
pub mod reset_flag;
#[doc = "PERI_RESET0 register accessor: an alias for `Reg<PERI_RESET0_SPEC>`"]
pub type PERI_RESET0 = crate::Reg<peri_reset0::PERI_RESET0_SPEC>;
#[doc = "desc PERI_RESET0"]
pub mod peri_reset0;
#[doc = "PERI_RESET1 register accessor: an alias for `Reg<PERI_RESET1_SPEC>`"]
pub type PERI_RESET1 = crate::Reg<peri_reset1::PERI_RESET1_SPEC>;
#[doc = "desc PERI_RESET1"]
pub mod peri_reset1;
