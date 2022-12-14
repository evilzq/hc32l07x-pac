#[doc = "Register `CALCNT` reader"]
pub struct R(crate::R<CALCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CALCNT` reader - desc CALCNT"]
pub type CALCNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - desc CALCNT"]
    #[inline(always)]
    pub fn calcnt(&self) -> CALCNT_R {
        CALCNT_R::new(self.bits)
    }
}
#[doc = "desc CALCNT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calcnt](index.html) module"]
pub struct CALCNT_SPEC;
impl crate::RegisterSpec for CALCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calcnt::R](R) reader structure"]
impl crate::Readable for CALCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CALCNT to value 0"]
impl crate::Resettable for CALCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
