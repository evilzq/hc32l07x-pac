#[doc = "Register `RTIE` reader"]
pub struct R(crate::R<RTIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTIE` writer"]
pub struct W(crate::W<RTIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTIE_SPEC>;
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
impl From<crate::W<RTIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSFF` reader - desc TSFF"]
pub type TSFF_R = crate::BitReader<bool>;
#[doc = "Field `EIE` reader - desc EIE"]
pub type EIE_R = crate::BitReader<bool>;
#[doc = "Field `EIE` writer - desc EIE"]
pub type EIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIE_SPEC, bool, O>;
#[doc = "Field `TSIE` reader - desc TSIE"]
pub type TSIE_R = crate::BitReader<bool>;
#[doc = "Field `TSIE` writer - desc TSIE"]
pub type TSIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIE_SPEC, bool, O>;
#[doc = "Field `TPIE` reader - desc TPIE"]
pub type TPIE_R = crate::BitReader<bool>;
#[doc = "Field `TPIE` writer - desc TPIE"]
pub type TPIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIE_SPEC, bool, O>;
#[doc = "Field `RAFIE` reader - desc RAFIE"]
pub type RAFIE_R = crate::BitReader<bool>;
#[doc = "Field `RAFIE` writer - desc RAFIE"]
pub type RAFIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIE_SPEC, bool, O>;
#[doc = "Field `RFIE` reader - desc RFIE"]
pub type RFIE_R = crate::BitReader<bool>;
#[doc = "Field `RFIE` writer - desc RFIE"]
pub type RFIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIE_SPEC, bool, O>;
#[doc = "Field `ROIE` reader - desc ROIE"]
pub type ROIE_R = crate::BitReader<bool>;
#[doc = "Field `ROIE` writer - desc ROIE"]
pub type ROIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIE_SPEC, bool, O>;
#[doc = "Field `RIE` reader - desc RIE"]
pub type RIE_R = crate::BitReader<bool>;
#[doc = "Field `RIE` writer - desc RIE"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc TSFF"]
    #[inline(always)]
    pub fn tsff(&self) -> TSFF_R {
        TSFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EIE"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TSIE"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TPIE"]
    #[inline(always)]
    pub fn tpie(&self) -> TPIE_R {
        TPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RAFIE"]
    #[inline(always)]
    pub fn rafie(&self) -> RAFIE_R {
        RAFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RFIE"]
    #[inline(always)]
    pub fn rfie(&self) -> RFIE_R {
        RFIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ROIE"]
    #[inline(always)]
    pub fn roie(&self) -> ROIE_R {
        ROIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RIE"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc EIE"]
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<1> {
        EIE_W::new(self)
    }
    #[doc = "Bit 2 - desc TSIE"]
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W<2> {
        TSIE_W::new(self)
    }
    #[doc = "Bit 3 - desc TPIE"]
    #[inline(always)]
    pub fn tpie(&mut self) -> TPIE_W<3> {
        TPIE_W::new(self)
    }
    #[doc = "Bit 4 - desc RAFIE"]
    #[inline(always)]
    pub fn rafie(&mut self) -> RAFIE_W<4> {
        RAFIE_W::new(self)
    }
    #[doc = "Bit 5 - desc RFIE"]
    #[inline(always)]
    pub fn rfie(&mut self) -> RFIE_W<5> {
        RFIE_W::new(self)
    }
    #[doc = "Bit 6 - desc ROIE"]
    #[inline(always)]
    pub fn roie(&mut self) -> ROIE_W<6> {
        ROIE_W::new(self)
    }
    #[doc = "Bit 7 - desc RIE"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<7> {
        RIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc RTIE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtie](index.html) module"]
pub struct RTIE_SPEC;
impl crate::RegisterSpec for RTIE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtie::R](R) reader structure"]
impl crate::Readable for RTIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtie::W](W) writer structure"]
impl crate::Writable for RTIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTIE to value 0xfe"]
impl crate::Resettable for RTIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfe
    }
}
