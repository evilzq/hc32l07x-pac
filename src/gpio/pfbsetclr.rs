#[doc = "Register `PFBSETCLR` reader"]
pub struct R(crate::R<PFBSETCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFBSETCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFBSETCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFBSETCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFBSETCLR` writer"]
pub struct W(crate::W<PFBSETCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFBSETCLR_SPEC>;
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
impl From<crate::W<PFBSETCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFBSETCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFBCLR` reader - desc PFBCLR"]
pub type PFBCLR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PFBCLR` writer - desc PFBCLR"]
pub type PFBCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PFBSETCLR_SPEC, u16, u16, 12, O>;
#[doc = "Field `PFBSET` reader - desc PFBSET"]
pub type PFBSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PFBSET` writer - desc PFBSET"]
pub type PFBSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PFBSETCLR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - desc PFBCLR"]
    #[inline(always)]
    pub fn pfbclr(&self) -> PFBCLR_R {
        PFBCLR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - desc PFBSET"]
    #[inline(always)]
    pub fn pfbset(&self) -> PFBSET_R {
        PFBSET_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc PFBCLR"]
    #[inline(always)]
    pub fn pfbclr(&mut self) -> PFBCLR_W<0> {
        PFBCLR_W::new(self)
    }
    #[doc = "Bits 16:27 - desc PFBSET"]
    #[inline(always)]
    pub fn pfbset(&mut self) -> PFBSET_W<16> {
        PFBSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc PFBSETCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfbsetclr](index.html) module"]
pub struct PFBSETCLR_SPEC;
impl crate::RegisterSpec for PFBSETCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfbsetclr::R](R) reader structure"]
impl crate::Readable for PFBSETCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfbsetclr::W](W) writer structure"]
impl crate::Writable for PFBSETCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PFBSETCLR to value 0"]
impl crate::Resettable for PFBSETCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
