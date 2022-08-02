#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OKF` reader - desc OKF"]
pub type OKF_R = crate::BitReader<bool>;
#[doc = "Field `OKF` writer - desc OKF"]
pub type OKF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `WARNF` reader - desc WARNF"]
pub type WARNF_R = crate::BitReader<bool>;
#[doc = "Field `WARNF` writer - desc WARNF"]
pub type WARNF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `ERRF` reader - desc ERRF"]
pub type ERRF_R = crate::BitReader<bool>;
#[doc = "Field `ERRF` writer - desc ERRF"]
pub type ERRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `UDFF` reader - desc UDFF"]
pub type UDFF_R = crate::BitReader<bool>;
#[doc = "Field `UDFF` writer - desc UDFF"]
pub type UDFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `CAPF` reader - desc CAPF"]
pub type CAPF_R = crate::BitReader<bool>;
#[doc = "Field `CAPF` writer - desc CAPF"]
pub type CAPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `ERR` reader - desc ERR"]
pub type ERR_R = crate::BitReader<bool>;
#[doc = "Field `ERR` writer - desc ERR"]
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `MISS` reader - desc MISS"]
pub type MISS_R = crate::BitReader<bool>;
#[doc = "Field `MISS` writer - desc MISS"]
pub type MISS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `OVF` reader - desc OVF"]
pub type OVF_R = crate::BitReader<bool>;
#[doc = "Field `OVF` writer - desc OVF"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `DIR` reader - desc DIR"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - desc DIR"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `FECAP` reader - desc FECAP"]
pub type FECAP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FECAP` writer - desc FECAP"]
pub type FECAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ISR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - desc OKF"]
    #[inline(always)]
    pub fn okf(&self) -> OKF_R {
        OKF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc WARNF"]
    #[inline(always)]
    pub fn warnf(&self) -> WARNF_R {
        WARNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ERRF"]
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc UDFF"]
    #[inline(always)]
    pub fn udff(&self) -> UDFF_R {
        UDFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CAPF"]
    #[inline(always)]
    pub fn capf(&self) -> CAPF_R {
        CAPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - desc ERR"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc MISS"]
    #[inline(always)]
    pub fn miss(&self) -> MISS_R {
        MISS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc OVF"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - desc DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - desc FECAP"]
    #[inline(always)]
    pub fn fecap(&self) -> FECAP_R {
        FECAP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - desc OKF"]
    #[inline(always)]
    pub fn okf(&mut self) -> OKF_W<0> {
        OKF_W::new(self)
    }
    #[doc = "Bit 1 - desc WARNF"]
    #[inline(always)]
    pub fn warnf(&mut self) -> WARNF_W<1> {
        WARNF_W::new(self)
    }
    #[doc = "Bit 2 - desc ERRF"]
    #[inline(always)]
    pub fn errf(&mut self) -> ERRF_W<2> {
        ERRF_W::new(self)
    }
    #[doc = "Bit 3 - desc UDFF"]
    #[inline(always)]
    pub fn udff(&mut self) -> UDFF_W<3> {
        UDFF_W::new(self)
    }
    #[doc = "Bit 4 - desc CAPF"]
    #[inline(always)]
    pub fn capf(&mut self) -> CAPF_W<4> {
        CAPF_W::new(self)
    }
    #[doc = "Bit 8 - desc ERR"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W<8> {
        ERR_W::new(self)
    }
    #[doc = "Bit 9 - desc MISS"]
    #[inline(always)]
    pub fn miss(&mut self) -> MISS_W<9> {
        MISS_W::new(self)
    }
    #[doc = "Bit 10 - desc OVF"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W<10> {
        OVF_W::new(self)
    }
    #[doc = "Bit 15 - desc DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<15> {
        DIR_W::new(self)
    }
    #[doc = "Bits 16:31 - desc FECAP"]
    #[inline(always)]
    pub fn fecap(&mut self) -> FECAP_W<16> {
        FECAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
