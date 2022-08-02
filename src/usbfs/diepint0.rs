#[doc = "Register `DIEPINT0` reader"]
pub struct R(crate::R<DIEPINT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPINT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPINT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPINT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPINT0` writer"]
pub struct W(crate::W<DIEPINT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPINT0_SPEC>;
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
impl From<crate::W<DIEPINT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPINT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRC` reader - desc XFRC"]
pub type XFRC_R = crate::BitReader<bool>;
#[doc = "Field `XFRC` writer - desc XFRC"]
pub type XFRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT0_SPEC, bool, O>;
#[doc = "Field `EPDISD` reader - desc EPDISD"]
pub type EPDISD_R = crate::BitReader<bool>;
#[doc = "Field `EPDISD` writer - desc EPDISD"]
pub type EPDISD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT0_SPEC, bool, O>;
#[doc = "Field `TOC` reader - desc TOC"]
pub type TOC_R = crate::BitReader<bool>;
#[doc = "Field `TOC` writer - desc TOC"]
pub type TOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT0_SPEC, bool, O>;
#[doc = "Field `TTXFE` reader - desc TTXFE"]
pub type TTXFE_R = crate::BitReader<bool>;
#[doc = "Field `TTXFE` writer - desc TTXFE"]
pub type TTXFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT0_SPEC, bool, O>;
#[doc = "Field `NEPNE` reader - desc NEPNE"]
pub type NEPNE_R = crate::BitReader<bool>;
#[doc = "Field `NEPNE` writer - desc NEPNE"]
pub type NEPNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT0_SPEC, bool, O>;
#[doc = "Field `TXFE` reader - desc TXFE"]
pub type TXFE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TOC"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TTXFE"]
    #[inline(always)]
    pub fn ttxfe(&self) -> TTXFE_R {
        TTXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc NEPNE"]
    #[inline(always)]
    pub fn nepne(&self) -> NEPNE_R {
        NEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TXFE"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc XFRC"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<0> {
        XFRC_W::new(self)
    }
    #[doc = "Bit 1 - desc EPDISD"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<1> {
        EPDISD_W::new(self)
    }
    #[doc = "Bit 3 - desc TOC"]
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W<3> {
        TOC_W::new(self)
    }
    #[doc = "Bit 4 - desc TTXFE"]
    #[inline(always)]
    pub fn ttxfe(&mut self) -> TTXFE_W<4> {
        TTXFE_W::new(self)
    }
    #[doc = "Bit 6 - desc NEPNE"]
    #[inline(always)]
    pub fn nepne(&mut self) -> NEPNE_W<6> {
        NEPNE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc DIEPINT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint0](index.html) module"]
pub struct DIEPINT0_SPEC;
impl crate::RegisterSpec for DIEPINT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepint0::R](R) reader structure"]
impl crate::Readable for DIEPINT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepint0::W](W) writer structure"]
impl crate::Writable for DIEPINT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPINT0 to value 0x80"]
impl crate::Resettable for DIEPINT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
