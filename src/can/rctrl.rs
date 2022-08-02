#[doc = "Register `RCTRL` reader"]
pub struct R(crate::R<RCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCTRL` writer"]
pub struct W(crate::W<RCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCTRL_SPEC>;
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
impl From<crate::W<RCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSSTAT` reader - desc RSSTAT"]
pub type RSSTAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RBALL` reader - desc RBALL"]
pub type RBALL_R = crate::BitReader<bool>;
#[doc = "Field `RBALL` writer - desc RBALL"]
pub type RBALL_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCTRL_SPEC, bool, O>;
#[doc = "Field `RREL` reader - desc RREL"]
pub type RREL_R = crate::BitReader<bool>;
#[doc = "Field `RREL` writer - desc RREL"]
pub type RREL_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCTRL_SPEC, bool, O>;
#[doc = "Field `ROV` reader - desc ROV"]
pub type ROV_R = crate::BitReader<bool>;
#[doc = "Field `ROM` reader - desc ROM"]
pub type ROM_R = crate::BitReader<bool>;
#[doc = "Field `ROM` writer - desc ROM"]
pub type ROM_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCTRL_SPEC, bool, O>;
#[doc = "Field `SACK` reader - desc SACK"]
pub type SACK_R = crate::BitReader<bool>;
#[doc = "Field `SACK` writer - desc SACK"]
pub type SACK_W<'a, const O: u8> = crate::BitWriter<'a, u8, RCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - desc RSSTAT"]
    #[inline(always)]
    pub fn rsstat(&self) -> RSSTAT_R {
        RSSTAT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - desc RBALL"]
    #[inline(always)]
    pub fn rball(&self) -> RBALL_R {
        RBALL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RREL"]
    #[inline(always)]
    pub fn rrel(&self) -> RREL_R {
        RREL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ROV"]
    #[inline(always)]
    pub fn rov(&self) -> ROV_R {
        ROV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc ROM"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SACK"]
    #[inline(always)]
    pub fn sack(&self) -> SACK_R {
        SACK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - desc RBALL"]
    #[inline(always)]
    pub fn rball(&mut self) -> RBALL_W<3> {
        RBALL_W::new(self)
    }
    #[doc = "Bit 4 - desc RREL"]
    #[inline(always)]
    pub fn rrel(&mut self) -> RREL_W<4> {
        RREL_W::new(self)
    }
    #[doc = "Bit 6 - desc ROM"]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W<6> {
        ROM_W::new(self)
    }
    #[doc = "Bit 7 - desc SACK"]
    #[inline(always)]
    pub fn sack(&mut self) -> SACK_W<7> {
        SACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc RCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rctrl](index.html) module"]
pub struct RCTRL_SPEC;
impl crate::RegisterSpec for RCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rctrl::R](R) reader structure"]
impl crate::Readable for RCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rctrl::W](W) writer structure"]
impl crate::Writable for RCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCTRL to value 0"]
impl crate::Resettable for RCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
