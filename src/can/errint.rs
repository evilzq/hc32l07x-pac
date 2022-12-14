#[doc = "Register `ERRINT` reader"]
pub struct R(crate::R<ERRINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRINT` writer"]
pub struct W(crate::W<ERRINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRINT_SPEC>;
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
impl From<crate::W<ERRINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BEIF` reader - desc BEIF"]
pub type BEIF_R = crate::BitReader<bool>;
#[doc = "Field `BEIF` writer - desc BEIF"]
pub type BEIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRINT_SPEC, bool, O>;
#[doc = "Field `BEIE` reader - desc BEIE"]
pub type BEIE_R = crate::BitReader<bool>;
#[doc = "Field `BEIE` writer - desc BEIE"]
pub type BEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRINT_SPEC, bool, O>;
#[doc = "Field `ALIF` reader - desc ALIF"]
pub type ALIF_R = crate::BitReader<bool>;
#[doc = "Field `ALIF` writer - desc ALIF"]
pub type ALIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRINT_SPEC, bool, O>;
#[doc = "Field `ALIE` reader - desc ALIE"]
pub type ALIE_R = crate::BitReader<bool>;
#[doc = "Field `ALIE` writer - desc ALIE"]
pub type ALIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRINT_SPEC, bool, O>;
#[doc = "Field `EPIF` reader - desc EPIF"]
pub type EPIF_R = crate::BitReader<bool>;
#[doc = "Field `EPIF` writer - desc EPIF"]
pub type EPIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRINT_SPEC, bool, O>;
#[doc = "Field `EPIE` reader - desc EPIE"]
pub type EPIE_R = crate::BitReader<bool>;
#[doc = "Field `EPIE` writer - desc EPIE"]
pub type EPIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRINT_SPEC, bool, O>;
#[doc = "Field `EPASS` reader - desc EPASS"]
pub type EPASS_R = crate::BitReader<bool>;
#[doc = "Field `EWARN` reader - desc EWARN"]
pub type EWARN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc BEIF"]
    #[inline(always)]
    pub fn beif(&self) -> BEIF_R {
        BEIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BEIE"]
    #[inline(always)]
    pub fn beie(&self) -> BEIE_R {
        BEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ALIF"]
    #[inline(always)]
    pub fn alif(&self) -> ALIF_R {
        ALIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc ALIE"]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc EPIF"]
    #[inline(always)]
    pub fn epif(&self) -> EPIF_R {
        EPIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc EPIE"]
    #[inline(always)]
    pub fn epie(&self) -> EPIE_R {
        EPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc EPASS"]
    #[inline(always)]
    pub fn epass(&self) -> EPASS_R {
        EPASS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc EWARN"]
    #[inline(always)]
    pub fn ewarn(&self) -> EWARN_R {
        EWARN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BEIF"]
    #[inline(always)]
    pub fn beif(&mut self) -> BEIF_W<0> {
        BEIF_W::new(self)
    }
    #[doc = "Bit 1 - desc BEIE"]
    #[inline(always)]
    pub fn beie(&mut self) -> BEIE_W<1> {
        BEIE_W::new(self)
    }
    #[doc = "Bit 2 - desc ALIF"]
    #[inline(always)]
    pub fn alif(&mut self) -> ALIF_W<2> {
        ALIF_W::new(self)
    }
    #[doc = "Bit 3 - desc ALIE"]
    #[inline(always)]
    pub fn alie(&mut self) -> ALIE_W<3> {
        ALIE_W::new(self)
    }
    #[doc = "Bit 4 - desc EPIF"]
    #[inline(always)]
    pub fn epif(&mut self) -> EPIF_W<4> {
        EPIF_W::new(self)
    }
    #[doc = "Bit 5 - desc EPIE"]
    #[inline(always)]
    pub fn epie(&mut self) -> EPIE_W<5> {
        EPIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc ERRINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errint](index.html) module"]
pub struct ERRINT_SPEC;
impl crate::RegisterSpec for ERRINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [errint::R](R) reader structure"]
impl crate::Readable for ERRINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errint::W](W) writer structure"]
impl crate::Writable for ERRINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERRINT to value 0"]
impl crate::Resettable for ERRINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
