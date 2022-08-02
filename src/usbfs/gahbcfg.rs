#[doc = "Register `GAHBCFG` reader"]
pub struct R(crate::R<GAHBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAHBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAHBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAHBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAHBCFG` writer"]
pub struct W(crate::W<GAHBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAHBCFG_SPEC>;
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
impl From<crate::W<GAHBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAHBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GINTMSK` reader - desc GINTMSK"]
pub type GINTMSK_R = crate::BitReader<bool>;
#[doc = "Field `GINTMSK` writer - desc GINTMSK"]
pub type GINTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
#[doc = "Field `HBSTLEN` reader - desc HBSTLEN"]
pub type HBSTLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HBSTLEN` writer - desc HBSTLEN"]
pub type HBSTLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAHBCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DMAEN` reader - desc DMAEN"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - desc DMAEN"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc GINTMSK"]
    #[inline(always)]
    pub fn gintmsk(&self) -> GINTMSK_R {
        GINTMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - desc HBSTLEN"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc GINTMSK"]
    #[inline(always)]
    pub fn gintmsk(&mut self) -> GINTMSK_W<0> {
        GINTMSK_W::new(self)
    }
    #[doc = "Bits 1:4 - desc HBSTLEN"]
    #[inline(always)]
    pub fn hbstlen(&mut self) -> HBSTLEN_W<1> {
        HBSTLEN_W::new(self)
    }
    #[doc = "Bit 5 - desc DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<5> {
        DMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc GAHBCFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gahbcfg](index.html) module"]
pub struct GAHBCFG_SPEC;
impl crate::RegisterSpec for GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gahbcfg::R](R) reader structure"]
impl crate::Readable for GAHBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gahbcfg::W](W) writer structure"]
impl crate::Writable for GAHBCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GAHBCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
