#[doc = "Register `DOR0` reader"]
pub struct R(crate::R<DOR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DOR0` reader - desc DOR0"]
pub type DOR0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:11 - desc DOR0"]
    #[inline(always)]
    pub fn dor0(&self) -> DOR0_R {
        DOR0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc DOR0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dor0](index.html) module"]
pub struct DOR0_SPEC;
impl crate::RegisterSpec for DOR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dor0::R](R) reader structure"]
impl crate::Readable for DOR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DOR0 to value 0"]
impl crate::Resettable for DOR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
