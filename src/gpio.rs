#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - desc PA00_SEL"]
    pub pa00_sel: crate::Reg<pa00_sel::PA00_SEL_SPEC>,
    #[doc = "0x04 - desc PA01_SEL"]
    pub pa01_sel: crate::Reg<pa01_sel::PA01_SEL_SPEC>,
    #[doc = "0x08 - desc PA02_SEL"]
    pub pa02_sel: crate::Reg<pa02_sel::PA02_SEL_SPEC>,
    #[doc = "0x0c - desc PA03_SEL"]
    pub pa03_sel: crate::Reg<pa03_sel::PA03_SEL_SPEC>,
    #[doc = "0x10 - desc PA04_SEL"]
    pub pa04_sel: crate::Reg<pa04_sel::PA04_SEL_SPEC>,
    #[doc = "0x14 - desc PA05_SEL"]
    pub pa05_sel: crate::Reg<pa05_sel::PA05_SEL_SPEC>,
    #[doc = "0x18 - desc PA06_SEL"]
    pub pa06_sel: crate::Reg<pa06_sel::PA06_SEL_SPEC>,
    #[doc = "0x1c - desc PA07_SEL"]
    pub pa07_sel: crate::Reg<pa07_sel::PA07_SEL_SPEC>,
    #[doc = "0x20 - desc PA08_SEL"]
    pub pa08_sel: crate::Reg<pa08_sel::PA08_SEL_SPEC>,
    #[doc = "0x24 - desc PA09_SEL"]
    pub pa09_sel: crate::Reg<pa09_sel::PA09_SEL_SPEC>,
    #[doc = "0x28 - desc PA10_SEL"]
    pub pa10_sel: crate::Reg<pa10_sel::PA10_SEL_SPEC>,
    #[doc = "0x2c - desc PA11_SEL"]
    pub pa11_sel: crate::Reg<pa11_sel::PA11_SEL_SPEC>,
    #[doc = "0x30 - desc PA12_SEL"]
    pub pa12_sel: crate::Reg<pa12_sel::PA12_SEL_SPEC>,
    #[doc = "0x34 - desc PA13_SEL"]
    pub pa13_sel: crate::Reg<pa13_sel::PA13_SEL_SPEC>,
    #[doc = "0x38 - desc PA14_SEL"]
    pub pa14_sel: crate::Reg<pa14_sel::PA14_SEL_SPEC>,
    #[doc = "0x3c - desc PA15_SEL"]
    pub pa15_sel: crate::Reg<pa15_sel::PA15_SEL_SPEC>,
    #[doc = "0x40 - desc PB00_SEL"]
    pub pb00_sel: crate::Reg<pb00_sel::PB00_SEL_SPEC>,
    #[doc = "0x44 - desc PB01_SEL"]
    pub pb01_sel: crate::Reg<pb01_sel::PB01_SEL_SPEC>,
    #[doc = "0x48 - desc PB02_SEL"]
    pub pb02_sel: crate::Reg<pb02_sel::PB02_SEL_SPEC>,
    #[doc = "0x4c - desc PB03_SEL"]
    pub pb03_sel: crate::Reg<pb03_sel::PB03_SEL_SPEC>,
    #[doc = "0x50 - desc PB04_SEL"]
    pub pb04_sel: crate::Reg<pb04_sel::PB04_SEL_SPEC>,
    #[doc = "0x54 - desc PB05_SEL"]
    pub pb05_sel: crate::Reg<pb05_sel::PB05_SEL_SPEC>,
    #[doc = "0x58 - desc PB06_SEL"]
    pub pb06_sel: crate::Reg<pb06_sel::PB06_SEL_SPEC>,
    #[doc = "0x5c - desc PB07_SEL"]
    pub pb07_sel: crate::Reg<pb07_sel::PB07_SEL_SPEC>,
    #[doc = "0x60 - desc PB08_SEL"]
    pub pb08_sel: crate::Reg<pb08_sel::PB08_SEL_SPEC>,
    #[doc = "0x64 - desc PB09_SEL"]
    pub pb09_sel: crate::Reg<pb09_sel::PB09_SEL_SPEC>,
    #[doc = "0x68 - desc PB10_SEL"]
    pub pb10_sel: crate::Reg<pb10_sel::PB10_SEL_SPEC>,
    #[doc = "0x6c - desc PB11_SEL"]
    pub pb11_sel: crate::Reg<pb11_sel::PB11_SEL_SPEC>,
    #[doc = "0x70 - desc PB12_SEL"]
    pub pb12_sel: crate::Reg<pb12_sel::PB12_SEL_SPEC>,
    #[doc = "0x74 - desc PB13_SEL"]
    pub pb13_sel: crate::Reg<pb13_sel::PB13_SEL_SPEC>,
    #[doc = "0x78 - desc PB14_SEL"]
    pub pb14_sel: crate::Reg<pb14_sel::PB14_SEL_SPEC>,
    #[doc = "0x7c - desc PB15_SEL"]
    pub pb15_sel: crate::Reg<pb15_sel::PB15_SEL_SPEC>,
    #[doc = "0x80 - desc PC00_SEL"]
    pub pc00_sel: crate::Reg<pc00_sel::PC00_SEL_SPEC>,
    #[doc = "0x84 - desc PC01_SEL"]
    pub pc01_sel: crate::Reg<pc01_sel::PC01_SEL_SPEC>,
    #[doc = "0x88 - desc PC02_SEL"]
    pub pc02_sel: crate::Reg<pc02_sel::PC02_SEL_SPEC>,
    #[doc = "0x8c - desc PC03_SEL"]
    pub pc03_sel: crate::Reg<pc03_sel::PC03_SEL_SPEC>,
    #[doc = "0x90 - desc PC04_SEL"]
    pub pc04_sel: crate::Reg<pc04_sel::PC04_SEL_SPEC>,
    #[doc = "0x94 - desc PC05_SEL"]
    pub pc05_sel: crate::Reg<pc05_sel::PC05_SEL_SPEC>,
    #[doc = "0x98 - desc PC06_SEL"]
    pub pc06_sel: crate::Reg<pc06_sel::PC06_SEL_SPEC>,
    #[doc = "0x9c - desc PC07_SEL"]
    pub pc07_sel: crate::Reg<pc07_sel::PC07_SEL_SPEC>,
    #[doc = "0xa0 - desc PC08_SEL"]
    pub pc08_sel: crate::Reg<pc08_sel::PC08_SEL_SPEC>,
    #[doc = "0xa4 - desc PC09_SEL"]
    pub pc09_sel: crate::Reg<pc09_sel::PC09_SEL_SPEC>,
    #[doc = "0xa8 - desc PC10_SEL"]
    pub pc10_sel: crate::Reg<pc10_sel::PC10_SEL_SPEC>,
    #[doc = "0xac - desc PC11_SEL"]
    pub pc11_sel: crate::Reg<pc11_sel::PC11_SEL_SPEC>,
    #[doc = "0xb0 - desc PC12_SEL"]
    pub pc12_sel: crate::Reg<pc12_sel::PC12_SEL_SPEC>,
    #[doc = "0xb4 - desc PC13_SEL"]
    pub pc13_sel: crate::Reg<pc13_sel::PC13_SEL_SPEC>,
    #[doc = "0xb8 - desc PC14_SEL"]
    pub pc14_sel: crate::Reg<pc14_sel::PC14_SEL_SPEC>,
    #[doc = "0xbc - desc PC15_SEL"]
    pub pc15_sel: crate::Reg<pc15_sel::PC15_SEL_SPEC>,
    #[doc = "0xc0 - desc PD00_SEL"]
    pub pd00_sel: crate::Reg<pd00_sel::PD00_SEL_SPEC>,
    #[doc = "0xc4 - desc PD01_SEL"]
    pub pd01_sel: crate::Reg<pd01_sel::PD01_SEL_SPEC>,
    #[doc = "0xc8 - desc PD02_SEL"]
    pub pd02_sel: crate::Reg<pd02_sel::PD02_SEL_SPEC>,
    #[doc = "0xcc - desc PD03_SEL"]
    pub pd03_sel: crate::Reg<pd03_sel::PD03_SEL_SPEC>,
    #[doc = "0xd0 - desc PD04_SEL"]
    pub pd04_sel: crate::Reg<pd04_sel::PD04_SEL_SPEC>,
    #[doc = "0xd4 - desc PD05_SEL"]
    pub pd05_sel: crate::Reg<pd05_sel::PD05_SEL_SPEC>,
    #[doc = "0xd8 - desc PD06_SEL"]
    pub pd06_sel: crate::Reg<pd06_sel::PD06_SEL_SPEC>,
    #[doc = "0xdc - desc PD07_SEL"]
    pub pd07_sel: crate::Reg<pd07_sel::PD07_SEL_SPEC>,
    #[doc = "0xe0 - desc PD08_SEL"]
    pub pd08_sel: crate::Reg<pd08_sel::PD08_SEL_SPEC>,
    #[doc = "0xe4 - desc PD09_SEL"]
    pub pd09_sel: crate::Reg<pd09_sel::PD09_SEL_SPEC>,
    #[doc = "0xe8 - desc PD10_SEL"]
    pub pd10_sel: crate::Reg<pd10_sel::PD10_SEL_SPEC>,
    #[doc = "0xec - desc PD11_SEL"]
    pub pd11_sel: crate::Reg<pd11_sel::PD11_SEL_SPEC>,
    #[doc = "0xf0 - desc PD12_SEL"]
    pub pd12_sel: crate::Reg<pd12_sel::PD12_SEL_SPEC>,
    #[doc = "0xf4 - desc PD13_SEL"]
    pub pd13_sel: crate::Reg<pd13_sel::PD13_SEL_SPEC>,
    #[doc = "0xf8 - desc PD14_SEL"]
    pub pd14_sel: crate::Reg<pd14_sel::PD14_SEL_SPEC>,
    #[doc = "0xfc - desc PD15_SEL"]
    pub pd15_sel: crate::Reg<pd15_sel::PD15_SEL_SPEC>,
    #[doc = "0x100 - desc PADIR"]
    pub padir: crate::Reg<padir::PADIR_SPEC>,
    #[doc = "0x104 - desc PAIN"]
    pub pain: crate::Reg<pain::PAIN_SPEC>,
    #[doc = "0x108 - desc PAOUT"]
    pub paout: crate::Reg<paout::PAOUT_SPEC>,
    #[doc = "0x10c - desc PAADS"]
    pub paads: crate::Reg<paads::PAADS_SPEC>,
    #[doc = "0x110 - desc PABSET"]
    pub pabset: crate::Reg<pabset::PABSET_SPEC>,
    #[doc = "0x114 - desc PABCLR"]
    pub pabclr: crate::Reg<pabclr::PABCLR_SPEC>,
    #[doc = "0x118 - desc PABSETCLR"]
    pub pabsetclr: crate::Reg<pabsetclr::PABSETCLR_SPEC>,
    #[doc = "0x11c - desc PADR"]
    pub padr: crate::Reg<padr::PADR_SPEC>,
    #[doc = "0x120 - desc PAPU"]
    pub papu: crate::Reg<papu::PAPU_SPEC>,
    #[doc = "0x124 - desc PAPD"]
    pub papd: crate::Reg<papd::PAPD_SPEC>,
    _reserved74: [u8; 0x04],
    #[doc = "0x12c - desc PAOD"]
    pub paod: crate::Reg<paod::PAOD_SPEC>,
    #[doc = "0x130 - desc PAHIE"]
    pub pahie: crate::Reg<pahie::PAHIE_SPEC>,
    #[doc = "0x134 - desc PALIE"]
    pub palie: crate::Reg<palie::PALIE_SPEC>,
    #[doc = "0x138 - desc PARIE"]
    pub parie: crate::Reg<parie::PARIE_SPEC>,
    #[doc = "0x13c - desc PAFIE"]
    pub pafie: crate::Reg<pafie::PAFIE_SPEC>,
    #[doc = "0x140 - desc PBDIR"]
    pub pbdir: crate::Reg<pbdir::PBDIR_SPEC>,
    #[doc = "0x144 - desc PBIN"]
    pub pbin: crate::Reg<pbin::PBIN_SPEC>,
    #[doc = "0x148 - desc PBOUT"]
    pub pbout: crate::Reg<pbout::PBOUT_SPEC>,
    #[doc = "0x14c - desc PBADS"]
    pub pbads: crate::Reg<pbads::PBADS_SPEC>,
    #[doc = "0x150 - desc PBBSET"]
    pub pbbset: crate::Reg<pbbset::PBBSET_SPEC>,
    #[doc = "0x154 - desc PBBCLR"]
    pub pbbclr: crate::Reg<pbbclr::PBBCLR_SPEC>,
    #[doc = "0x158 - desc PBBSETCLR"]
    pub pbbsetclr: crate::Reg<pbbsetclr::PBBSETCLR_SPEC>,
    #[doc = "0x15c - desc PBDR"]
    pub pbdr: crate::Reg<pbdr::PBDR_SPEC>,
    #[doc = "0x160 - desc PBPU"]
    pub pbpu: crate::Reg<pbpu::PBPU_SPEC>,
    #[doc = "0x164 - desc PBPD"]
    pub pbpd: crate::Reg<pbpd::PBPD_SPEC>,
    _reserved89: [u8; 0x04],
    #[doc = "0x16c - desc PBOD"]
    pub pbod: crate::Reg<pbod::PBOD_SPEC>,
    #[doc = "0x170 - desc PBHIE"]
    pub pbhie: crate::Reg<pbhie::PBHIE_SPEC>,
    #[doc = "0x174 - desc PBLIE"]
    pub pblie: crate::Reg<pblie::PBLIE_SPEC>,
    #[doc = "0x178 - desc PBRIE"]
    pub pbrie: crate::Reg<pbrie::PBRIE_SPEC>,
    #[doc = "0x17c - desc PBFIE"]
    pub pbfie: crate::Reg<pbfie::PBFIE_SPEC>,
    #[doc = "0x180 - desc PCDIR"]
    pub pcdir: crate::Reg<pcdir::PCDIR_SPEC>,
    #[doc = "0x184 - desc PCIN"]
    pub pcin: crate::Reg<pcin::PCIN_SPEC>,
    #[doc = "0x188 - desc PCOUT"]
    pub pcout: crate::Reg<pcout::PCOUT_SPEC>,
    #[doc = "0x18c - desc PCADS"]
    pub pcads: crate::Reg<pcads::PCADS_SPEC>,
    #[doc = "0x190 - desc PCBSET"]
    pub pcbset: crate::Reg<pcbset::PCBSET_SPEC>,
    #[doc = "0x194 - desc PCBCLR"]
    pub pcbclr: crate::Reg<pcbclr::PCBCLR_SPEC>,
    #[doc = "0x198 - desc PCBSETCLR"]
    pub pcbsetclr: crate::Reg<pcbsetclr::PCBSETCLR_SPEC>,
    #[doc = "0x19c - desc PCDR"]
    pub pcdr: crate::Reg<pcdr::PCDR_SPEC>,
    #[doc = "0x1a0 - desc PCPU"]
    pub pcpu: crate::Reg<pcpu::PCPU_SPEC>,
    #[doc = "0x1a4 - desc PCPD"]
    pub pcpd: crate::Reg<pcpd::PCPD_SPEC>,
    _reserved104: [u8; 0x04],
    #[doc = "0x1ac - desc PCOD"]
    pub pcod: crate::Reg<pcod::PCOD_SPEC>,
    #[doc = "0x1b0 - desc PCHIE"]
    pub pchie: crate::Reg<pchie::PCHIE_SPEC>,
    #[doc = "0x1b4 - desc PCLIE"]
    pub pclie: crate::Reg<pclie::PCLIE_SPEC>,
    #[doc = "0x1b8 - desc PCRIE"]
    pub pcrie: crate::Reg<pcrie::PCRIE_SPEC>,
    #[doc = "0x1bc - desc PCFIE"]
    pub pcfie: crate::Reg<pcfie::PCFIE_SPEC>,
    #[doc = "0x1c0 - desc PDDIR"]
    pub pddir: crate::Reg<pddir::PDDIR_SPEC>,
    #[doc = "0x1c4 - desc PDIN"]
    pub pdin: crate::Reg<pdin::PDIN_SPEC>,
    #[doc = "0x1c8 - desc PDOUT"]
    pub pdout: crate::Reg<pdout::PDOUT_SPEC>,
    #[doc = "0x1cc - desc PDADS"]
    pub pdads: crate::Reg<pdads::PDADS_SPEC>,
    #[doc = "0x1d0 - desc PDBSET"]
    pub pdbset: crate::Reg<pdbset::PDBSET_SPEC>,
    #[doc = "0x1d4 - desc PDBCLR"]
    pub pdbclr: crate::Reg<pdbclr::PDBCLR_SPEC>,
    #[doc = "0x1d8 - desc PDBSETCLR"]
    pub pdbsetclr: crate::Reg<pdbsetclr::PDBSETCLR_SPEC>,
    #[doc = "0x1dc - desc PDDR"]
    pub pddr: crate::Reg<pddr::PDDR_SPEC>,
    #[doc = "0x1e0 - desc PDPU"]
    pub pdpu: crate::Reg<pdpu::PDPU_SPEC>,
    #[doc = "0x1e4 - desc PDPD"]
    pub pdpd: crate::Reg<pdpd::PDPD_SPEC>,
    _reserved119: [u8; 0x04],
    #[doc = "0x1ec - desc PDOD"]
    pub pdod: crate::Reg<pdod::PDOD_SPEC>,
    #[doc = "0x1f0 - desc PDHIE"]
    pub pdhie: crate::Reg<pdhie::PDHIE_SPEC>,
    #[doc = "0x1f4 - desc PDLIE"]
    pub pdlie: crate::Reg<pdlie::PDLIE_SPEC>,
    #[doc = "0x1f8 - desc PDRIE"]
    pub pdrie: crate::Reg<pdrie::PDRIE_SPEC>,
    #[doc = "0x1fc - desc PDFIE"]
    pub pdfie: crate::Reg<pdfie::PDFIE_SPEC>,
    #[doc = "0x200 - desc PA_STAT"]
    pub pa_stat: crate::Reg<pa_stat::PA_STAT_SPEC>,
    _reserved125: [u8; 0x0c],
    #[doc = "0x210 - desc PA_ICLR"]
    pub pa_iclr: crate::Reg<pa_iclr::PA_ICLR_SPEC>,
    _reserved126: [u8; 0x2c],
    #[doc = "0x240 - desc PB_STAT"]
    pub pb_stat: crate::Reg<pb_stat::PB_STAT_SPEC>,
    _reserved127: [u8; 0x0c],
    #[doc = "0x250 - desc PB_ICLR"]
    pub pb_iclr: crate::Reg<pb_iclr::PB_ICLR_SPEC>,
    _reserved128: [u8; 0x2c],
    #[doc = "0x280 - desc PC_STAT"]
    pub pc_stat: crate::Reg<pc_stat::PC_STAT_SPEC>,
    _reserved129: [u8; 0x0c],
    #[doc = "0x290 - desc PC_ICLR"]
    pub pc_iclr: crate::Reg<pc_iclr::PC_ICLR_SPEC>,
    _reserved130: [u8; 0x2c],
    #[doc = "0x2c0 - desc PD_STAT"]
    pub pd_stat: crate::Reg<pd_stat::PD_STAT_SPEC>,
    _reserved131: [u8; 0x0c],
    #[doc = "0x2d0 - desc PD_ICLR"]
    pub pd_iclr: crate::Reg<pd_iclr::PD_ICLR_SPEC>,
    _reserved132: [u8; 0x2c],
    #[doc = "0x300 - desc CTRL0"]
    pub ctrl0: crate::Reg<ctrl0::CTRL0_SPEC>,
    #[doc = "0x304 - desc CTRL1"]
    pub ctrl1: crate::Reg<ctrl1::CTRL1_SPEC>,
    #[doc = "0x308 - desc CTRL2"]
    pub ctrl2: crate::Reg<ctrl2::CTRL2_SPEC>,
    #[doc = "0x30c - desc TIMGS"]
    pub timgs: crate::Reg<timgs::TIMGS_SPEC>,
    #[doc = "0x310 - desc TIMES"]
    pub times: crate::Reg<times::TIMES_SPEC>,
    #[doc = "0x314 - desc TIMCPS"]
    pub timcps: crate::Reg<timcps::TIMCPS_SPEC>,
    #[doc = "0x318 - desc PCAS"]
    pub pcas: crate::Reg<pcas::PCAS_SPEC>,
    #[doc = "0x31c - desc PCNT"]
    pub pcnt: crate::Reg<pcnt::PCNT_SPEC>,
    _reserved140: [u8; 0x0ce0],
    #[doc = "0x1000 - desc PE00_SEL"]
    pub pe00_sel: crate::Reg<pe00_sel::PE00_SEL_SPEC>,
    #[doc = "0x1004 - desc PE01_SEL"]
    pub pe01_sel: crate::Reg<pe01_sel::PE01_SEL_SPEC>,
    #[doc = "0x1008 - desc PE02_SEL"]
    pub pe02_sel: crate::Reg<pe02_sel::PE02_SEL_SPEC>,
    #[doc = "0x100c - desc PE03_SEL"]
    pub pe03_sel: crate::Reg<pe03_sel::PE03_SEL_SPEC>,
    #[doc = "0x1010 - desc PE04_SEL"]
    pub pe04_sel: crate::Reg<pe04_sel::PE04_SEL_SPEC>,
    #[doc = "0x1014 - desc PE05_SEL"]
    pub pe05_sel: crate::Reg<pe05_sel::PE05_SEL_SPEC>,
    #[doc = "0x1018 - desc PE06_SEL"]
    pub pe06_sel: crate::Reg<pe06_sel::PE06_SEL_SPEC>,
    #[doc = "0x101c - desc PE07_SEL"]
    pub pe07_sel: crate::Reg<pe07_sel::PE07_SEL_SPEC>,
    #[doc = "0x1020 - desc PE08_SEL"]
    pub pe08_sel: crate::Reg<pe08_sel::PE08_SEL_SPEC>,
    #[doc = "0x1024 - desc PE09_SEL"]
    pub pe09_sel: crate::Reg<pe09_sel::PE09_SEL_SPEC>,
    #[doc = "0x1028 - desc PE10_SEL"]
    pub pe10_sel: crate::Reg<pe10_sel::PE10_SEL_SPEC>,
    #[doc = "0x102c - desc PE11_SEL"]
    pub pe11_sel: crate::Reg<pe11_sel::PE11_SEL_SPEC>,
    #[doc = "0x1030 - desc PE12_SEL"]
    pub pe12_sel: crate::Reg<pe12_sel::PE12_SEL_SPEC>,
    #[doc = "0x1034 - desc PE13_SEL"]
    pub pe13_sel: crate::Reg<pe13_sel::PE13_SEL_SPEC>,
    #[doc = "0x1038 - desc PE14_SEL"]
    pub pe14_sel: crate::Reg<pe14_sel::PE14_SEL_SPEC>,
    #[doc = "0x103c - desc PE15_SEL"]
    pub pe15_sel: crate::Reg<pe15_sel::PE15_SEL_SPEC>,
    #[doc = "0x1040 - desc PF00_SEL"]
    pub pf00_sel: crate::Reg<pf00_sel::PF00_SEL_SPEC>,
    #[doc = "0x1044 - desc PF01_SEL"]
    pub pf01_sel: crate::Reg<pf01_sel::PF01_SEL_SPEC>,
    #[doc = "0x1048 - desc PF02_SEL"]
    pub pf02_sel: crate::Reg<pf02_sel::PF02_SEL_SPEC>,
    #[doc = "0x104c - desc PF03_SEL"]
    pub pf03_sel: crate::Reg<pf03_sel::PF03_SEL_SPEC>,
    #[doc = "0x1050 - desc PF04_SEL"]
    pub pf04_sel: crate::Reg<pf04_sel::PF04_SEL_SPEC>,
    #[doc = "0x1054 - desc PF05_SEL"]
    pub pf05_sel: crate::Reg<pf05_sel::PF05_SEL_SPEC>,
    #[doc = "0x1058 - desc PF06_SEL"]
    pub pf06_sel: crate::Reg<pf06_sel::PF06_SEL_SPEC>,
    #[doc = "0x105c - desc PF07_SEL"]
    pub pf07_sel: crate::Reg<pf07_sel::PF07_SEL_SPEC>,
    #[doc = "0x1060 - desc PF08_SEL"]
    pub pf08_sel: crate::Reg<pf08_sel::PF08_SEL_SPEC>,
    #[doc = "0x1064 - desc PF09_SEL"]
    pub pf09_sel: crate::Reg<pf09_sel::PF09_SEL_SPEC>,
    #[doc = "0x1068 - desc PF10_SEL"]
    pub pf10_sel: crate::Reg<pf10_sel::PF10_SEL_SPEC>,
    #[doc = "0x106c - desc PF11_SEL"]
    pub pf11_sel: crate::Reg<pf11_sel::PF11_SEL_SPEC>,
    _reserved168: [u8; 0x90],
    #[doc = "0x1100 - desc PEDIR"]
    pub pedir: crate::Reg<pedir::PEDIR_SPEC>,
    #[doc = "0x1104 - desc PEIN"]
    pub pein: crate::Reg<pein::PEIN_SPEC>,
    #[doc = "0x1108 - desc PEOUT"]
    pub peout: crate::Reg<peout::PEOUT_SPEC>,
    #[doc = "0x110c - desc PEADS"]
    pub peads: crate::Reg<peads::PEADS_SPEC>,
    #[doc = "0x1110 - desc PEBSET"]
    pub pebset: crate::Reg<pebset::PEBSET_SPEC>,
    #[doc = "0x1114 - desc PEBCLR"]
    pub pebclr: crate::Reg<pebclr::PEBCLR_SPEC>,
    #[doc = "0x1118 - desc PEBSETCLR"]
    pub pebsetclr: crate::Reg<pebsetclr::PEBSETCLR_SPEC>,
    #[doc = "0x111c - desc PEDR"]
    pub pedr: crate::Reg<pedr::PEDR_SPEC>,
    #[doc = "0x1120 - desc PEPU"]
    pub pepu: crate::Reg<pepu::PEPU_SPEC>,
    #[doc = "0x1124 - desc PEPD"]
    pub pepd: crate::Reg<pepd::PEPD_SPEC>,
    _reserved178: [u8; 0x04],
    #[doc = "0x112c - desc PEOD"]
    pub peod: crate::Reg<peod::PEOD_SPEC>,
    #[doc = "0x1130 - desc PEHIE"]
    pub pehie: crate::Reg<pehie::PEHIE_SPEC>,
    #[doc = "0x1134 - desc PELIE"]
    pub pelie: crate::Reg<pelie::PELIE_SPEC>,
    #[doc = "0x1138 - desc PERIE"]
    pub perie: crate::Reg<perie::PERIE_SPEC>,
    #[doc = "0x113c - desc PEFIE"]
    pub pefie: crate::Reg<pefie::PEFIE_SPEC>,
    #[doc = "0x1140 - desc PFDIR"]
    pub pfdir: crate::Reg<pfdir::PFDIR_SPEC>,
    #[doc = "0x1144 - desc PFIN"]
    pub pfin: crate::Reg<pfin::PFIN_SPEC>,
    #[doc = "0x1148 - desc PFOUT"]
    pub pfout: crate::Reg<pfout::PFOUT_SPEC>,
    #[doc = "0x114c - desc PFADS"]
    pub pfads: crate::Reg<pfads::PFADS_SPEC>,
    #[doc = "0x1150 - desc PFBSET"]
    pub pfbset: crate::Reg<pfbset::PFBSET_SPEC>,
    #[doc = "0x1154 - desc PFBCLR"]
    pub pfbclr: crate::Reg<pfbclr::PFBCLR_SPEC>,
    #[doc = "0x1158 - desc PFBSETCLR"]
    pub pfbsetclr: crate::Reg<pfbsetclr::PFBSETCLR_SPEC>,
    #[doc = "0x115c - desc PFDR"]
    pub pfdr: crate::Reg<pfdr::PFDR_SPEC>,
    #[doc = "0x1160 - desc PFPU"]
    pub pfpu: crate::Reg<pfpu::PFPU_SPEC>,
    #[doc = "0x1164 - desc PFPD"]
    pub pfpd: crate::Reg<pfpd::PFPD_SPEC>,
    _reserved193: [u8; 0x04],
    #[doc = "0x116c - desc PFOD"]
    pub pfod: crate::Reg<pfod::PFOD_SPEC>,
    #[doc = "0x1170 - desc PFHIE"]
    pub pfhie: crate::Reg<pfhie::PFHIE_SPEC>,
    #[doc = "0x1174 - desc PFLIE"]
    pub pflie: crate::Reg<pflie::PFLIE_SPEC>,
    #[doc = "0x1178 - desc PFRIE"]
    pub pfrie: crate::Reg<pfrie::PFRIE_SPEC>,
    #[doc = "0x117c - desc PFFIE"]
    pub pffie: crate::Reg<pffie::PFFIE_SPEC>,
    _reserved198: [u8; 0x80],
    #[doc = "0x1200 - desc PE_STAT"]
    pub pe_stat: crate::Reg<pe_stat::PE_STAT_SPEC>,
    _reserved199: [u8; 0x0c],
    #[doc = "0x1210 - desc PE_ICLR"]
    pub pe_iclr: crate::Reg<pe_iclr::PE_ICLR_SPEC>,
    _reserved200: [u8; 0x2c],
    #[doc = "0x1240 - desc PF_STAT"]
    pub pf_stat: crate::Reg<pf_stat::PF_STAT_SPEC>,
    _reserved201: [u8; 0x0c],
    #[doc = "0x1250 - desc PF_ICLR"]
    pub pf_iclr: crate::Reg<pf_iclr::PF_ICLR_SPEC>,
}
#[doc = "PA00_SEL register accessor: an alias for `Reg<PA00_SEL_SPEC>`"]
pub type PA00_SEL = crate::Reg<pa00_sel::PA00_SEL_SPEC>;
#[doc = "desc PA00_SEL"]
pub mod pa00_sel;
#[doc = "PA01_SEL register accessor: an alias for `Reg<PA01_SEL_SPEC>`"]
pub type PA01_SEL = crate::Reg<pa01_sel::PA01_SEL_SPEC>;
#[doc = "desc PA01_SEL"]
pub mod pa01_sel;
#[doc = "PA02_SEL register accessor: an alias for `Reg<PA02_SEL_SPEC>`"]
pub type PA02_SEL = crate::Reg<pa02_sel::PA02_SEL_SPEC>;
#[doc = "desc PA02_SEL"]
pub mod pa02_sel;
#[doc = "PA03_SEL register accessor: an alias for `Reg<PA03_SEL_SPEC>`"]
pub type PA03_SEL = crate::Reg<pa03_sel::PA03_SEL_SPEC>;
#[doc = "desc PA03_SEL"]
pub mod pa03_sel;
#[doc = "PA04_SEL register accessor: an alias for `Reg<PA04_SEL_SPEC>`"]
pub type PA04_SEL = crate::Reg<pa04_sel::PA04_SEL_SPEC>;
#[doc = "desc PA04_SEL"]
pub mod pa04_sel;
#[doc = "PA05_SEL register accessor: an alias for `Reg<PA05_SEL_SPEC>`"]
pub type PA05_SEL = crate::Reg<pa05_sel::PA05_SEL_SPEC>;
#[doc = "desc PA05_SEL"]
pub mod pa05_sel;
#[doc = "PA06_SEL register accessor: an alias for `Reg<PA06_SEL_SPEC>`"]
pub type PA06_SEL = crate::Reg<pa06_sel::PA06_SEL_SPEC>;
#[doc = "desc PA06_SEL"]
pub mod pa06_sel;
#[doc = "PA07_SEL register accessor: an alias for `Reg<PA07_SEL_SPEC>`"]
pub type PA07_SEL = crate::Reg<pa07_sel::PA07_SEL_SPEC>;
#[doc = "desc PA07_SEL"]
pub mod pa07_sel;
#[doc = "PA08_SEL register accessor: an alias for `Reg<PA08_SEL_SPEC>`"]
pub type PA08_SEL = crate::Reg<pa08_sel::PA08_SEL_SPEC>;
#[doc = "desc PA08_SEL"]
pub mod pa08_sel;
#[doc = "PA09_SEL register accessor: an alias for `Reg<PA09_SEL_SPEC>`"]
pub type PA09_SEL = crate::Reg<pa09_sel::PA09_SEL_SPEC>;
#[doc = "desc PA09_SEL"]
pub mod pa09_sel;
#[doc = "PA10_SEL register accessor: an alias for `Reg<PA10_SEL_SPEC>`"]
pub type PA10_SEL = crate::Reg<pa10_sel::PA10_SEL_SPEC>;
#[doc = "desc PA10_SEL"]
pub mod pa10_sel;
#[doc = "PA11_SEL register accessor: an alias for `Reg<PA11_SEL_SPEC>`"]
pub type PA11_SEL = crate::Reg<pa11_sel::PA11_SEL_SPEC>;
#[doc = "desc PA11_SEL"]
pub mod pa11_sel;
#[doc = "PA12_SEL register accessor: an alias for `Reg<PA12_SEL_SPEC>`"]
pub type PA12_SEL = crate::Reg<pa12_sel::PA12_SEL_SPEC>;
#[doc = "desc PA12_SEL"]
pub mod pa12_sel;
#[doc = "PA13_SEL register accessor: an alias for `Reg<PA13_SEL_SPEC>`"]
pub type PA13_SEL = crate::Reg<pa13_sel::PA13_SEL_SPEC>;
#[doc = "desc PA13_SEL"]
pub mod pa13_sel;
#[doc = "PA14_SEL register accessor: an alias for `Reg<PA14_SEL_SPEC>`"]
pub type PA14_SEL = crate::Reg<pa14_sel::PA14_SEL_SPEC>;
#[doc = "desc PA14_SEL"]
pub mod pa14_sel;
#[doc = "PA15_SEL register accessor: an alias for `Reg<PA15_SEL_SPEC>`"]
pub type PA15_SEL = crate::Reg<pa15_sel::PA15_SEL_SPEC>;
#[doc = "desc PA15_SEL"]
pub mod pa15_sel;
#[doc = "PB00_SEL register accessor: an alias for `Reg<PB00_SEL_SPEC>`"]
pub type PB00_SEL = crate::Reg<pb00_sel::PB00_SEL_SPEC>;
#[doc = "desc PB00_SEL"]
pub mod pb00_sel;
#[doc = "PB01_SEL register accessor: an alias for `Reg<PB01_SEL_SPEC>`"]
pub type PB01_SEL = crate::Reg<pb01_sel::PB01_SEL_SPEC>;
#[doc = "desc PB01_SEL"]
pub mod pb01_sel;
#[doc = "PB02_SEL register accessor: an alias for `Reg<PB02_SEL_SPEC>`"]
pub type PB02_SEL = crate::Reg<pb02_sel::PB02_SEL_SPEC>;
#[doc = "desc PB02_SEL"]
pub mod pb02_sel;
#[doc = "PB03_SEL register accessor: an alias for `Reg<PB03_SEL_SPEC>`"]
pub type PB03_SEL = crate::Reg<pb03_sel::PB03_SEL_SPEC>;
#[doc = "desc PB03_SEL"]
pub mod pb03_sel;
#[doc = "PB04_SEL register accessor: an alias for `Reg<PB04_SEL_SPEC>`"]
pub type PB04_SEL = crate::Reg<pb04_sel::PB04_SEL_SPEC>;
#[doc = "desc PB04_SEL"]
pub mod pb04_sel;
#[doc = "PB05_SEL register accessor: an alias for `Reg<PB05_SEL_SPEC>`"]
pub type PB05_SEL = crate::Reg<pb05_sel::PB05_SEL_SPEC>;
#[doc = "desc PB05_SEL"]
pub mod pb05_sel;
#[doc = "PB06_SEL register accessor: an alias for `Reg<PB06_SEL_SPEC>`"]
pub type PB06_SEL = crate::Reg<pb06_sel::PB06_SEL_SPEC>;
#[doc = "desc PB06_SEL"]
pub mod pb06_sel;
#[doc = "PB07_SEL register accessor: an alias for `Reg<PB07_SEL_SPEC>`"]
pub type PB07_SEL = crate::Reg<pb07_sel::PB07_SEL_SPEC>;
#[doc = "desc PB07_SEL"]
pub mod pb07_sel;
#[doc = "PB08_SEL register accessor: an alias for `Reg<PB08_SEL_SPEC>`"]
pub type PB08_SEL = crate::Reg<pb08_sel::PB08_SEL_SPEC>;
#[doc = "desc PB08_SEL"]
pub mod pb08_sel;
#[doc = "PB09_SEL register accessor: an alias for `Reg<PB09_SEL_SPEC>`"]
pub type PB09_SEL = crate::Reg<pb09_sel::PB09_SEL_SPEC>;
#[doc = "desc PB09_SEL"]
pub mod pb09_sel;
#[doc = "PB10_SEL register accessor: an alias for `Reg<PB10_SEL_SPEC>`"]
pub type PB10_SEL = crate::Reg<pb10_sel::PB10_SEL_SPEC>;
#[doc = "desc PB10_SEL"]
pub mod pb10_sel;
#[doc = "PB11_SEL register accessor: an alias for `Reg<PB11_SEL_SPEC>`"]
pub type PB11_SEL = crate::Reg<pb11_sel::PB11_SEL_SPEC>;
#[doc = "desc PB11_SEL"]
pub mod pb11_sel;
#[doc = "PB12_SEL register accessor: an alias for `Reg<PB12_SEL_SPEC>`"]
pub type PB12_SEL = crate::Reg<pb12_sel::PB12_SEL_SPEC>;
#[doc = "desc PB12_SEL"]
pub mod pb12_sel;
#[doc = "PB13_SEL register accessor: an alias for `Reg<PB13_SEL_SPEC>`"]
pub type PB13_SEL = crate::Reg<pb13_sel::PB13_SEL_SPEC>;
#[doc = "desc PB13_SEL"]
pub mod pb13_sel;
#[doc = "PB14_SEL register accessor: an alias for `Reg<PB14_SEL_SPEC>`"]
pub type PB14_SEL = crate::Reg<pb14_sel::PB14_SEL_SPEC>;
#[doc = "desc PB14_SEL"]
pub mod pb14_sel;
#[doc = "PB15_SEL register accessor: an alias for `Reg<PB15_SEL_SPEC>`"]
pub type PB15_SEL = crate::Reg<pb15_sel::PB15_SEL_SPEC>;
#[doc = "desc PB15_SEL"]
pub mod pb15_sel;
#[doc = "PC00_SEL register accessor: an alias for `Reg<PC00_SEL_SPEC>`"]
pub type PC00_SEL = crate::Reg<pc00_sel::PC00_SEL_SPEC>;
#[doc = "desc PC00_SEL"]
pub mod pc00_sel;
#[doc = "PC01_SEL register accessor: an alias for `Reg<PC01_SEL_SPEC>`"]
pub type PC01_SEL = crate::Reg<pc01_sel::PC01_SEL_SPEC>;
#[doc = "desc PC01_SEL"]
pub mod pc01_sel;
#[doc = "PC02_SEL register accessor: an alias for `Reg<PC02_SEL_SPEC>`"]
pub type PC02_SEL = crate::Reg<pc02_sel::PC02_SEL_SPEC>;
#[doc = "desc PC02_SEL"]
pub mod pc02_sel;
#[doc = "PC03_SEL register accessor: an alias for `Reg<PC03_SEL_SPEC>`"]
pub type PC03_SEL = crate::Reg<pc03_sel::PC03_SEL_SPEC>;
#[doc = "desc PC03_SEL"]
pub mod pc03_sel;
#[doc = "PC04_SEL register accessor: an alias for `Reg<PC04_SEL_SPEC>`"]
pub type PC04_SEL = crate::Reg<pc04_sel::PC04_SEL_SPEC>;
#[doc = "desc PC04_SEL"]
pub mod pc04_sel;
#[doc = "PC05_SEL register accessor: an alias for `Reg<PC05_SEL_SPEC>`"]
pub type PC05_SEL = crate::Reg<pc05_sel::PC05_SEL_SPEC>;
#[doc = "desc PC05_SEL"]
pub mod pc05_sel;
#[doc = "PC06_SEL register accessor: an alias for `Reg<PC06_SEL_SPEC>`"]
pub type PC06_SEL = crate::Reg<pc06_sel::PC06_SEL_SPEC>;
#[doc = "desc PC06_SEL"]
pub mod pc06_sel;
#[doc = "PC07_SEL register accessor: an alias for `Reg<PC07_SEL_SPEC>`"]
pub type PC07_SEL = crate::Reg<pc07_sel::PC07_SEL_SPEC>;
#[doc = "desc PC07_SEL"]
pub mod pc07_sel;
#[doc = "PC08_SEL register accessor: an alias for `Reg<PC08_SEL_SPEC>`"]
pub type PC08_SEL = crate::Reg<pc08_sel::PC08_SEL_SPEC>;
#[doc = "desc PC08_SEL"]
pub mod pc08_sel;
#[doc = "PC09_SEL register accessor: an alias for `Reg<PC09_SEL_SPEC>`"]
pub type PC09_SEL = crate::Reg<pc09_sel::PC09_SEL_SPEC>;
#[doc = "desc PC09_SEL"]
pub mod pc09_sel;
#[doc = "PC10_SEL register accessor: an alias for `Reg<PC10_SEL_SPEC>`"]
pub type PC10_SEL = crate::Reg<pc10_sel::PC10_SEL_SPEC>;
#[doc = "desc PC10_SEL"]
pub mod pc10_sel;
#[doc = "PC11_SEL register accessor: an alias for `Reg<PC11_SEL_SPEC>`"]
pub type PC11_SEL = crate::Reg<pc11_sel::PC11_SEL_SPEC>;
#[doc = "desc PC11_SEL"]
pub mod pc11_sel;
#[doc = "PC12_SEL register accessor: an alias for `Reg<PC12_SEL_SPEC>`"]
pub type PC12_SEL = crate::Reg<pc12_sel::PC12_SEL_SPEC>;
#[doc = "desc PC12_SEL"]
pub mod pc12_sel;
#[doc = "PC13_SEL register accessor: an alias for `Reg<PC13_SEL_SPEC>`"]
pub type PC13_SEL = crate::Reg<pc13_sel::PC13_SEL_SPEC>;
#[doc = "desc PC13_SEL"]
pub mod pc13_sel;
#[doc = "PC14_SEL register accessor: an alias for `Reg<PC14_SEL_SPEC>`"]
pub type PC14_SEL = crate::Reg<pc14_sel::PC14_SEL_SPEC>;
#[doc = "desc PC14_SEL"]
pub mod pc14_sel;
#[doc = "PC15_SEL register accessor: an alias for `Reg<PC15_SEL_SPEC>`"]
pub type PC15_SEL = crate::Reg<pc15_sel::PC15_SEL_SPEC>;
#[doc = "desc PC15_SEL"]
pub mod pc15_sel;
#[doc = "PD00_SEL register accessor: an alias for `Reg<PD00_SEL_SPEC>`"]
pub type PD00_SEL = crate::Reg<pd00_sel::PD00_SEL_SPEC>;
#[doc = "desc PD00_SEL"]
pub mod pd00_sel;
#[doc = "PD01_SEL register accessor: an alias for `Reg<PD01_SEL_SPEC>`"]
pub type PD01_SEL = crate::Reg<pd01_sel::PD01_SEL_SPEC>;
#[doc = "desc PD01_SEL"]
pub mod pd01_sel;
#[doc = "PD02_SEL register accessor: an alias for `Reg<PD02_SEL_SPEC>`"]
pub type PD02_SEL = crate::Reg<pd02_sel::PD02_SEL_SPEC>;
#[doc = "desc PD02_SEL"]
pub mod pd02_sel;
#[doc = "PD03_SEL register accessor: an alias for `Reg<PD03_SEL_SPEC>`"]
pub type PD03_SEL = crate::Reg<pd03_sel::PD03_SEL_SPEC>;
#[doc = "desc PD03_SEL"]
pub mod pd03_sel;
#[doc = "PD04_SEL register accessor: an alias for `Reg<PD04_SEL_SPEC>`"]
pub type PD04_SEL = crate::Reg<pd04_sel::PD04_SEL_SPEC>;
#[doc = "desc PD04_SEL"]
pub mod pd04_sel;
#[doc = "PD05_SEL register accessor: an alias for `Reg<PD05_SEL_SPEC>`"]
pub type PD05_SEL = crate::Reg<pd05_sel::PD05_SEL_SPEC>;
#[doc = "desc PD05_SEL"]
pub mod pd05_sel;
#[doc = "PD06_SEL register accessor: an alias for `Reg<PD06_SEL_SPEC>`"]
pub type PD06_SEL = crate::Reg<pd06_sel::PD06_SEL_SPEC>;
#[doc = "desc PD06_SEL"]
pub mod pd06_sel;
#[doc = "PD07_SEL register accessor: an alias for `Reg<PD07_SEL_SPEC>`"]
pub type PD07_SEL = crate::Reg<pd07_sel::PD07_SEL_SPEC>;
#[doc = "desc PD07_SEL"]
pub mod pd07_sel;
#[doc = "PD08_SEL register accessor: an alias for `Reg<PD08_SEL_SPEC>`"]
pub type PD08_SEL = crate::Reg<pd08_sel::PD08_SEL_SPEC>;
#[doc = "desc PD08_SEL"]
pub mod pd08_sel;
#[doc = "PD09_SEL register accessor: an alias for `Reg<PD09_SEL_SPEC>`"]
pub type PD09_SEL = crate::Reg<pd09_sel::PD09_SEL_SPEC>;
#[doc = "desc PD09_SEL"]
pub mod pd09_sel;
#[doc = "PD10_SEL register accessor: an alias for `Reg<PD10_SEL_SPEC>`"]
pub type PD10_SEL = crate::Reg<pd10_sel::PD10_SEL_SPEC>;
#[doc = "desc PD10_SEL"]
pub mod pd10_sel;
#[doc = "PD11_SEL register accessor: an alias for `Reg<PD11_SEL_SPEC>`"]
pub type PD11_SEL = crate::Reg<pd11_sel::PD11_SEL_SPEC>;
#[doc = "desc PD11_SEL"]
pub mod pd11_sel;
#[doc = "PD12_SEL register accessor: an alias for `Reg<PD12_SEL_SPEC>`"]
pub type PD12_SEL = crate::Reg<pd12_sel::PD12_SEL_SPEC>;
#[doc = "desc PD12_SEL"]
pub mod pd12_sel;
#[doc = "PD13_SEL register accessor: an alias for `Reg<PD13_SEL_SPEC>`"]
pub type PD13_SEL = crate::Reg<pd13_sel::PD13_SEL_SPEC>;
#[doc = "desc PD13_SEL"]
pub mod pd13_sel;
#[doc = "PD14_SEL register accessor: an alias for `Reg<PD14_SEL_SPEC>`"]
pub type PD14_SEL = crate::Reg<pd14_sel::PD14_SEL_SPEC>;
#[doc = "desc PD14_SEL"]
pub mod pd14_sel;
#[doc = "PD15_SEL register accessor: an alias for `Reg<PD15_SEL_SPEC>`"]
pub type PD15_SEL = crate::Reg<pd15_sel::PD15_SEL_SPEC>;
#[doc = "desc PD15_SEL"]
pub mod pd15_sel;
#[doc = "PADIR register accessor: an alias for `Reg<PADIR_SPEC>`"]
pub type PADIR = crate::Reg<padir::PADIR_SPEC>;
#[doc = "desc PADIR"]
pub mod padir;
#[doc = "PAIN register accessor: an alias for `Reg<PAIN_SPEC>`"]
pub type PAIN = crate::Reg<pain::PAIN_SPEC>;
#[doc = "desc PAIN"]
pub mod pain;
#[doc = "PAOUT register accessor: an alias for `Reg<PAOUT_SPEC>`"]
pub type PAOUT = crate::Reg<paout::PAOUT_SPEC>;
#[doc = "desc PAOUT"]
pub mod paout;
#[doc = "PAADS register accessor: an alias for `Reg<PAADS_SPEC>`"]
pub type PAADS = crate::Reg<paads::PAADS_SPEC>;
#[doc = "desc PAADS"]
pub mod paads;
#[doc = "PABSET register accessor: an alias for `Reg<PABSET_SPEC>`"]
pub type PABSET = crate::Reg<pabset::PABSET_SPEC>;
#[doc = "desc PABSET"]
pub mod pabset;
#[doc = "PABCLR register accessor: an alias for `Reg<PABCLR_SPEC>`"]
pub type PABCLR = crate::Reg<pabclr::PABCLR_SPEC>;
#[doc = "desc PABCLR"]
pub mod pabclr;
#[doc = "PABSETCLR register accessor: an alias for `Reg<PABSETCLR_SPEC>`"]
pub type PABSETCLR = crate::Reg<pabsetclr::PABSETCLR_SPEC>;
#[doc = "desc PABSETCLR"]
pub mod pabsetclr;
#[doc = "PADR register accessor: an alias for `Reg<PADR_SPEC>`"]
pub type PADR = crate::Reg<padr::PADR_SPEC>;
#[doc = "desc PADR"]
pub mod padr;
#[doc = "PAPU register accessor: an alias for `Reg<PAPU_SPEC>`"]
pub type PAPU = crate::Reg<papu::PAPU_SPEC>;
#[doc = "desc PAPU"]
pub mod papu;
#[doc = "PAPD register accessor: an alias for `Reg<PAPD_SPEC>`"]
pub type PAPD = crate::Reg<papd::PAPD_SPEC>;
#[doc = "desc PAPD"]
pub mod papd;
#[doc = "PAOD register accessor: an alias for `Reg<PAOD_SPEC>`"]
pub type PAOD = crate::Reg<paod::PAOD_SPEC>;
#[doc = "desc PAOD"]
pub mod paod;
#[doc = "PAHIE register accessor: an alias for `Reg<PAHIE_SPEC>`"]
pub type PAHIE = crate::Reg<pahie::PAHIE_SPEC>;
#[doc = "desc PAHIE"]
pub mod pahie;
#[doc = "PALIE register accessor: an alias for `Reg<PALIE_SPEC>`"]
pub type PALIE = crate::Reg<palie::PALIE_SPEC>;
#[doc = "desc PALIE"]
pub mod palie;
#[doc = "PARIE register accessor: an alias for `Reg<PARIE_SPEC>`"]
pub type PARIE = crate::Reg<parie::PARIE_SPEC>;
#[doc = "desc PARIE"]
pub mod parie;
#[doc = "PAFIE register accessor: an alias for `Reg<PAFIE_SPEC>`"]
pub type PAFIE = crate::Reg<pafie::PAFIE_SPEC>;
#[doc = "desc PAFIE"]
pub mod pafie;
#[doc = "PBDIR register accessor: an alias for `Reg<PBDIR_SPEC>`"]
pub type PBDIR = crate::Reg<pbdir::PBDIR_SPEC>;
#[doc = "desc PBDIR"]
pub mod pbdir;
#[doc = "PBIN register accessor: an alias for `Reg<PBIN_SPEC>`"]
pub type PBIN = crate::Reg<pbin::PBIN_SPEC>;
#[doc = "desc PBIN"]
pub mod pbin;
#[doc = "PBOUT register accessor: an alias for `Reg<PBOUT_SPEC>`"]
pub type PBOUT = crate::Reg<pbout::PBOUT_SPEC>;
#[doc = "desc PBOUT"]
pub mod pbout;
#[doc = "PBADS register accessor: an alias for `Reg<PBADS_SPEC>`"]
pub type PBADS = crate::Reg<pbads::PBADS_SPEC>;
#[doc = "desc PBADS"]
pub mod pbads;
#[doc = "PBBSET register accessor: an alias for `Reg<PBBSET_SPEC>`"]
pub type PBBSET = crate::Reg<pbbset::PBBSET_SPEC>;
#[doc = "desc PBBSET"]
pub mod pbbset;
#[doc = "PBBCLR register accessor: an alias for `Reg<PBBCLR_SPEC>`"]
pub type PBBCLR = crate::Reg<pbbclr::PBBCLR_SPEC>;
#[doc = "desc PBBCLR"]
pub mod pbbclr;
#[doc = "PBBSETCLR register accessor: an alias for `Reg<PBBSETCLR_SPEC>`"]
pub type PBBSETCLR = crate::Reg<pbbsetclr::PBBSETCLR_SPEC>;
#[doc = "desc PBBSETCLR"]
pub mod pbbsetclr;
#[doc = "PBDR register accessor: an alias for `Reg<PBDR_SPEC>`"]
pub type PBDR = crate::Reg<pbdr::PBDR_SPEC>;
#[doc = "desc PBDR"]
pub mod pbdr;
#[doc = "PBPU register accessor: an alias for `Reg<PBPU_SPEC>`"]
pub type PBPU = crate::Reg<pbpu::PBPU_SPEC>;
#[doc = "desc PBPU"]
pub mod pbpu;
#[doc = "PBPD register accessor: an alias for `Reg<PBPD_SPEC>`"]
pub type PBPD = crate::Reg<pbpd::PBPD_SPEC>;
#[doc = "desc PBPD"]
pub mod pbpd;
#[doc = "PBOD register accessor: an alias for `Reg<PBOD_SPEC>`"]
pub type PBOD = crate::Reg<pbod::PBOD_SPEC>;
#[doc = "desc PBOD"]
pub mod pbod;
#[doc = "PBHIE register accessor: an alias for `Reg<PBHIE_SPEC>`"]
pub type PBHIE = crate::Reg<pbhie::PBHIE_SPEC>;
#[doc = "desc PBHIE"]
pub mod pbhie;
#[doc = "PBLIE register accessor: an alias for `Reg<PBLIE_SPEC>`"]
pub type PBLIE = crate::Reg<pblie::PBLIE_SPEC>;
#[doc = "desc PBLIE"]
pub mod pblie;
#[doc = "PBRIE register accessor: an alias for `Reg<PBRIE_SPEC>`"]
pub type PBRIE = crate::Reg<pbrie::PBRIE_SPEC>;
#[doc = "desc PBRIE"]
pub mod pbrie;
#[doc = "PBFIE register accessor: an alias for `Reg<PBFIE_SPEC>`"]
pub type PBFIE = crate::Reg<pbfie::PBFIE_SPEC>;
#[doc = "desc PBFIE"]
pub mod pbfie;
#[doc = "PCDIR register accessor: an alias for `Reg<PCDIR_SPEC>`"]
pub type PCDIR = crate::Reg<pcdir::PCDIR_SPEC>;
#[doc = "desc PCDIR"]
pub mod pcdir;
#[doc = "PCIN register accessor: an alias for `Reg<PCIN_SPEC>`"]
pub type PCIN = crate::Reg<pcin::PCIN_SPEC>;
#[doc = "desc PCIN"]
pub mod pcin;
#[doc = "PCOUT register accessor: an alias for `Reg<PCOUT_SPEC>`"]
pub type PCOUT = crate::Reg<pcout::PCOUT_SPEC>;
#[doc = "desc PCOUT"]
pub mod pcout;
#[doc = "PCADS register accessor: an alias for `Reg<PCADS_SPEC>`"]
pub type PCADS = crate::Reg<pcads::PCADS_SPEC>;
#[doc = "desc PCADS"]
pub mod pcads;
#[doc = "PCBSET register accessor: an alias for `Reg<PCBSET_SPEC>`"]
pub type PCBSET = crate::Reg<pcbset::PCBSET_SPEC>;
#[doc = "desc PCBSET"]
pub mod pcbset;
#[doc = "PCBCLR register accessor: an alias for `Reg<PCBCLR_SPEC>`"]
pub type PCBCLR = crate::Reg<pcbclr::PCBCLR_SPEC>;
#[doc = "desc PCBCLR"]
pub mod pcbclr;
#[doc = "PCBSETCLR register accessor: an alias for `Reg<PCBSETCLR_SPEC>`"]
pub type PCBSETCLR = crate::Reg<pcbsetclr::PCBSETCLR_SPEC>;
#[doc = "desc PCBSETCLR"]
pub mod pcbsetclr;
#[doc = "PCDR register accessor: an alias for `Reg<PCDR_SPEC>`"]
pub type PCDR = crate::Reg<pcdr::PCDR_SPEC>;
#[doc = "desc PCDR"]
pub mod pcdr;
#[doc = "PCPU register accessor: an alias for `Reg<PCPU_SPEC>`"]
pub type PCPU = crate::Reg<pcpu::PCPU_SPEC>;
#[doc = "desc PCPU"]
pub mod pcpu;
#[doc = "PCPD register accessor: an alias for `Reg<PCPD_SPEC>`"]
pub type PCPD = crate::Reg<pcpd::PCPD_SPEC>;
#[doc = "desc PCPD"]
pub mod pcpd;
#[doc = "PCOD register accessor: an alias for `Reg<PCOD_SPEC>`"]
pub type PCOD = crate::Reg<pcod::PCOD_SPEC>;
#[doc = "desc PCOD"]
pub mod pcod;
#[doc = "PCHIE register accessor: an alias for `Reg<PCHIE_SPEC>`"]
pub type PCHIE = crate::Reg<pchie::PCHIE_SPEC>;
#[doc = "desc PCHIE"]
pub mod pchie;
#[doc = "PCLIE register accessor: an alias for `Reg<PCLIE_SPEC>`"]
pub type PCLIE = crate::Reg<pclie::PCLIE_SPEC>;
#[doc = "desc PCLIE"]
pub mod pclie;
#[doc = "PCRIE register accessor: an alias for `Reg<PCRIE_SPEC>`"]
pub type PCRIE = crate::Reg<pcrie::PCRIE_SPEC>;
#[doc = "desc PCRIE"]
pub mod pcrie;
#[doc = "PCFIE register accessor: an alias for `Reg<PCFIE_SPEC>`"]
pub type PCFIE = crate::Reg<pcfie::PCFIE_SPEC>;
#[doc = "desc PCFIE"]
pub mod pcfie;
#[doc = "PDDIR register accessor: an alias for `Reg<PDDIR_SPEC>`"]
pub type PDDIR = crate::Reg<pddir::PDDIR_SPEC>;
#[doc = "desc PDDIR"]
pub mod pddir;
#[doc = "PDIN register accessor: an alias for `Reg<PDIN_SPEC>`"]
pub type PDIN = crate::Reg<pdin::PDIN_SPEC>;
#[doc = "desc PDIN"]
pub mod pdin;
#[doc = "PDOUT register accessor: an alias for `Reg<PDOUT_SPEC>`"]
pub type PDOUT = crate::Reg<pdout::PDOUT_SPEC>;
#[doc = "desc PDOUT"]
pub mod pdout;
#[doc = "PDADS register accessor: an alias for `Reg<PDADS_SPEC>`"]
pub type PDADS = crate::Reg<pdads::PDADS_SPEC>;
#[doc = "desc PDADS"]
pub mod pdads;
#[doc = "PDBSET register accessor: an alias for `Reg<PDBSET_SPEC>`"]
pub type PDBSET = crate::Reg<pdbset::PDBSET_SPEC>;
#[doc = "desc PDBSET"]
pub mod pdbset;
#[doc = "PDBCLR register accessor: an alias for `Reg<PDBCLR_SPEC>`"]
pub type PDBCLR = crate::Reg<pdbclr::PDBCLR_SPEC>;
#[doc = "desc PDBCLR"]
pub mod pdbclr;
#[doc = "PDBSETCLR register accessor: an alias for `Reg<PDBSETCLR_SPEC>`"]
pub type PDBSETCLR = crate::Reg<pdbsetclr::PDBSETCLR_SPEC>;
#[doc = "desc PDBSETCLR"]
pub mod pdbsetclr;
#[doc = "PDDR register accessor: an alias for `Reg<PDDR_SPEC>`"]
pub type PDDR = crate::Reg<pddr::PDDR_SPEC>;
#[doc = "desc PDDR"]
pub mod pddr;
#[doc = "PDPU register accessor: an alias for `Reg<PDPU_SPEC>`"]
pub type PDPU = crate::Reg<pdpu::PDPU_SPEC>;
#[doc = "desc PDPU"]
pub mod pdpu;
#[doc = "PDPD register accessor: an alias for `Reg<PDPD_SPEC>`"]
pub type PDPD = crate::Reg<pdpd::PDPD_SPEC>;
#[doc = "desc PDPD"]
pub mod pdpd;
#[doc = "PDOD register accessor: an alias for `Reg<PDOD_SPEC>`"]
pub type PDOD = crate::Reg<pdod::PDOD_SPEC>;
#[doc = "desc PDOD"]
pub mod pdod;
#[doc = "PDHIE register accessor: an alias for `Reg<PDHIE_SPEC>`"]
pub type PDHIE = crate::Reg<pdhie::PDHIE_SPEC>;
#[doc = "desc PDHIE"]
pub mod pdhie;
#[doc = "PDLIE register accessor: an alias for `Reg<PDLIE_SPEC>`"]
pub type PDLIE = crate::Reg<pdlie::PDLIE_SPEC>;
#[doc = "desc PDLIE"]
pub mod pdlie;
#[doc = "PDRIE register accessor: an alias for `Reg<PDRIE_SPEC>`"]
pub type PDRIE = crate::Reg<pdrie::PDRIE_SPEC>;
#[doc = "desc PDRIE"]
pub mod pdrie;
#[doc = "PDFIE register accessor: an alias for `Reg<PDFIE_SPEC>`"]
pub type PDFIE = crate::Reg<pdfie::PDFIE_SPEC>;
#[doc = "desc PDFIE"]
pub mod pdfie;
#[doc = "PA_STAT register accessor: an alias for `Reg<PA_STAT_SPEC>`"]
pub type PA_STAT = crate::Reg<pa_stat::PA_STAT_SPEC>;
#[doc = "desc PA_STAT"]
pub mod pa_stat;
#[doc = "PA_ICLR register accessor: an alias for `Reg<PA_ICLR_SPEC>`"]
pub type PA_ICLR = crate::Reg<pa_iclr::PA_ICLR_SPEC>;
#[doc = "desc PA_ICLR"]
pub mod pa_iclr;
#[doc = "PB_STAT register accessor: an alias for `Reg<PB_STAT_SPEC>`"]
pub type PB_STAT = crate::Reg<pb_stat::PB_STAT_SPEC>;
#[doc = "desc PB_STAT"]
pub mod pb_stat;
#[doc = "PB_ICLR register accessor: an alias for `Reg<PB_ICLR_SPEC>`"]
pub type PB_ICLR = crate::Reg<pb_iclr::PB_ICLR_SPEC>;
#[doc = "desc PB_ICLR"]
pub mod pb_iclr;
#[doc = "PC_STAT register accessor: an alias for `Reg<PC_STAT_SPEC>`"]
pub type PC_STAT = crate::Reg<pc_stat::PC_STAT_SPEC>;
#[doc = "desc PC_STAT"]
pub mod pc_stat;
#[doc = "PC_ICLR register accessor: an alias for `Reg<PC_ICLR_SPEC>`"]
pub type PC_ICLR = crate::Reg<pc_iclr::PC_ICLR_SPEC>;
#[doc = "desc PC_ICLR"]
pub mod pc_iclr;
#[doc = "PD_STAT register accessor: an alias for `Reg<PD_STAT_SPEC>`"]
pub type PD_STAT = crate::Reg<pd_stat::PD_STAT_SPEC>;
#[doc = "desc PD_STAT"]
pub mod pd_stat;
#[doc = "PD_ICLR register accessor: an alias for `Reg<PD_ICLR_SPEC>`"]
pub type PD_ICLR = crate::Reg<pd_iclr::PD_ICLR_SPEC>;
#[doc = "desc PD_ICLR"]
pub mod pd_iclr;
#[doc = "CTRL0 register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "desc CTRL0"]
pub mod ctrl0;
#[doc = "CTRL1 register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "desc CTRL1"]
pub mod ctrl1;
#[doc = "CTRL2 register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "desc CTRL2"]
pub mod ctrl2;
#[doc = "TIMGS register accessor: an alias for `Reg<TIMGS_SPEC>`"]
pub type TIMGS = crate::Reg<timgs::TIMGS_SPEC>;
#[doc = "desc TIMGS"]
pub mod timgs;
#[doc = "TIMES register accessor: an alias for `Reg<TIMES_SPEC>`"]
pub type TIMES = crate::Reg<times::TIMES_SPEC>;
#[doc = "desc TIMES"]
pub mod times;
#[doc = "TIMCPS register accessor: an alias for `Reg<TIMCPS_SPEC>`"]
pub type TIMCPS = crate::Reg<timcps::TIMCPS_SPEC>;
#[doc = "desc TIMCPS"]
pub mod timcps;
#[doc = "PCAS register accessor: an alias for `Reg<PCAS_SPEC>`"]
pub type PCAS = crate::Reg<pcas::PCAS_SPEC>;
#[doc = "desc PCAS"]
pub mod pcas;
#[doc = "PCNT register accessor: an alias for `Reg<PCNT_SPEC>`"]
pub type PCNT = crate::Reg<pcnt::PCNT_SPEC>;
#[doc = "desc PCNT"]
pub mod pcnt;
#[doc = "PE00_SEL register accessor: an alias for `Reg<PE00_SEL_SPEC>`"]
pub type PE00_SEL = crate::Reg<pe00_sel::PE00_SEL_SPEC>;
#[doc = "desc PE00_SEL"]
pub mod pe00_sel;
#[doc = "PE01_SEL register accessor: an alias for `Reg<PE01_SEL_SPEC>`"]
pub type PE01_SEL = crate::Reg<pe01_sel::PE01_SEL_SPEC>;
#[doc = "desc PE01_SEL"]
pub mod pe01_sel;
#[doc = "PE02_SEL register accessor: an alias for `Reg<PE02_SEL_SPEC>`"]
pub type PE02_SEL = crate::Reg<pe02_sel::PE02_SEL_SPEC>;
#[doc = "desc PE02_SEL"]
pub mod pe02_sel;
#[doc = "PE03_SEL register accessor: an alias for `Reg<PE03_SEL_SPEC>`"]
pub type PE03_SEL = crate::Reg<pe03_sel::PE03_SEL_SPEC>;
#[doc = "desc PE03_SEL"]
pub mod pe03_sel;
#[doc = "PE04_SEL register accessor: an alias for `Reg<PE04_SEL_SPEC>`"]
pub type PE04_SEL = crate::Reg<pe04_sel::PE04_SEL_SPEC>;
#[doc = "desc PE04_SEL"]
pub mod pe04_sel;
#[doc = "PE05_SEL register accessor: an alias for `Reg<PE05_SEL_SPEC>`"]
pub type PE05_SEL = crate::Reg<pe05_sel::PE05_SEL_SPEC>;
#[doc = "desc PE05_SEL"]
pub mod pe05_sel;
#[doc = "PE06_SEL register accessor: an alias for `Reg<PE06_SEL_SPEC>`"]
pub type PE06_SEL = crate::Reg<pe06_sel::PE06_SEL_SPEC>;
#[doc = "desc PE06_SEL"]
pub mod pe06_sel;
#[doc = "PE07_SEL register accessor: an alias for `Reg<PE07_SEL_SPEC>`"]
pub type PE07_SEL = crate::Reg<pe07_sel::PE07_SEL_SPEC>;
#[doc = "desc PE07_SEL"]
pub mod pe07_sel;
#[doc = "PE08_SEL register accessor: an alias for `Reg<PE08_SEL_SPEC>`"]
pub type PE08_SEL = crate::Reg<pe08_sel::PE08_SEL_SPEC>;
#[doc = "desc PE08_SEL"]
pub mod pe08_sel;
#[doc = "PE09_SEL register accessor: an alias for `Reg<PE09_SEL_SPEC>`"]
pub type PE09_SEL = crate::Reg<pe09_sel::PE09_SEL_SPEC>;
#[doc = "desc PE09_SEL"]
pub mod pe09_sel;
#[doc = "PE10_SEL register accessor: an alias for `Reg<PE10_SEL_SPEC>`"]
pub type PE10_SEL = crate::Reg<pe10_sel::PE10_SEL_SPEC>;
#[doc = "desc PE10_SEL"]
pub mod pe10_sel;
#[doc = "PE11_SEL register accessor: an alias for `Reg<PE11_SEL_SPEC>`"]
pub type PE11_SEL = crate::Reg<pe11_sel::PE11_SEL_SPEC>;
#[doc = "desc PE11_SEL"]
pub mod pe11_sel;
#[doc = "PE12_SEL register accessor: an alias for `Reg<PE12_SEL_SPEC>`"]
pub type PE12_SEL = crate::Reg<pe12_sel::PE12_SEL_SPEC>;
#[doc = "desc PE12_SEL"]
pub mod pe12_sel;
#[doc = "PE13_SEL register accessor: an alias for `Reg<PE13_SEL_SPEC>`"]
pub type PE13_SEL = crate::Reg<pe13_sel::PE13_SEL_SPEC>;
#[doc = "desc PE13_SEL"]
pub mod pe13_sel;
#[doc = "PE14_SEL register accessor: an alias for `Reg<PE14_SEL_SPEC>`"]
pub type PE14_SEL = crate::Reg<pe14_sel::PE14_SEL_SPEC>;
#[doc = "desc PE14_SEL"]
pub mod pe14_sel;
#[doc = "PE15_SEL register accessor: an alias for `Reg<PE15_SEL_SPEC>`"]
pub type PE15_SEL = crate::Reg<pe15_sel::PE15_SEL_SPEC>;
#[doc = "desc PE15_SEL"]
pub mod pe15_sel;
#[doc = "PF00_SEL register accessor: an alias for `Reg<PF00_SEL_SPEC>`"]
pub type PF00_SEL = crate::Reg<pf00_sel::PF00_SEL_SPEC>;
#[doc = "desc PF00_SEL"]
pub mod pf00_sel;
#[doc = "PF01_SEL register accessor: an alias for `Reg<PF01_SEL_SPEC>`"]
pub type PF01_SEL = crate::Reg<pf01_sel::PF01_SEL_SPEC>;
#[doc = "desc PF01_SEL"]
pub mod pf01_sel;
#[doc = "PF02_SEL register accessor: an alias for `Reg<PF02_SEL_SPEC>`"]
pub type PF02_SEL = crate::Reg<pf02_sel::PF02_SEL_SPEC>;
#[doc = "desc PF02_SEL"]
pub mod pf02_sel;
#[doc = "PF03_SEL register accessor: an alias for `Reg<PF03_SEL_SPEC>`"]
pub type PF03_SEL = crate::Reg<pf03_sel::PF03_SEL_SPEC>;
#[doc = "desc PF03_SEL"]
pub mod pf03_sel;
#[doc = "PF04_SEL register accessor: an alias for `Reg<PF04_SEL_SPEC>`"]
pub type PF04_SEL = crate::Reg<pf04_sel::PF04_SEL_SPEC>;
#[doc = "desc PF04_SEL"]
pub mod pf04_sel;
#[doc = "PF05_SEL register accessor: an alias for `Reg<PF05_SEL_SPEC>`"]
pub type PF05_SEL = crate::Reg<pf05_sel::PF05_SEL_SPEC>;
#[doc = "desc PF05_SEL"]
pub mod pf05_sel;
#[doc = "PF06_SEL register accessor: an alias for `Reg<PF06_SEL_SPEC>`"]
pub type PF06_SEL = crate::Reg<pf06_sel::PF06_SEL_SPEC>;
#[doc = "desc PF06_SEL"]
pub mod pf06_sel;
#[doc = "PF07_SEL register accessor: an alias for `Reg<PF07_SEL_SPEC>`"]
pub type PF07_SEL = crate::Reg<pf07_sel::PF07_SEL_SPEC>;
#[doc = "desc PF07_SEL"]
pub mod pf07_sel;
#[doc = "PF08_SEL register accessor: an alias for `Reg<PF08_SEL_SPEC>`"]
pub type PF08_SEL = crate::Reg<pf08_sel::PF08_SEL_SPEC>;
#[doc = "desc PF08_SEL"]
pub mod pf08_sel;
#[doc = "PF09_SEL register accessor: an alias for `Reg<PF09_SEL_SPEC>`"]
pub type PF09_SEL = crate::Reg<pf09_sel::PF09_SEL_SPEC>;
#[doc = "desc PF09_SEL"]
pub mod pf09_sel;
#[doc = "PF10_SEL register accessor: an alias for `Reg<PF10_SEL_SPEC>`"]
pub type PF10_SEL = crate::Reg<pf10_sel::PF10_SEL_SPEC>;
#[doc = "desc PF10_SEL"]
pub mod pf10_sel;
#[doc = "PF11_SEL register accessor: an alias for `Reg<PF11_SEL_SPEC>`"]
pub type PF11_SEL = crate::Reg<pf11_sel::PF11_SEL_SPEC>;
#[doc = "desc PF11_SEL"]
pub mod pf11_sel;
#[doc = "PEDIR register accessor: an alias for `Reg<PEDIR_SPEC>`"]
pub type PEDIR = crate::Reg<pedir::PEDIR_SPEC>;
#[doc = "desc PEDIR"]
pub mod pedir;
#[doc = "PEIN register accessor: an alias for `Reg<PEIN_SPEC>`"]
pub type PEIN = crate::Reg<pein::PEIN_SPEC>;
#[doc = "desc PEIN"]
pub mod pein;
#[doc = "PEOUT register accessor: an alias for `Reg<PEOUT_SPEC>`"]
pub type PEOUT = crate::Reg<peout::PEOUT_SPEC>;
#[doc = "desc PEOUT"]
pub mod peout;
#[doc = "PEADS register accessor: an alias for `Reg<PEADS_SPEC>`"]
pub type PEADS = crate::Reg<peads::PEADS_SPEC>;
#[doc = "desc PEADS"]
pub mod peads;
#[doc = "PEBSET register accessor: an alias for `Reg<PEBSET_SPEC>`"]
pub type PEBSET = crate::Reg<pebset::PEBSET_SPEC>;
#[doc = "desc PEBSET"]
pub mod pebset;
#[doc = "PEBCLR register accessor: an alias for `Reg<PEBCLR_SPEC>`"]
pub type PEBCLR = crate::Reg<pebclr::PEBCLR_SPEC>;
#[doc = "desc PEBCLR"]
pub mod pebclr;
#[doc = "PEBSETCLR register accessor: an alias for `Reg<PEBSETCLR_SPEC>`"]
pub type PEBSETCLR = crate::Reg<pebsetclr::PEBSETCLR_SPEC>;
#[doc = "desc PEBSETCLR"]
pub mod pebsetclr;
#[doc = "PEDR register accessor: an alias for `Reg<PEDR_SPEC>`"]
pub type PEDR = crate::Reg<pedr::PEDR_SPEC>;
#[doc = "desc PEDR"]
pub mod pedr;
#[doc = "PEPU register accessor: an alias for `Reg<PEPU_SPEC>`"]
pub type PEPU = crate::Reg<pepu::PEPU_SPEC>;
#[doc = "desc PEPU"]
pub mod pepu;
#[doc = "PEPD register accessor: an alias for `Reg<PEPD_SPEC>`"]
pub type PEPD = crate::Reg<pepd::PEPD_SPEC>;
#[doc = "desc PEPD"]
pub mod pepd;
#[doc = "PEOD register accessor: an alias for `Reg<PEOD_SPEC>`"]
pub type PEOD = crate::Reg<peod::PEOD_SPEC>;
#[doc = "desc PEOD"]
pub mod peod;
#[doc = "PEHIE register accessor: an alias for `Reg<PEHIE_SPEC>`"]
pub type PEHIE = crate::Reg<pehie::PEHIE_SPEC>;
#[doc = "desc PEHIE"]
pub mod pehie;
#[doc = "PELIE register accessor: an alias for `Reg<PELIE_SPEC>`"]
pub type PELIE = crate::Reg<pelie::PELIE_SPEC>;
#[doc = "desc PELIE"]
pub mod pelie;
#[doc = "PERIE register accessor: an alias for `Reg<PERIE_SPEC>`"]
pub type PERIE = crate::Reg<perie::PERIE_SPEC>;
#[doc = "desc PERIE"]
pub mod perie;
#[doc = "PEFIE register accessor: an alias for `Reg<PEFIE_SPEC>`"]
pub type PEFIE = crate::Reg<pefie::PEFIE_SPEC>;
#[doc = "desc PEFIE"]
pub mod pefie;
#[doc = "PFDIR register accessor: an alias for `Reg<PFDIR_SPEC>`"]
pub type PFDIR = crate::Reg<pfdir::PFDIR_SPEC>;
#[doc = "desc PFDIR"]
pub mod pfdir;
#[doc = "PFIN register accessor: an alias for `Reg<PFIN_SPEC>`"]
pub type PFIN = crate::Reg<pfin::PFIN_SPEC>;
#[doc = "desc PFIN"]
pub mod pfin;
#[doc = "PFOUT register accessor: an alias for `Reg<PFOUT_SPEC>`"]
pub type PFOUT = crate::Reg<pfout::PFOUT_SPEC>;
#[doc = "desc PFOUT"]
pub mod pfout;
#[doc = "PFADS register accessor: an alias for `Reg<PFADS_SPEC>`"]
pub type PFADS = crate::Reg<pfads::PFADS_SPEC>;
#[doc = "desc PFADS"]
pub mod pfads;
#[doc = "PFBSET register accessor: an alias for `Reg<PFBSET_SPEC>`"]
pub type PFBSET = crate::Reg<pfbset::PFBSET_SPEC>;
#[doc = "desc PFBSET"]
pub mod pfbset;
#[doc = "PFBCLR register accessor: an alias for `Reg<PFBCLR_SPEC>`"]
pub type PFBCLR = crate::Reg<pfbclr::PFBCLR_SPEC>;
#[doc = "desc PFBCLR"]
pub mod pfbclr;
#[doc = "PFBSETCLR register accessor: an alias for `Reg<PFBSETCLR_SPEC>`"]
pub type PFBSETCLR = crate::Reg<pfbsetclr::PFBSETCLR_SPEC>;
#[doc = "desc PFBSETCLR"]
pub mod pfbsetclr;
#[doc = "PFDR register accessor: an alias for `Reg<PFDR_SPEC>`"]
pub type PFDR = crate::Reg<pfdr::PFDR_SPEC>;
#[doc = "desc PFDR"]
pub mod pfdr;
#[doc = "PFPU register accessor: an alias for `Reg<PFPU_SPEC>`"]
pub type PFPU = crate::Reg<pfpu::PFPU_SPEC>;
#[doc = "desc PFPU"]
pub mod pfpu;
#[doc = "PFPD register accessor: an alias for `Reg<PFPD_SPEC>`"]
pub type PFPD = crate::Reg<pfpd::PFPD_SPEC>;
#[doc = "desc PFPD"]
pub mod pfpd;
#[doc = "PFOD register accessor: an alias for `Reg<PFOD_SPEC>`"]
pub type PFOD = crate::Reg<pfod::PFOD_SPEC>;
#[doc = "desc PFOD"]
pub mod pfod;
#[doc = "PFHIE register accessor: an alias for `Reg<PFHIE_SPEC>`"]
pub type PFHIE = crate::Reg<pfhie::PFHIE_SPEC>;
#[doc = "desc PFHIE"]
pub mod pfhie;
#[doc = "PFLIE register accessor: an alias for `Reg<PFLIE_SPEC>`"]
pub type PFLIE = crate::Reg<pflie::PFLIE_SPEC>;
#[doc = "desc PFLIE"]
pub mod pflie;
#[doc = "PFRIE register accessor: an alias for `Reg<PFRIE_SPEC>`"]
pub type PFRIE = crate::Reg<pfrie::PFRIE_SPEC>;
#[doc = "desc PFRIE"]
pub mod pfrie;
#[doc = "PFFIE register accessor: an alias for `Reg<PFFIE_SPEC>`"]
pub type PFFIE = crate::Reg<pffie::PFFIE_SPEC>;
#[doc = "desc PFFIE"]
pub mod pffie;
#[doc = "PE_STAT register accessor: an alias for `Reg<PE_STAT_SPEC>`"]
pub type PE_STAT = crate::Reg<pe_stat::PE_STAT_SPEC>;
#[doc = "desc PE_STAT"]
pub mod pe_stat;
#[doc = "PE_ICLR register accessor: an alias for `Reg<PE_ICLR_SPEC>`"]
pub type PE_ICLR = crate::Reg<pe_iclr::PE_ICLR_SPEC>;
#[doc = "desc PE_ICLR"]
pub mod pe_iclr;
#[doc = "PF_STAT register accessor: an alias for `Reg<PF_STAT_SPEC>`"]
pub type PF_STAT = crate::Reg<pf_stat::PF_STAT_SPEC>;
#[doc = "desc PF_STAT"]
pub mod pf_stat;
#[doc = "PF_ICLR register accessor: an alias for `Reg<PF_ICLR_SPEC>`"]
pub type PF_ICLR = crate::Reg<pf_iclr::PF_ICLR_SPEC>;
#[doc = "desc PF_ICLR"]
pub mod pf_iclr;
