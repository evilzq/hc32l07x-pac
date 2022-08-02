#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OKC` reader - desc OKC"]
pub type OKC_R = crate::BitReader<bool>;
#[doc = "Field `OKC` writer - desc OKC"]
pub type OKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `WARNC` reader - desc WARNC"]
pub type WARNC_R = crate::BitReader<bool>;
#[doc = "Field `WARNC` writer - desc WARNC"]
pub type WARNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `ERRC` reader - desc ERRC"]
pub type ERRC_R = crate::BitReader<bool>;
#[doc = "Field `ERRC` writer - desc ERRC"]
pub type ERRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `UDFC` reader - desc UDFC"]
pub type UDFC_R = crate::BitReader<bool>;
#[doc = "Field `UDFC` writer - desc UDFC"]
pub type UDFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CAPC` reader - desc CAPC"]
pub type CAPC_R = crate::BitReader<bool>;
#[doc = "Field `CAPC` writer - desc CAPC"]
pub type CAPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc OKC"]
    #[inline(always)]
    pub fn okc(&self) -> OKC_R {
        OKC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc WARNC"]
    #[inline(always)]
    pub fn warnc(&self) -> WARNC_R {
        WARNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ERRC"]
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc UDFC"]
    #[inline(always)]
    pub fn udfc(&self) -> UDFC_R {
        UDFC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CAPC"]
    #[inline(always)]
    pub fn capc(&self) -> CAPC_R {
        CAPC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc OKC"]
    #[inline(always)]
    pub fn okc(&mut self) -> OKC_W<0> {
        OKC_W::new(self)
    }
    #[doc = "Bit 1 - desc WARNC"]
    #[inline(always)]
    pub fn warnc(&mut self) -> WARNC_W<1> {
        WARNC_W::new(self)
    }
    #[doc = "Bit 2 - desc ERRC"]
    #[inline(always)]
    pub fn errc(&mut self) -> ERRC_W<2> {
        ERRC_W::new(self)
    }
    #[doc = "Bit 3 - desc UDFC"]
    #[inline(always)]
    pub fn udfc(&mut self) -> UDFC_W<3> {
        UDFC_W::new(self)
    }
    #[doc = "Bit 4 - desc CAPC"]
    #[inline(always)]
    pub fn capc(&mut self) -> CAPC_W<4> {
        CAPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc ICR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0x1f"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
