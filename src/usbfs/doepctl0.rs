#[doc = "Register `DOEPCTL0` reader"]
pub struct R(crate::R<DOEPCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MPSIZ` reader - desc MPSIZ"]
pub type MPSIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBAEP` reader - desc USBAEP"]
pub type USBAEP_R = crate::BitReader<bool>;
#[doc = "Field `NAKSTS` reader - desc NAKSTS"]
pub type NAKSTS_R = crate::BitReader<bool>;
#[doc = "Field `EPTYP` reader - desc EPTYP"]
pub type EPTYP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNPM` reader - desc SNPM"]
pub type SNPM_R = crate::BitReader<bool>;
#[doc = "Field `STALL` reader - desc STALL"]
pub type STALL_R = crate::BitReader<bool>;
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
    #[doc = "Bit 20 - desc SNPM"]
    #[inline(always)]
    pub fn snpm(&self) -> SNPM_R {
        SNPM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc STALL"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
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
#[doc = "desc DOEPCTL0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl0](index.html) module"]
pub struct DOEPCTL0_SPEC;
impl crate::RegisterSpec for DOEPCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepctl0::R](R) reader structure"]
impl crate::Readable for DOEPCTL0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DOEPCTL0 to value 0x8000"]
impl crate::Resettable for DOEPCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
