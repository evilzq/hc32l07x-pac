#[doc = "Register `DHR12L0` reader"]
pub struct R(crate::R<DHR12L0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHR12L0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHR12L0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHR12L0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DHR12L0` writer"]
pub struct W(crate::W<DHR12L0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DHR12L0_SPEC>;
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
impl From<crate::W<DHR12L0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DHR12L0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DHR0` reader - desc DHR0"]
pub type DHR0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DHR0` writer - desc DHR0"]
pub type DHR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DHR12L0_SPEC, u16, u16, 12, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHR12L0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 4:15 - desc DHR0"]
    #[inline(always)]
    pub fn dhr0(&self) -> DHR0_R {
        DHR0_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:15 - desc DHR0"]
    #[inline(always)]
    pub fn dhr0(&mut self) -> DHR0_W<4> {
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
#[doc = "desc DHR12L0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr12l0](index.html) module"]
pub struct DHR12L0_SPEC;
impl crate::RegisterSpec for DHR12L0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dhr12l0::R](R) reader structure"]
impl crate::Readable for DHR12L0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dhr12l0::W](W) writer structure"]
impl crate::Writable for DHR12L0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DHR12L0 to value 0"]
impl crate::Resettable for DHR12L0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
