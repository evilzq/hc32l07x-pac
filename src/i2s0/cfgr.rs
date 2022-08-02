#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHIEN` reader - desc CHIEN"]
pub type CHIEN_R = crate::BitReader<bool>;
#[doc = "Field `CHIEN` writer - desc CHIEN"]
pub type CHIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `DATLEN` reader - desc DATLEN"]
pub type DATLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATLEN` writer - desc DATLEN"]
pub type DATLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CKPOL` reader - desc CKPOL"]
pub type CKPOL_R = crate::BitReader<bool>;
#[doc = "Field `CKPOL` writer - desc CKPOL"]
pub type CKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `STD` reader - desc STD"]
pub type STD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STD` writer - desc STD"]
pub type STD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CKSEL` reader - desc CKSEL"]
pub type CKSEL_R = crate::BitReader<bool>;
#[doc = "Field `CKSEL` writer - desc CKSEL"]
pub type CKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `PCMSYNC` reader - desc PCMSYNC"]
pub type PCMSYNC_R = crate::BitReader<bool>;
#[doc = "Field `PCMSYNC` writer - desc PCMSYNC"]
pub type PCMSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `CFG` reader - desc CFG"]
pub type CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG` writer - desc CFG"]
pub type CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `E` reader - desc E"]
pub type E_R = crate::BitReader<bool>;
#[doc = "Field `E` writer - desc E"]
pub type E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc CHIEN"]
    #[inline(always)]
    pub fn chien(&self) -> CHIEN_R {
        CHIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - desc DATLEN"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - desc CKPOL"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc STD"]
    #[inline(always)]
    pub fn std(&self) -> STD_R {
        STD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc CKSEL"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PCMSYNC"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - desc CFG"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - desc E"]
    #[inline(always)]
    pub fn e(&self) -> E_R {
        E_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc CHIEN"]
    #[inline(always)]
    pub fn chien(&mut self) -> CHIEN_W<0> {
        CHIEN_W::new(self)
    }
    #[doc = "Bits 1:2 - desc DATLEN"]
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W<1> {
        DATLEN_W::new(self)
    }
    #[doc = "Bit 3 - desc CKPOL"]
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W<3> {
        CKPOL_W::new(self)
    }
    #[doc = "Bits 4:5 - desc STD"]
    #[inline(always)]
    pub fn std(&mut self) -> STD_W<4> {
        STD_W::new(self)
    }
    #[doc = "Bit 6 - desc CKSEL"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W<6> {
        CKSEL_W::new(self)
    }
    #[doc = "Bit 7 - desc PCMSYNC"]
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<7> {
        PCMSYNC_W::new(self)
    }
    #[doc = "Bits 8:9 - desc CFG"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CFG_W<8> {
        CFG_W::new(self)
    }
    #[doc = "Bit 10 - desc E"]
    #[inline(always)]
    pub fn e(&mut self) -> E_W<10> {
        E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
