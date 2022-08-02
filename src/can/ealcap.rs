#[doc = "Register `EALCAP` reader"]
pub struct R(crate::R<EALCAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EALCAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EALCAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EALCAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ALC` reader - desc ALC"]
pub type ALC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KOER` reader - desc KOER"]
pub type KOER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - desc ALC"]
    #[inline(always)]
    pub fn alc(&self) -> ALC_R {
        ALC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - desc KOER"]
    #[inline(always)]
    pub fn koer(&self) -> KOER_R {
        KOER_R::new(((self.bits >> 5) & 7) as u8)
    }
}
#[doc = "desc EALCAP\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ealcap](index.html) module"]
pub struct EALCAP_SPEC;
impl crate::RegisterSpec for EALCAP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ealcap::R](R) reader structure"]
impl crate::Readable for EALCAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EALCAP to value 0"]
impl crate::Resettable for EALCAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
