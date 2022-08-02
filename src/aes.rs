#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc CR"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - desc DATA0"]
    pub data0: crate::Reg<data0::DATA0_SPEC>,
    #[doc = "0x14 - desc DATA1"]
    pub data1: crate::Reg<data1::DATA1_SPEC>,
    #[doc = "0x18 - desc DATA2"]
    pub data2: crate::Reg<data2::DATA2_SPEC>,
    #[doc = "0x1c - desc DATA3"]
    pub data3: crate::Reg<data3::DATA3_SPEC>,
    #[doc = "0x20 - desc KEY0"]
    pub key0: crate::Reg<key0::KEY0_SPEC>,
    #[doc = "0x24 - desc KEY1"]
    pub key1: crate::Reg<key1::KEY1_SPEC>,
    #[doc = "0x28 - desc KEY2"]
    pub key2: crate::Reg<key2::KEY2_SPEC>,
    #[doc = "0x2c - desc KEY3"]
    pub key3: crate::Reg<key3::KEY3_SPEC>,
    #[doc = "0x30 - desc KEY4"]
    pub key4: crate::Reg<key4::KEY4_SPEC>,
    #[doc = "0x34 - desc KEY5"]
    pub key5: crate::Reg<key5::KEY5_SPEC>,
    #[doc = "0x38 - desc KEY6"]
    pub key6: crate::Reg<key6::KEY6_SPEC>,
    #[doc = "0x3c - desc KEY7"]
    pub key7: crate::Reg<key7::KEY7_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "desc CR"]
pub mod cr;
#[doc = "DATA0 register accessor: an alias for `Reg<DATA0_SPEC>`"]
pub type DATA0 = crate::Reg<data0::DATA0_SPEC>;
#[doc = "desc DATA0"]
pub mod data0;
#[doc = "DATA1 register accessor: an alias for `Reg<DATA1_SPEC>`"]
pub type DATA1 = crate::Reg<data1::DATA1_SPEC>;
#[doc = "desc DATA1"]
pub mod data1;
#[doc = "DATA2 register accessor: an alias for `Reg<DATA2_SPEC>`"]
pub type DATA2 = crate::Reg<data2::DATA2_SPEC>;
#[doc = "desc DATA2"]
pub mod data2;
#[doc = "DATA3 register accessor: an alias for `Reg<DATA3_SPEC>`"]
pub type DATA3 = crate::Reg<data3::DATA3_SPEC>;
#[doc = "desc DATA3"]
pub mod data3;
#[doc = "KEY0 register accessor: an alias for `Reg<KEY0_SPEC>`"]
pub type KEY0 = crate::Reg<key0::KEY0_SPEC>;
#[doc = "desc KEY0"]
pub mod key0;
#[doc = "KEY1 register accessor: an alias for `Reg<KEY1_SPEC>`"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "desc KEY1"]
pub mod key1;
#[doc = "KEY2 register accessor: an alias for `Reg<KEY2_SPEC>`"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "desc KEY2"]
pub mod key2;
#[doc = "KEY3 register accessor: an alias for `Reg<KEY3_SPEC>`"]
pub type KEY3 = crate::Reg<key3::KEY3_SPEC>;
#[doc = "desc KEY3"]
pub mod key3;
#[doc = "KEY4 register accessor: an alias for `Reg<KEY4_SPEC>`"]
pub type KEY4 = crate::Reg<key4::KEY4_SPEC>;
#[doc = "desc KEY4"]
pub mod key4;
#[doc = "KEY5 register accessor: an alias for `Reg<KEY5_SPEC>`"]
pub type KEY5 = crate::Reg<key5::KEY5_SPEC>;
#[doc = "desc KEY5"]
pub mod key5;
#[doc = "KEY6 register accessor: an alias for `Reg<KEY6_SPEC>`"]
pub type KEY6 = crate::Reg<key6::KEY6_SPEC>;
#[doc = "desc KEY6"]
pub mod key6;
#[doc = "KEY7 register accessor: an alias for `Reg<KEY7_SPEC>`"]
pub type KEY7 = crate::Reg<key7::KEY7_SPEC>;
#[doc = "desc KEY7"]
pub mod key7;
