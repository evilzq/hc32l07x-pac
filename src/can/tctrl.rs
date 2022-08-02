#[doc = "Register `TCTRL` reader"]
pub struct R(crate::R<TCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCTRL` writer"]
pub struct W(crate::W<TCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCTRL_SPEC>;
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
impl From<crate::W<TCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSSTAT` reader - desc TSSTAT"]
pub type TSSTAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTBM` reader - desc TTBM"]
pub type TTBM_R = crate::BitReader<bool>;
#[doc = "Field `TTBM` writer - desc TTBM"]
pub type TTBM_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCTRL_SPEC, bool, O>;
#[doc = "Field `TSMODE` reader - desc TSMODE"]
pub type TSMODE_R = crate::BitReader<bool>;
#[doc = "Field `TSMODE` writer - desc TSMODE"]
pub type TSMODE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCTRL_SPEC, bool, O>;
#[doc = "Field `TSNEXT` reader - desc TSNEXT"]
pub type TSNEXT_R = crate::BitReader<bool>;
#[doc = "Field `TSNEXT` writer - desc TSNEXT"]
pub type TSNEXT_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - desc TSSTAT"]
    #[inline(always)]
    pub fn tsstat(&self) -> TSSTAT_R {
        TSSTAT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - desc TTBM"]
    #[inline(always)]
    pub fn ttbm(&self) -> TTBM_R {
        TTBM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc TSMODE"]
    #[inline(always)]
    pub fn tsmode(&self) -> TSMODE_R {
        TSMODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TSNEXT"]
    #[inline(always)]
    pub fn tsnext(&self) -> TSNEXT_R {
        TSNEXT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - desc TTBM"]
    #[inline(always)]
    pub fn ttbm(&mut self) -> TTBM_W<4> {
        TTBM_W::new(self)
    }
    #[doc = "Bit 5 - desc TSMODE"]
    #[inline(always)]
    pub fn tsmode(&mut self) -> TSMODE_W<5> {
        TSMODE_W::new(self)
    }
    #[doc = "Bit 6 - desc TSNEXT"]
    #[inline(always)]
    pub fn tsnext(&mut self) -> TSNEXT_W<6> {
        TSNEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tctrl](index.html) module"]
pub struct TCTRL_SPEC;
impl crate::RegisterSpec for TCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tctrl::R](R) reader structure"]
impl crate::Readable for TCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tctrl::W](W) writer structure"]
impl crate::Writable for TCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCTRL to value 0x90"]
impl crate::Resettable for TCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x90
    }
}
