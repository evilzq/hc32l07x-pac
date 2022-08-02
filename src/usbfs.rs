#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - desc GAHBCFG"]
    pub gahbcfg: crate::Reg<gahbcfg::GAHBCFG_SPEC>,
    #[doc = "0x0c - desc GUSBCFG"]
    pub gusbcfg: crate::Reg<gusbcfg::GUSBCFG_SPEC>,
    #[doc = "0x10 - desc GRSTCTL"]
    pub grstctl: crate::Reg<grstctl::GRSTCTL_SPEC>,
    #[doc = "0x14 - desc GINTSTS"]
    pub gintsts: crate::Reg<gintsts::GINTSTS_SPEC>,
    #[doc = "0x18 - desc GINTMSK"]
    pub gintmsk: crate::Reg<gintmsk::GINTMSK_SPEC>,
    #[doc = "0x1c - desc GRXSTSR"]
    pub grxstsr: crate::Reg<grxstsr::GRXSTSR_SPEC>,
    #[doc = "0x20 - desc GRXSTSP"]
    pub grxstsp: crate::Reg<grxstsp::GRXSTSP_SPEC>,
    #[doc = "0x24 - desc GRXFSIZ"]
    pub grxfsiz: crate::Reg<grxfsiz::GRXFSIZ_SPEC>,
    _reserved8: [u8; 0x14],
    #[doc = "0x3c - desc CID"]
    pub cid: crate::Reg<cid::CID_SPEC>,
    _reserved9: [u8; 0xc4],
    #[doc = "0x104 - desc DIEPTXF1"]
    pub dieptxf1: crate::Reg<dieptxf1::DIEPTXF1_SPEC>,
    #[doc = "0x108 - desc DIEPTXF2"]
    pub dieptxf2: crate::Reg<dieptxf2::DIEPTXF2_SPEC>,
    #[doc = "0x10c - desc DIEPTXF3"]
    pub dieptxf3: crate::Reg<dieptxf3::DIEPTXF3_SPEC>,
    #[doc = "0x110 - desc DIEPTXF4"]
    pub dieptxf4: crate::Reg<dieptxf4::DIEPTXF4_SPEC>,
    _reserved13: [u8; 0x06ec],
    #[doc = "0x800 - desc DCFG"]
    pub dcfg: crate::Reg<dcfg::DCFG_SPEC>,
    #[doc = "0x804 - desc DCTL"]
    pub dctl: crate::Reg<dctl::DCTL_SPEC>,
    #[doc = "0x808 - desc DSTS"]
    pub dsts: crate::Reg<dsts::DSTS_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x810 - desc DIEPMSK"]
    pub diepmsk: crate::Reg<diepmsk::DIEPMSK_SPEC>,
    #[doc = "0x814 - desc DOEPMSK"]
    pub doepmsk: crate::Reg<doepmsk::DOEPMSK_SPEC>,
    #[doc = "0x818 - desc DAINT"]
    pub daint: crate::Reg<daint::DAINT_SPEC>,
    #[doc = "0x81c - desc DAINTMSK"]
    pub daintmsk: crate::Reg<daintmsk::DAINTMSK_SPEC>,
    _reserved20: [u8; 0x14],
    #[doc = "0x834 - desc DIEPEMPMSK"]
    pub diepempmsk: crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>,
    _reserved21: [u8; 0xc8],
    #[doc = "0x900 - desc DIEPCTL0"]
    pub diepctl0: crate::Reg<diepctl0::DIEPCTL0_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x908 - desc DIEPINT0"]
    pub diepint0: crate::Reg<diepint0::DIEPINT0_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x910 - desc DIEPTSIZ0"]
    pub dieptsiz0: crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>,
    #[doc = "0x914 - desc DIEPDMA0"]
    pub diepdma0: crate::Reg<diepdma0::DIEPDMA0_SPEC>,
    #[doc = "0x918 - desc DTXFSTS0"]
    pub dtxfsts0: crate::Reg<dtxfsts0::DTXFSTS0_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x920 - desc DIEPCTL1"]
    pub diepctl1: crate::Reg<diepctl1::DIEPCTL1_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x928 - desc DIEPINT1"]
    pub diepint1: crate::Reg<diepint1::DIEPINT1_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0x930 - desc DIEPTSIZ1"]
    pub dieptsiz1: crate::Reg<dieptsiz1::DIEPTSIZ1_SPEC>,
    #[doc = "0x934 - desc DIEPDMA1"]
    pub diepdma1: crate::Reg<diepdma1::DIEPDMA1_SPEC>,
    #[doc = "0x938 - desc DTXFSTS1"]
    pub dtxfsts1: crate::Reg<dtxfsts1::DTXFSTS1_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0x940 - desc DIEPCTL2"]
    pub diepctl2: crate::Reg<diepctl2::DIEPCTL2_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x948 - desc DIEPINT2"]
    pub diepint2: crate::Reg<diepint2::DIEPINT2_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0x950 - desc DIEPTSIZ2"]
    pub dieptsiz2: crate::Reg<dieptsiz2::DIEPTSIZ2_SPEC>,
    #[doc = "0x954 - desc DIEPDMA2"]
    pub diepdma2: crate::Reg<diepdma2::DIEPDMA2_SPEC>,
    #[doc = "0x958 - desc DTXFSTS2"]
    pub dtxfsts2: crate::Reg<dtxfsts2::DTXFSTS2_SPEC>,
    _reserved36: [u8; 0x04],
    #[doc = "0x960 - desc DIEPCTL3"]
    pub diepctl3: crate::Reg<diepctl3::DIEPCTL3_SPEC>,
    _reserved37: [u8; 0x04],
    #[doc = "0x968 - desc DIEPINT3"]
    pub diepint3: crate::Reg<diepint3::DIEPINT3_SPEC>,
    _reserved38: [u8; 0x04],
    #[doc = "0x970 - desc DIEPTSIZ3"]
    pub dieptsiz3: crate::Reg<dieptsiz3::DIEPTSIZ3_SPEC>,
    #[doc = "0x974 - desc DIEPDMA3"]
    pub diepdma3: crate::Reg<diepdma3::DIEPDMA3_SPEC>,
    #[doc = "0x978 - desc DTXFSTS3"]
    pub dtxfsts3: crate::Reg<dtxfsts3::DTXFSTS3_SPEC>,
    _reserved41: [u8; 0x04],
    #[doc = "0x980 - desc DIEPCTL4"]
    pub diepctl4: crate::Reg<diepctl4::DIEPCTL4_SPEC>,
    _reserved42: [u8; 0x04],
    #[doc = "0x988 - desc DIEPINT4"]
    pub diepint4: crate::Reg<diepint4::DIEPINT4_SPEC>,
    _reserved43: [u8; 0x04],
    #[doc = "0x990 - desc DIEPTSIZ4"]
    pub dieptsiz4: crate::Reg<dieptsiz4::DIEPTSIZ4_SPEC>,
    #[doc = "0x994 - desc DIEPDMA4"]
    pub diepdma4: crate::Reg<diepdma4::DIEPDMA4_SPEC>,
    #[doc = "0x998 - desc DTXFSTS4"]
    pub dtxfsts4: crate::Reg<dtxfsts4::DTXFSTS4_SPEC>,
    _reserved46: [u8; 0x0164],
    #[doc = "0xb00 - desc DOEPCTL0"]
    pub doepctl0: crate::Reg<doepctl0::DOEPCTL0_SPEC>,
    _reserved47: [u8; 0x04],
    #[doc = "0xb08 - desc DOEPINT0"]
    pub doepint0: crate::Reg<doepint0::DOEPINT0_SPEC>,
    _reserved48: [u8; 0x04],
    #[doc = "0xb10 - desc DOEPTSIZ0"]
    pub doeptsiz0: crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>,
    #[doc = "0xb14 - desc DOEPDMA0"]
    pub doepdma0: crate::Reg<doepdma0::DOEPDMA0_SPEC>,
    _reserved50: [u8; 0x08],
    #[doc = "0xb20 - desc DOEPCTL1"]
    pub doepctl1: crate::Reg<doepctl1::DOEPCTL1_SPEC>,
    _reserved51: [u8; 0x04],
    #[doc = "0xb28 - desc DOEPINT1"]
    pub doepint1: crate::Reg<doepint1::DOEPINT1_SPEC>,
    _reserved52: [u8; 0x04],
    #[doc = "0xb30 - desc DOEPTSIZ1"]
    pub doeptsiz1: crate::Reg<doeptsiz1::DOEPTSIZ1_SPEC>,
    #[doc = "0xb34 - desc DOEPDMA1"]
    pub doepdma1: crate::Reg<doepdma1::DOEPDMA1_SPEC>,
    _reserved54: [u8; 0x08],
    #[doc = "0xb40 - desc DOEPCTL2"]
    pub doepctl2: crate::Reg<doepctl2::DOEPCTL2_SPEC>,
    _reserved55: [u8; 0x04],
    #[doc = "0xb48 - desc DOEPINT2"]
    pub doepint2: crate::Reg<doepint2::DOEPINT2_SPEC>,
    _reserved56: [u8; 0x04],
    #[doc = "0xb50 - desc DOEPTSIZ2"]
    pub doeptsiz2: crate::Reg<doeptsiz2::DOEPTSIZ2_SPEC>,
    #[doc = "0xb54 - desc DOEPDMA2"]
    pub doepdma2: crate::Reg<doepdma2::DOEPDMA2_SPEC>,
    _reserved58: [u8; 0x08],
    #[doc = "0xb60 - desc DOEPCTL3"]
    pub doepctl3: crate::Reg<doepctl3::DOEPCTL3_SPEC>,
    _reserved59: [u8; 0x04],
    #[doc = "0xb68 - desc DOEPINT3"]
    pub doepint3: crate::Reg<doepint3::DOEPINT3_SPEC>,
    _reserved60: [u8; 0x04],
    #[doc = "0xb70 - desc DOEPTSIZ3"]
    pub doeptsiz3: crate::Reg<doeptsiz3::DOEPTSIZ3_SPEC>,
    #[doc = "0xb74 - desc DOEPDMA3"]
    pub doepdma3: crate::Reg<doepdma3::DOEPDMA3_SPEC>,
    _reserved62: [u8; 0x08],
    #[doc = "0xb80 - desc DOEPCTL4"]
    pub doepctl4: crate::Reg<doepctl4::DOEPCTL4_SPEC>,
    _reserved63: [u8; 0x04],
    #[doc = "0xb88 - desc DOEPINT4"]
    pub doepint4: crate::Reg<doepint4::DOEPINT4_SPEC>,
    _reserved64: [u8; 0x04],
    #[doc = "0xb90 - desc DOEPTSIZ4"]
    pub doeptsiz4: crate::Reg<doeptsiz4::DOEPTSIZ4_SPEC>,
    #[doc = "0xb94 - desc DOEPDMA4"]
    pub doepdma4: crate::Reg<doepdma4::DOEPDMA4_SPEC>,
    _reserved66: [u8; 0x0268],
    #[doc = "0xe00 - desc PCGCCTL"]
    pub pcgcctl: crate::Reg<pcgcctl::PCGCCTL_SPEC>,
}
#[doc = "GAHBCFG register accessor: an alias for `Reg<GAHBCFG_SPEC>`"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "desc GAHBCFG"]
pub mod gahbcfg;
#[doc = "GUSBCFG register accessor: an alias for `Reg<GUSBCFG_SPEC>`"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "desc GUSBCFG"]
pub mod gusbcfg;
#[doc = "GRSTCTL register accessor: an alias for `Reg<GRSTCTL_SPEC>`"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "desc GRSTCTL"]
pub mod grstctl;
#[doc = "GINTSTS register accessor: an alias for `Reg<GINTSTS_SPEC>`"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = "desc GINTSTS"]
pub mod gintsts;
#[doc = "GINTMSK register accessor: an alias for `Reg<GINTMSK_SPEC>`"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = "desc GINTMSK"]
pub mod gintmsk;
#[doc = "GRXSTSR register accessor: an alias for `Reg<GRXSTSR_SPEC>`"]
pub type GRXSTSR = crate::Reg<grxstsr::GRXSTSR_SPEC>;
#[doc = "desc GRXSTSR"]
pub mod grxstsr;
#[doc = "GRXSTSP register accessor: an alias for `Reg<GRXSTSP_SPEC>`"]
pub type GRXSTSP = crate::Reg<grxstsp::GRXSTSP_SPEC>;
#[doc = "desc GRXSTSP"]
pub mod grxstsp;
#[doc = "GRXFSIZ register accessor: an alias for `Reg<GRXFSIZ_SPEC>`"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "desc GRXFSIZ"]
pub mod grxfsiz;
#[doc = "CID register accessor: an alias for `Reg<CID_SPEC>`"]
pub type CID = crate::Reg<cid::CID_SPEC>;
#[doc = "desc CID"]
pub mod cid;
#[doc = "DIEPTXF1 register accessor: an alias for `Reg<DIEPTXF1_SPEC>`"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = "desc DIEPTXF1"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 register accessor: an alias for `Reg<DIEPTXF2_SPEC>`"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = "desc DIEPTXF2"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 register accessor: an alias for `Reg<DIEPTXF3_SPEC>`"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = "desc DIEPTXF3"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 register accessor: an alias for `Reg<DIEPTXF4_SPEC>`"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
#[doc = "desc DIEPTXF4"]
pub mod dieptxf4;
#[doc = "DCFG register accessor: an alias for `Reg<DCFG_SPEC>`"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "desc DCFG"]
pub mod dcfg;
#[doc = "DCTL register accessor: an alias for `Reg<DCTL_SPEC>`"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "desc DCTL"]
pub mod dctl;
#[doc = "DSTS register accessor: an alias for `Reg<DSTS_SPEC>`"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "desc DSTS"]
pub mod dsts;
#[doc = "DIEPMSK register accessor: an alias for `Reg<DIEPMSK_SPEC>`"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "desc DIEPMSK"]
pub mod diepmsk;
#[doc = "DOEPMSK register accessor: an alias for `Reg<DOEPMSK_SPEC>`"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "desc DOEPMSK"]
pub mod doepmsk;
#[doc = "DAINT register accessor: an alias for `Reg<DAINT_SPEC>`"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "desc DAINT"]
pub mod daint;
#[doc = "DAINTMSK register accessor: an alias for `Reg<DAINTMSK_SPEC>`"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "desc DAINTMSK"]
pub mod daintmsk;
#[doc = "DIEPEMPMSK register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "desc DIEPEMPMSK"]
pub mod diepempmsk;
#[doc = "DIEPCTL0 register accessor: an alias for `Reg<DIEPCTL0_SPEC>`"]
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0_SPEC>;
#[doc = "desc DIEPCTL0"]
pub mod diepctl0;
#[doc = "DIEPINT0 register accessor: an alias for `Reg<DIEPINT0_SPEC>`"]
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0_SPEC>;
#[doc = "desc DIEPINT0"]
pub mod diepint0;
#[doc = "DIEPTSIZ0 register accessor: an alias for `Reg<DIEPTSIZ0_SPEC>`"]
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>;
#[doc = "desc DIEPTSIZ0"]
pub mod dieptsiz0;
#[doc = "DIEPDMA0 register accessor: an alias for `Reg<DIEPDMA0_SPEC>`"]
pub type DIEPDMA0 = crate::Reg<diepdma0::DIEPDMA0_SPEC>;
#[doc = "desc DIEPDMA0"]
pub mod diepdma0;
#[doc = "DTXFSTS0 register accessor: an alias for `Reg<DTXFSTS0_SPEC>`"]
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0_SPEC>;
#[doc = "desc DTXFSTS0"]
pub mod dtxfsts0;
#[doc = "DIEPCTL1 register accessor: an alias for `Reg<DIEPCTL1_SPEC>`"]
pub type DIEPCTL1 = crate::Reg<diepctl1::DIEPCTL1_SPEC>;
#[doc = "desc DIEPCTL1"]
pub mod diepctl1;
#[doc = "DIEPINT1 register accessor: an alias for `Reg<DIEPINT1_SPEC>`"]
pub type DIEPINT1 = crate::Reg<diepint1::DIEPINT1_SPEC>;
#[doc = "desc DIEPINT1"]
pub mod diepint1;
#[doc = "DIEPTSIZ1 register accessor: an alias for `Reg<DIEPTSIZ1_SPEC>`"]
pub type DIEPTSIZ1 = crate::Reg<dieptsiz1::DIEPTSIZ1_SPEC>;
#[doc = "desc DIEPTSIZ1"]
pub mod dieptsiz1;
#[doc = "DIEPDMA1 register accessor: an alias for `Reg<DIEPDMA1_SPEC>`"]
pub type DIEPDMA1 = crate::Reg<diepdma1::DIEPDMA1_SPEC>;
#[doc = "desc DIEPDMA1"]
pub mod diepdma1;
#[doc = "DTXFSTS1 register accessor: an alias for `Reg<DTXFSTS1_SPEC>`"]
pub type DTXFSTS1 = crate::Reg<dtxfsts1::DTXFSTS1_SPEC>;
#[doc = "desc DTXFSTS1"]
pub mod dtxfsts1;
#[doc = "DIEPCTL2 register accessor: an alias for `Reg<DIEPCTL2_SPEC>`"]
pub type DIEPCTL2 = crate::Reg<diepctl2::DIEPCTL2_SPEC>;
#[doc = "desc DIEPCTL2"]
pub mod diepctl2;
#[doc = "DIEPINT2 register accessor: an alias for `Reg<DIEPINT2_SPEC>`"]
pub type DIEPINT2 = crate::Reg<diepint2::DIEPINT2_SPEC>;
#[doc = "desc DIEPINT2"]
pub mod diepint2;
#[doc = "DIEPTSIZ2 register accessor: an alias for `Reg<DIEPTSIZ2_SPEC>`"]
pub type DIEPTSIZ2 = crate::Reg<dieptsiz2::DIEPTSIZ2_SPEC>;
#[doc = "desc DIEPTSIZ2"]
pub mod dieptsiz2;
#[doc = "DIEPDMA2 register accessor: an alias for `Reg<DIEPDMA2_SPEC>`"]
pub type DIEPDMA2 = crate::Reg<diepdma2::DIEPDMA2_SPEC>;
#[doc = "desc DIEPDMA2"]
pub mod diepdma2;
#[doc = "DTXFSTS2 register accessor: an alias for `Reg<DTXFSTS2_SPEC>`"]
pub type DTXFSTS2 = crate::Reg<dtxfsts2::DTXFSTS2_SPEC>;
#[doc = "desc DTXFSTS2"]
pub mod dtxfsts2;
#[doc = "DIEPCTL3 register accessor: an alias for `Reg<DIEPCTL3_SPEC>`"]
pub type DIEPCTL3 = crate::Reg<diepctl3::DIEPCTL3_SPEC>;
#[doc = "desc DIEPCTL3"]
pub mod diepctl3;
#[doc = "DIEPINT3 register accessor: an alias for `Reg<DIEPINT3_SPEC>`"]
pub type DIEPINT3 = crate::Reg<diepint3::DIEPINT3_SPEC>;
#[doc = "desc DIEPINT3"]
pub mod diepint3;
#[doc = "DIEPTSIZ3 register accessor: an alias for `Reg<DIEPTSIZ3_SPEC>`"]
pub type DIEPTSIZ3 = crate::Reg<dieptsiz3::DIEPTSIZ3_SPEC>;
#[doc = "desc DIEPTSIZ3"]
pub mod dieptsiz3;
#[doc = "DIEPDMA3 register accessor: an alias for `Reg<DIEPDMA3_SPEC>`"]
pub type DIEPDMA3 = crate::Reg<diepdma3::DIEPDMA3_SPEC>;
#[doc = "desc DIEPDMA3"]
pub mod diepdma3;
#[doc = "DTXFSTS3 register accessor: an alias for `Reg<DTXFSTS3_SPEC>`"]
pub type DTXFSTS3 = crate::Reg<dtxfsts3::DTXFSTS3_SPEC>;
#[doc = "desc DTXFSTS3"]
pub mod dtxfsts3;
#[doc = "DIEPCTL4 register accessor: an alias for `Reg<DIEPCTL4_SPEC>`"]
pub type DIEPCTL4 = crate::Reg<diepctl4::DIEPCTL4_SPEC>;
#[doc = "desc DIEPCTL4"]
pub mod diepctl4;
#[doc = "DIEPINT4 register accessor: an alias for `Reg<DIEPINT4_SPEC>`"]
pub type DIEPINT4 = crate::Reg<diepint4::DIEPINT4_SPEC>;
#[doc = "desc DIEPINT4"]
pub mod diepint4;
#[doc = "DIEPTSIZ4 register accessor: an alias for `Reg<DIEPTSIZ4_SPEC>`"]
pub type DIEPTSIZ4 = crate::Reg<dieptsiz4::DIEPTSIZ4_SPEC>;
#[doc = "desc DIEPTSIZ4"]
pub mod dieptsiz4;
#[doc = "DIEPDMA4 register accessor: an alias for `Reg<DIEPDMA4_SPEC>`"]
pub type DIEPDMA4 = crate::Reg<diepdma4::DIEPDMA4_SPEC>;
#[doc = "desc DIEPDMA4"]
pub mod diepdma4;
#[doc = "DTXFSTS4 register accessor: an alias for `Reg<DTXFSTS4_SPEC>`"]
pub type DTXFSTS4 = crate::Reg<dtxfsts4::DTXFSTS4_SPEC>;
#[doc = "desc DTXFSTS4"]
pub mod dtxfsts4;
#[doc = "DOEPCTL0 register accessor: an alias for `Reg<DOEPCTL0_SPEC>`"]
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0_SPEC>;
#[doc = "desc DOEPCTL0"]
pub mod doepctl0;
#[doc = "DOEPINT0 register accessor: an alias for `Reg<DOEPINT0_SPEC>`"]
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0_SPEC>;
#[doc = "desc DOEPINT0"]
pub mod doepint0;
#[doc = "DOEPTSIZ0 register accessor: an alias for `Reg<DOEPTSIZ0_SPEC>`"]
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>;
#[doc = "desc DOEPTSIZ0"]
pub mod doeptsiz0;
#[doc = "DOEPDMA0 register accessor: an alias for `Reg<DOEPDMA0_SPEC>`"]
pub type DOEPDMA0 = crate::Reg<doepdma0::DOEPDMA0_SPEC>;
#[doc = "desc DOEPDMA0"]
pub mod doepdma0;
#[doc = "DOEPCTL1 register accessor: an alias for `Reg<DOEPCTL1_SPEC>`"]
pub type DOEPCTL1 = crate::Reg<doepctl1::DOEPCTL1_SPEC>;
#[doc = "desc DOEPCTL1"]
pub mod doepctl1;
#[doc = "DOEPINT1 register accessor: an alias for `Reg<DOEPINT1_SPEC>`"]
pub type DOEPINT1 = crate::Reg<doepint1::DOEPINT1_SPEC>;
#[doc = "desc DOEPINT1"]
pub mod doepint1;
#[doc = "DOEPTSIZ1 register accessor: an alias for `Reg<DOEPTSIZ1_SPEC>`"]
pub type DOEPTSIZ1 = crate::Reg<doeptsiz1::DOEPTSIZ1_SPEC>;
#[doc = "desc DOEPTSIZ1"]
pub mod doeptsiz1;
#[doc = "DOEPDMA1 register accessor: an alias for `Reg<DOEPDMA1_SPEC>`"]
pub type DOEPDMA1 = crate::Reg<doepdma1::DOEPDMA1_SPEC>;
#[doc = "desc DOEPDMA1"]
pub mod doepdma1;
#[doc = "DOEPCTL2 register accessor: an alias for `Reg<DOEPCTL2_SPEC>`"]
pub type DOEPCTL2 = crate::Reg<doepctl2::DOEPCTL2_SPEC>;
#[doc = "desc DOEPCTL2"]
pub mod doepctl2;
#[doc = "DOEPINT2 register accessor: an alias for `Reg<DOEPINT2_SPEC>`"]
pub type DOEPINT2 = crate::Reg<doepint2::DOEPINT2_SPEC>;
#[doc = "desc DOEPINT2"]
pub mod doepint2;
#[doc = "DOEPTSIZ2 register accessor: an alias for `Reg<DOEPTSIZ2_SPEC>`"]
pub type DOEPTSIZ2 = crate::Reg<doeptsiz2::DOEPTSIZ2_SPEC>;
#[doc = "desc DOEPTSIZ2"]
pub mod doeptsiz2;
#[doc = "DOEPDMA2 register accessor: an alias for `Reg<DOEPDMA2_SPEC>`"]
pub type DOEPDMA2 = crate::Reg<doepdma2::DOEPDMA2_SPEC>;
#[doc = "desc DOEPDMA2"]
pub mod doepdma2;
#[doc = "DOEPCTL3 register accessor: an alias for `Reg<DOEPCTL3_SPEC>`"]
pub type DOEPCTL3 = crate::Reg<doepctl3::DOEPCTL3_SPEC>;
#[doc = "desc DOEPCTL3"]
pub mod doepctl3;
#[doc = "DOEPINT3 register accessor: an alias for `Reg<DOEPINT3_SPEC>`"]
pub type DOEPINT3 = crate::Reg<doepint3::DOEPINT3_SPEC>;
#[doc = "desc DOEPINT3"]
pub mod doepint3;
#[doc = "DOEPTSIZ3 register accessor: an alias for `Reg<DOEPTSIZ3_SPEC>`"]
pub type DOEPTSIZ3 = crate::Reg<doeptsiz3::DOEPTSIZ3_SPEC>;
#[doc = "desc DOEPTSIZ3"]
pub mod doeptsiz3;
#[doc = "DOEPDMA3 register accessor: an alias for `Reg<DOEPDMA3_SPEC>`"]
pub type DOEPDMA3 = crate::Reg<doepdma3::DOEPDMA3_SPEC>;
#[doc = "desc DOEPDMA3"]
pub mod doepdma3;
#[doc = "DOEPCTL4 register accessor: an alias for `Reg<DOEPCTL4_SPEC>`"]
pub type DOEPCTL4 = crate::Reg<doepctl4::DOEPCTL4_SPEC>;
#[doc = "desc DOEPCTL4"]
pub mod doepctl4;
#[doc = "DOEPINT4 register accessor: an alias for `Reg<DOEPINT4_SPEC>`"]
pub type DOEPINT4 = crate::Reg<doepint4::DOEPINT4_SPEC>;
#[doc = "desc DOEPINT4"]
pub mod doepint4;
#[doc = "DOEPTSIZ4 register accessor: an alias for `Reg<DOEPTSIZ4_SPEC>`"]
pub type DOEPTSIZ4 = crate::Reg<doeptsiz4::DOEPTSIZ4_SPEC>;
#[doc = "desc DOEPTSIZ4"]
pub mod doeptsiz4;
#[doc = "DOEPDMA4 register accessor: an alias for `Reg<DOEPDMA4_SPEC>`"]
pub type DOEPDMA4 = crate::Reg<doepdma4::DOEPDMA4_SPEC>;
#[doc = "desc DOEPDMA4"]
pub mod doepdma4;
#[doc = "PCGCCTL register accessor: an alias for `Reg<PCGCCTL_SPEC>`"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "desc PCGCCTL"]
pub mod pcgcctl;
