#[doc = "Register `OVCK_CR` reader"]
pub struct R(crate::R<OVCK_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVCK_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVCK_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVCK_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OVCK_CR` writer"]
pub struct W(crate::W<OVCK_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVCK_CR_SPEC>;
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
impl From<crate::W<OVCK_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVCK_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVCK` reader - desc OVCK"]
pub type OVCK_R = crate::BitReader<bool>;
#[doc = "Field `OVCK` writer - desc OVCK"]
pub type OVCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OVCK_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc OVCK"]
    #[inline(always)]
    pub fn ovck(&self) -> OVCK_R {
        OVCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc OVCK"]
    #[inline(always)]
    pub fn ovck(&mut self) -> OVCK_W<0> {
        OVCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc OVCK_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovck_cr](index.html) module"]
pub struct OVCK_CR_SPEC;
impl crate::RegisterSpec for OVCK_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ovck_cr::R](R) reader structure"]
impl crate::Readable for OVCK_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ovck_cr::W](W) writer structure"]
impl crate::Writable for OVCK_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OVCK_CR to value 0"]
impl crate::Resettable for OVCK_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
