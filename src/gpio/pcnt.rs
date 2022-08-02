#[doc = "Register `PCNT` reader"]
pub struct R(crate::R<PCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCNT` writer"]
pub struct W(crate::W<PCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCNT_SPEC>;
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
impl From<crate::W<PCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCNT_S0` reader - desc PCNT_S0"]
pub type PCNT_S0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCNT_S0` writer - desc PCNT_S0"]
pub type PCNT_S0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCNT_SPEC, u8, u8, 2, O>;
#[doc = "Field `PCNT_S1` reader - desc PCNT_S1"]
pub type PCNT_S1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCNT_S1` writer - desc PCNT_S1"]
pub type PCNT_S1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCNT_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - desc PCNT_S0"]
    #[inline(always)]
    pub fn pcnt_s0(&self) -> PCNT_S0_R {
        PCNT_S0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc PCNT_S1"]
    #[inline(always)]
    pub fn pcnt_s1(&self) -> PCNT_S1_R {
        PCNT_S1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc PCNT_S0"]
    #[inline(always)]
    pub fn pcnt_s0(&mut self) -> PCNT_S0_W<0> {
        PCNT_S0_W::new(self)
    }
    #[doc = "Bits 2:3 - desc PCNT_S1"]
    #[inline(always)]
    pub fn pcnt_s1(&mut self) -> PCNT_S1_W<2> {
        PCNT_S1_W::new(self)
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
#[doc = "desc PCNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcnt](index.html) module"]
pub struct PCNT_SPEC;
impl crate::RegisterSpec for PCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcnt::R](R) reader structure"]
impl crate::Readable for PCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcnt::W](W) writer structure"]
impl crate::Writable for PCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCNT to value 0"]
impl crate::Resettable for PCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
