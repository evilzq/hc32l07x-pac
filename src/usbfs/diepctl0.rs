#[doc = "Register `DIEPCTL0` reader"]
pub struct R(crate::R<DIEPCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MPSIZ` reader - desc MPSIZ"]
pub type MPSIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBAEP` reader - desc USBAEP"]
pub type USBAEP_R = crate::BitReader<bool>;
#[doc = "Field `EONUM_DPID` reader - desc EONUM_DPID"]
pub type EONUM_DPID_R = crate::BitReader<bool>;
#[doc = "Field `NAKSTS` reader - desc NAKSTS"]
pub type NAKSTS_R = crate::BitReader<bool>;
#[doc = "Field `EPTYP` reader - desc EPTYP"]
pub type EPTYP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STALL` reader - desc STALL"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `TXFNUM` reader - desc TXFNUM"]
pub type TXFNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SD0PID_SEVNFRM` reader - desc SD0PID_SEVNFRM"]
pub type SD0PID_SEVNFRM_R = crate::BitReader<bool>;
#[doc = "Field `SODDFRM` reader - desc SODDFRM"]
pub type SODDFRM_R = crate::BitReader<bool>;
#[doc = "Field `EPDIS` reader - desc EPDIS"]
pub type EPDIS_R = crate::BitReader<bool>;
#[doc = "Field `EPENA` reader - desc EPENA"]
pub type EPENA_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - desc MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - desc USBAEP"]
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc EONUM_DPID"]
    #[inline(always)]
    pub fn eonum_dpid(&self) -> EONUM_DPID_R {
        EONUM_DPID_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc NAKSTS"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - desc EPTYP"]
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - desc STALL"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - desc TXFNUM"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - desc SD0PID_SEVNFRM"]
    #[inline(always)]
    pub fn sd0pid_sevnfrm(&self) -> SD0PID_SEVNFRM_R {
        SD0PID_SEVNFRM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc SODDFRM"]
    #[inline(always)]
    pub fn soddfrm(&self) -> SODDFRM_R {
        SODDFRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc EPDIS"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc EPENA"]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc DIEPCTL0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl0](index.html) module"]
pub struct DIEPCTL0_SPEC;
impl crate::RegisterSpec for DIEPCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepctl0::R](R) reader structure"]
impl crate::Readable for DIEPCTL0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIEPCTL0 to value 0x8000"]
impl crate::Resettable for DIEPCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
