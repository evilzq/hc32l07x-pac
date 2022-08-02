#[doc = "Register `DHR8R0` reader"]
pub struct R(crate::R<DHR8R0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHR8R0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHR8R0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHR8R0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DHR8R0` writer"]
pub struct W(crate::W<DHR8R0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DHR8R0_SPEC>;
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
impl From<crate::W<DHR8R0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DHR8R0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DHR0` reader - desc DHR0"]
pub type DHR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DHR0` writer - desc DHR0"]
pub type DHR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DHR8R0_SPEC, u8, u8, 8, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHR8R0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - desc DHR0"]
    #[inline(always)]
    pub fn dhr0(&self) -> DHR0_R {
        DHR0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc DHR0"]
    #[inline(always)]
    pub fn dhr0(&mut self) -> DHR0_W<0> {
        DHR0_W::new(self)
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
#[doc = "desc DHR8R0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr8r0](index.html) module"]
pub struct DHR8R0_SPEC;
impl crate::RegisterSpec for DHR8R0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dhr8r0::R](R) reader structure"]
impl crate::Readable for DHR8R0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dhr8r0::W](W) writer structure"]
impl crate::Writable for DHR8R0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DHR8R0 to value 0"]
impl crate::Resettable for DHR8R0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
