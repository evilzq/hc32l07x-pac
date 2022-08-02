#[doc = "Register `CID` reader"]
pub struct R(crate::R<CID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "desc CID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid](index.html) module"]
pub struct CID_SPEC;
impl crate::RegisterSpec for CID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cid::R](R) reader structure"]
impl crate::Readable for CID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CID to value 0x1234_5678"]
impl crate::Resettable for CID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1234_5678
    }
}
