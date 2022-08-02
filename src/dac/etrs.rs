#[doc = "Register `ETRS` reader"]
pub struct R(crate::R<ETRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETRS` writer"]
pub struct W(crate::W<ETRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ETRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTRIGSEL` reader - desc PTRIGSEL"]
pub type PTRIGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PTRIGSEL` writer - desc PTRIGSEL"]
pub type PTRIGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETRS_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETRS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 4:6 - desc PTRIGSEL"]
    #[inline(always)]
    pub fn ptrigsel(&self) -> PTRIGSEL_R {
        PTRIGSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - desc PTRIGSEL"]
    #[inline(always)]
    pub fn ptrigsel(&mut self) -> PTRIGSEL_W<4> {
        PTRIGSEL_W::new(self)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<31> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc ETRS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etrs](index.html) module"]
pub struct ETRS_SPEC;
impl crate::RegisterSpec for ETRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etrs::R](R) reader structure"]
impl crate::Readable for ETRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etrs::W](W) writer structure"]
impl crate::Writable for ETRS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETRS to value 0"]
impl crate::Resettable for ETRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
