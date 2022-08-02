#[doc = "Register `DAINTMSK` reader"]
pub struct R(crate::R<DAINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IEPINTM` reader - desc IEPINTM"]
pub type IEPINTM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OEPINTM` reader - desc OEPINTM"]
pub type OEPINTM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc IEPINTM"]
    #[inline(always)]
    pub fn iepintm(&self) -> IEPINTM_R {
        IEPINTM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - desc OEPINTM"]
    #[inline(always)]
    pub fn oepintm(&self) -> OEPINTM_R {
        OEPINTM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "desc DAINTMSK\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daintmsk](index.html) module"]
pub struct DAINTMSK_SPEC;
impl crate::RegisterSpec for DAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daintmsk::R](R) reader structure"]
impl crate::Readable for DAINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DAINTMSK to value 0"]
impl crate::Resettable for DAINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
