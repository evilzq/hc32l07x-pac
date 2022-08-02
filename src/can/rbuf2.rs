#[doc = "Register `RBUF2` reader"]
pub struct R(crate::R<RBUF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBUF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBUF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBUF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "desc RBUF2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbuf2](index.html) module"]
pub struct RBUF2_SPEC;
impl crate::RegisterSpec for RBUF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbuf2::R](R) reader structure"]
impl crate::Readable for RBUF2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RBUF2 to value 0"]
impl crate::Resettable for RBUF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
