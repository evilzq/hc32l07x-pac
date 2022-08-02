#[doc = "Register `GRXSTSP` reader"]
pub struct R(crate::R<GRXSTSP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRXSTSP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRXSTSP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRXSTSP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHNUM_EPNUM` reader - desc CHNUM_EPNUM"]
pub type CHNUM_EPNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCNT` reader - desc BCNT"]
pub type BCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DPID` reader - desc DPID"]
pub type DPID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PKTSTS` reader - desc PKTSTS"]
pub type PKTSTS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - desc CHNUM_EPNUM"]
    #[inline(always)]
    pub fn chnum_epnum(&self) -> CHNUM_EPNUM_R {
        CHNUM_EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - desc BCNT"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - desc DPID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - desc PKTSTS"]
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
#[doc = "desc GRXSTSP\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxstsp](index.html) module"]
pub struct GRXSTSP_SPEC;
impl crate::RegisterSpec for GRXSTSP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grxstsp::R](R) reader structure"]
impl crate::Readable for GRXSTSP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GRXSTSP to value 0"]
impl crate::Resettable for GRXSTSP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
