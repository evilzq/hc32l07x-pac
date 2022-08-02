#[doc = "Register `LIMIT` reader"]
pub struct R(crate::R<LIMIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIMIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIMIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIMIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIMIT` writer"]
pub struct W(crate::W<LIMIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIMIT_SPEC>;
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
impl From<crate::W<LIMIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIMIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWL` reader - desc EWL"]
pub type EWL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EWL` writer - desc EWL"]
pub type EWL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LIMIT_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFWL` reader - desc AFWL"]
pub type AFWL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFWL` writer - desc AFWL"]
pub type AFWL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LIMIT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - desc EWL"]
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc AFWL"]
    #[inline(always)]
    pub fn afwl(&self) -> AFWL_R {
        AFWL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc EWL"]
    #[inline(always)]
    pub fn ewl(&mut self) -> EWL_W<0> {
        EWL_W::new(self)
    }
    #[doc = "Bits 4:7 - desc AFWL"]
    #[inline(always)]
    pub fn afwl(&mut self) -> AFWL_W<4> {
        AFWL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc LIMIT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limit](index.html) module"]
pub struct LIMIT_SPEC;
impl crate::RegisterSpec for LIMIT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [limit::R](R) reader structure"]
impl crate::Readable for LIMIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [limit::W](W) writer structure"]
impl crate::Writable for LIMIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIMIT to value 0x1b"]
impl crate::Resettable for LIMIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1b
    }
}
