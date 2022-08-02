#[doc = "Register `RTIF` reader"]
pub struct R(crate::R<RTIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTIF` writer"]
pub struct W(crate::W<RTIF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTIF_SPEC>;
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
impl From<crate::W<RTIF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTIF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AIF` reader - desc AIF"]
pub type AIF_R = crate::BitReader<bool>;
#[doc = "Field `AIF` writer - desc AIF"]
pub type AIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
#[doc = "Field `EIF` reader - desc EIF"]
pub type EIF_R = crate::BitReader<bool>;
#[doc = "Field `EIF` writer - desc EIF"]
pub type EIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
#[doc = "Field `TSIF` reader - desc TSIF"]
pub type TSIF_R = crate::BitReader<bool>;
#[doc = "Field `TSIF` writer - desc TSIF"]
pub type TSIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
#[doc = "Field `TPIF` reader - desc TPIF"]
pub type TPIF_R = crate::BitReader<bool>;
#[doc = "Field `TPIF` writer - desc TPIF"]
pub type TPIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
#[doc = "Field `RAFIF` reader - desc RAFIF"]
pub type RAFIF_R = crate::BitReader<bool>;
#[doc = "Field `RAFIF` writer - desc RAFIF"]
pub type RAFIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
#[doc = "Field `RFIF` reader - desc RFIF"]
pub type RFIF_R = crate::BitReader<bool>;
#[doc = "Field `RFIF` writer - desc RFIF"]
pub type RFIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
#[doc = "Field `ROIF` reader - desc ROIF"]
pub type ROIF_R = crate::BitReader<bool>;
#[doc = "Field `ROIF` writer - desc ROIF"]
pub type ROIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
#[doc = "Field `RIF` reader - desc RIF"]
pub type RIF_R = crate::BitReader<bool>;
#[doc = "Field `RIF` writer - desc RIF"]
pub type RIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc AIF"]
    #[inline(always)]
    pub fn aif(&self) -> AIF_R {
        AIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EIF"]
    #[inline(always)]
    pub fn eif(&self) -> EIF_R {
        EIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TSIF"]
    #[inline(always)]
    pub fn tsif(&self) -> TSIF_R {
        TSIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TPIF"]
    #[inline(always)]
    pub fn tpif(&self) -> TPIF_R {
        TPIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RAFIF"]
    #[inline(always)]
    pub fn rafif(&self) -> RAFIF_R {
        RAFIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RFIF"]
    #[inline(always)]
    pub fn rfif(&self) -> RFIF_R {
        RFIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ROIF"]
    #[inline(always)]
    pub fn roif(&self) -> ROIF_R {
        ROIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RIF"]
    #[inline(always)]
    pub fn rif(&self) -> RIF_R {
        RIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc AIF"]
    #[inline(always)]
    pub fn aif(&mut self) -> AIF_W<0> {
        AIF_W::new(self)
    }
    #[doc = "Bit 1 - desc EIF"]
    #[inline(always)]
    pub fn eif(&mut self) -> EIF_W<1> {
        EIF_W::new(self)
    }
    #[doc = "Bit 2 - desc TSIF"]
    #[inline(always)]
    pub fn tsif(&mut self) -> TSIF_W<2> {
        TSIF_W::new(self)
    }
    #[doc = "Bit 3 - desc TPIF"]
    #[inline(always)]
    pub fn tpif(&mut self) -> TPIF_W<3> {
        TPIF_W::new(self)
    }
    #[doc = "Bit 4 - desc RAFIF"]
    #[inline(always)]
    pub fn rafif(&mut self) -> RAFIF_W<4> {
        RAFIF_W::new(self)
    }
    #[doc = "Bit 5 - desc RFIF"]
    #[inline(always)]
    pub fn rfif(&mut self) -> RFIF_W<5> {
        RFIF_W::new(self)
    }
    #[doc = "Bit 6 - desc ROIF"]
    #[inline(always)]
    pub fn roif(&mut self) -> ROIF_W<6> {
        ROIF_W::new(self)
    }
    #[doc = "Bit 7 - desc RIF"]
    #[inline(always)]
    pub fn rif(&mut self) -> RIF_W<7> {
        RIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc RTIF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtif](index.html) module"]
pub struct RTIF_SPEC;
impl crate::RegisterSpec for RTIF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtif::R](R) reader structure"]
impl crate::Readable for RTIF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtif::W](W) writer structure"]
impl crate::Writable for RTIF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTIF to value 0"]
impl crate::Resettable for RTIF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
