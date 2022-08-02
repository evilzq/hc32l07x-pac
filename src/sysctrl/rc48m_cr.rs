#[doc = "Register `RC48M_CR` reader"]
pub struct R(crate::R<RC48M_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC48M_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC48M_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC48M_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RC48M_CR` writer"]
pub struct W(crate::W<RC48M_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC48M_CR_SPEC>;
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
impl From<crate::W<RC48M_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC48M_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM` reader - desc TRIM"]
pub type TRIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRIM` writer - desc TRIM"]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RC48M_CR_SPEC, u16, u16, 10, O>;
#[doc = "Field `STABLE` reader - desc STABLE"]
pub type STABLE_R = crate::BitReader<bool>;
#[doc = "Field `EN` reader - desc EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - desc EN"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC48M_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 11 - desc STABLE"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<0> {
        TRIM_W::new(self)
    }
    #[doc = "Bit 12 - desc EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<12> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc RC48M_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc48m_cr](index.html) module"]
pub struct RC48M_CR_SPEC;
impl crate::RegisterSpec for RC48M_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc48m_cr::R](R) reader structure"]
impl crate::Readable for RC48M_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc48m_cr::W](W) writer structure"]
impl crate::Writable for RC48M_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RC48M_CR to value 0x0280"]
impl crate::Resettable for RC48M_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0280
    }
}
