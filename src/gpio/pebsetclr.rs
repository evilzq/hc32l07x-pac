#[doc = "Register `PEBSETCLR` reader"]
pub struct R(crate::R<PEBSETCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEBSETCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEBSETCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEBSETCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEBSETCLR` writer"]
pub struct W(crate::W<PEBSETCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEBSETCLR_SPEC>;
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
impl From<crate::W<PEBSETCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEBSETCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEBCLR` reader - desc PEBCLR"]
pub type PEBCLR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PEBCLR` writer - desc PEBCLR"]
pub type PEBCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PEBSETCLR_SPEC, u16, u16, 16, O>;
#[doc = "Field `PEBSET` reader - desc PEBSET"]
pub type PEBSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PEBSET` writer - desc PEBSET"]
pub type PEBSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PEBSETCLR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - desc PEBCLR"]
    #[inline(always)]
    pub fn pebclr(&self) -> PEBCLR_R {
        PEBCLR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - desc PEBSET"]
    #[inline(always)]
    pub fn pebset(&self) -> PEBSET_R {
        PEBSET_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc PEBCLR"]
    #[inline(always)]
    pub fn pebclr(&mut self) -> PEBCLR_W<0> {
        PEBCLR_W::new(self)
    }
    #[doc = "Bits 16:31 - desc PEBSET"]
    #[inline(always)]
    pub fn pebset(&mut self) -> PEBSET_W<16> {
        PEBSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc PEBSETCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pebsetclr](index.html) module"]
pub struct PEBSETCLR_SPEC;
impl crate::RegisterSpec for PEBSETCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pebsetclr::R](R) reader structure"]
impl crate::Readable for PEBSETCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pebsetclr::W](W) writer structure"]
impl crate::Writable for PEBSETCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEBSETCLR to value 0"]
impl crate::Resettable for PEBSETCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
