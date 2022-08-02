#[doc = "Register `KEY6` reader"]
pub struct R(crate::R<KEY6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY6` writer"]
pub struct W(crate::W<KEY6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY6_SPEC>;
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
impl From<crate::W<KEY6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY0` reader - desc KEY0"]
pub type KEY0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEY0` writer - desc KEY0"]
pub type KEY0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEY6_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - desc KEY0"]
    #[inline(always)]
    pub fn key0(&self) -> KEY0_R {
        KEY0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - desc KEY0"]
    #[inline(always)]
    pub fn key0(&mut self) -> KEY0_W<0> {
        KEY0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc KEY6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key6](index.html) module"]
pub struct KEY6_SPEC;
impl crate::RegisterSpec for KEY6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key6::R](R) reader structure"]
impl crate::Readable for KEY6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key6::W](W) writer structure"]
impl crate::Writable for KEY6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY6 to value 0"]
impl crate::Resettable for KEY6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
