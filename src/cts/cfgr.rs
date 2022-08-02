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
#[doc = "Field `ARR` reader - desc ARR"]
pub type ARR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ARR` writer - desc ARR"]
pub type ARR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u16, u16, 16, O>;
#[doc = "Field `FELIM` reader - desc FELIM"]
pub type FELIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FELIM` writer - desc FELIM"]
pub type FELIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DIV` reader - desc DIV"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - desc DIV"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `POL` reader - desc POL"]
pub type POL_R = crate::BitReader<bool>;
#[doc = "Field `POL` writer - desc POL"]
pub type POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `REFSRC` reader - desc REFSRC"]
pub type REFSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFSRC` writer - desc REFSRC"]
pub type REFSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CLKSRC` reader - desc CLKSRC"]
pub type CLKSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKSRC` writer - desc CLKSRC"]
pub type CLKSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:15 - desc ARR"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - desc FELIM"]
    #[inline(always)]
    pub fn felim(&self) -> FELIM_R {
        FELIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - desc DIV"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - desc POL"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - desc REFSRC"]
    #[inline(always)]
    pub fn refsrc(&self) -> REFSRC_R {
        REFSRC_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - desc CLKSRC"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc ARR"]
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W<0> {
        ARR_W::new(self)
    }
    #[doc = "Bits 16:23 - desc FELIM"]
    #[inline(always)]
    pub fn felim(&mut self) -> FELIM_W<16> {
        FELIM_W::new(self)
    }
    #[doc = "Bits 24:26 - desc DIV"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<24> {
        DIV_W::new(self)
    }
    #[doc = "Bit 27 - desc POL"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<27> {
        POL_W::new(self)
    }
    #[doc = "Bits 28:29 - desc REFSRC"]
    #[inline(always)]
    pub fn refsrc(&mut self) -> REFSRC_W<28> {
        REFSRC_W::new(self)
    }
    #[doc = "Bits 30:31 - desc CLKSRC"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W<30> {
        CLKSRC_W::new(self)
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
#[doc = "`reset()` method sets CFGR to value 0x0022_bb7f"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0022_bb7f
    }
}
