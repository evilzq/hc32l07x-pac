#[doc = "Register `DHR12R1` reader"]
pub struct R(crate::R<DHR12R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHR12R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHR12R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHR12R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DHR12R1` writer"]
pub struct W(crate::W<DHR12R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DHR12R1_SPEC>;
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
impl From<crate::W<DHR12R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DHR12R1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DHR1` reader - desc DHR1"]
pub type DHR1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DHR1` writer - desc DHR1"]
pub type DHR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DHR12R1_SPEC, u16, u16, 12, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DHR12R1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - desc DHR1"]
    #[inline(always)]
    pub fn dhr1(&self) -> DHR1_R {
        DHR1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc DHR1"]
    #[inline(always)]
    pub fn dhr1(&mut self) -> DHR1_W<0> {
        DHR1_W::new(self)
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
#[doc = "desc DHR12R1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhr12r1](index.html) module"]
pub struct DHR12R1_SPEC;
impl crate::RegisterSpec for DHR12R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dhr12r1::R](R) reader structure"]
impl crate::Readable for DHR12R1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dhr12r1::W](W) writer structure"]
impl crate::Writable for DHR12R1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DHR12R1 to value 0"]
impl crate::Resettable for DHR12R1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
