#[doc = "Register `CFG_STAT` reader"]
pub struct R(crate::R<CFG_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_STAT` writer"]
pub struct W(crate::W<CFG_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_STAT_SPEC>;
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
impl From<crate::W<CFG_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSOFF` reader - desc BUSOFF"]
pub type BUSOFF_R = crate::BitReader<bool>;
#[doc = "Field `BUSOFF` writer - desc BUSOFF"]
pub type BUSOFF_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFG_STAT_SPEC, bool, O>;
#[doc = "Field `TACTIVE` reader - desc TACTIVE"]
pub type TACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `RACTIVE` reader - desc RACTIVE"]
pub type RACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `TSSS` reader - desc TSSS"]
pub type TSSS_R = crate::BitReader<bool>;
#[doc = "Field `TSSS` writer - desc TSSS"]
pub type TSSS_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFG_STAT_SPEC, bool, O>;
#[doc = "Field `TPSS` reader - desc TPSS"]
pub type TPSS_R = crate::BitReader<bool>;
#[doc = "Field `TPSS` writer - desc TPSS"]
pub type TPSS_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFG_STAT_SPEC, bool, O>;
#[doc = "Field `LBMI` reader - desc LBMI"]
pub type LBMI_R = crate::BitReader<bool>;
#[doc = "Field `LBMI` writer - desc LBMI"]
pub type LBMI_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFG_STAT_SPEC, bool, O>;
#[doc = "Field `LBME` reader - desc LBME"]
pub type LBME_R = crate::BitReader<bool>;
#[doc = "Field `LBME` writer - desc LBME"]
pub type LBME_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFG_STAT_SPEC, bool, O>;
#[doc = "Field `RESET` reader - desc RESET"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - desc RESET"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u8, CFG_STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc BUSOFF"]
    #[inline(always)]
    pub fn busoff(&self) -> BUSOFF_R {
        BUSOFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TACTIVE"]
    #[inline(always)]
    pub fn tactive(&self) -> TACTIVE_R {
        TACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RACTIVE"]
    #[inline(always)]
    pub fn ractive(&self) -> RACTIVE_R {
        RACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TSSS"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TPSS"]
    #[inline(always)]
    pub fn tpss(&self) -> TPSS_R {
        TPSS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc LBMI"]
    #[inline(always)]
    pub fn lbmi(&self) -> LBMI_R {
        LBMI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc LBME"]
    #[inline(always)]
    pub fn lbme(&self) -> LBME_R {
        LBME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc BUSOFF"]
    #[inline(always)]
    pub fn busoff(&mut self) -> BUSOFF_W<0> {
        BUSOFF_W::new(self)
    }
    #[doc = "Bit 3 - desc TSSS"]
    #[inline(always)]
    pub fn tsss(&mut self) -> TSSS_W<3> {
        TSSS_W::new(self)
    }
    #[doc = "Bit 4 - desc TPSS"]
    #[inline(always)]
    pub fn tpss(&mut self) -> TPSS_W<4> {
        TPSS_W::new(self)
    }
    #[doc = "Bit 5 - desc LBMI"]
    #[inline(always)]
    pub fn lbmi(&mut self) -> LBMI_W<5> {
        LBMI_W::new(self)
    }
    #[doc = "Bit 6 - desc LBME"]
    #[inline(always)]
    pub fn lbme(&mut self) -> LBME_W<6> {
        LBME_W::new(self)
    }
    #[doc = "Bit 7 - desc RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<7> {
        RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CFG_STAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_stat](index.html) module"]
pub struct CFG_STAT_SPEC;
impl crate::RegisterSpec for CFG_STAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cfg_stat::R](R) reader structure"]
impl crate::Readable for CFG_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_stat::W](W) writer structure"]
impl crate::Writable for CFG_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_STAT to value 0x80"]
impl crate::Resettable for CFG_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
