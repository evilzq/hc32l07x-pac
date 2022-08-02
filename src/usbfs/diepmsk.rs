#[doc = "Register `DIEPMSK` reader"]
pub struct R(crate::R<DIEPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPMSK` writer"]
pub struct W(crate::W<DIEPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPMSK_SPEC>;
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
impl From<crate::W<DIEPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRCM` reader - desc XFRCM"]
pub type XFRCM_R = crate::BitReader<bool>;
#[doc = "Field `XFRCM` writer - desc XFRCM"]
pub type XFRCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
#[doc = "Field `EPDM` reader - desc EPDM"]
pub type EPDM_R = crate::BitReader<bool>;
#[doc = "Field `EPDM` writer - desc EPDM"]
pub type EPDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
#[doc = "Field `TOM` reader - desc TOM"]
pub type TOM_R = crate::BitReader<bool>;
#[doc = "Field `TOM` writer - desc TOM"]
pub type TOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
#[doc = "Field `ITTXFEMSK` reader - desc ITTXFEMSK"]
pub type ITTXFEMSK_R = crate::BitReader<bool>;
#[doc = "Field `ITTXFEMSK` writer - desc ITTXFEMSK"]
pub type ITTXFEMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
#[doc = "Field `INEPNMM` reader - desc INEPNMM"]
pub type INEPNMM_R = crate::BitReader<bool>;
#[doc = "Field `INEPNMM` writer - desc INEPNMM"]
pub type INEPNMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
#[doc = "Field `IINEPNEM` reader - desc IINEPNEM"]
pub type IINEPNEM_R = crate::BitReader<bool>;
#[doc = "Field `IINEPNEM` writer - desc IINEPNEM"]
pub type IINEPNEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EPDM"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TOM"]
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc ITTXFEMSK"]
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc INEPNMM"]
    #[inline(always)]
    pub fn inepnmm(&self) -> INEPNMM_R {
        INEPNMM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc IINEPNEM"]
    #[inline(always)]
    pub fn iinepnem(&self) -> IINEPNEM_R {
        IINEPNEM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<0> {
        XFRCM_W::new(self)
    }
    #[doc = "Bit 1 - desc EPDM"]
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W<1> {
        EPDM_W::new(self)
    }
    #[doc = "Bit 3 - desc TOM"]
    #[inline(always)]
    pub fn tom(&mut self) -> TOM_W<3> {
        TOM_W::new(self)
    }
    #[doc = "Bit 4 - desc ITTXFEMSK"]
    #[inline(always)]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W<4> {
        ITTXFEMSK_W::new(self)
    }
    #[doc = "Bit 5 - desc INEPNMM"]
    #[inline(always)]
    pub fn inepnmm(&mut self) -> INEPNMM_W<5> {
        INEPNMM_W::new(self)
    }
    #[doc = "Bit 6 - desc IINEPNEM"]
    #[inline(always)]
    pub fn iinepnem(&mut self) -> IINEPNEM_W<6> {
        IINEPNEM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc DIEPMSK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepmsk](index.html) module"]
pub struct DIEPMSK_SPEC;
impl crate::RegisterSpec for DIEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepmsk::R](R) reader structure"]
impl crate::Readable for DIEPMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepmsk::W](W) writer structure"]
impl crate::Writable for DIEPMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPMSK to value 0"]
impl crate::Resettable for DIEPMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
