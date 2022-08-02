#[doc = "Register `TRG_CFG` reader"]
pub struct R(crate::R<TRG_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRG_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRG_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRG_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRG_CFG` writer"]
pub struct W(crate::W<TRG_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRG_CFG_SPEC>;
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
impl From<crate::W<TRG_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRG_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTPTR` reader - desc TTPTR"]
pub type TTPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTPTR` writer - desc TTPTR"]
pub type TTPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TRG_CFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `TTYPE` reader - desc TTYPE"]
pub type TTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTYPE` writer - desc TTYPE"]
pub type TTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TRG_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `TEW` reader - desc TEW"]
pub type TEW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEW` writer - desc TEW"]
pub type TEW_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TRG_CFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:5 - desc TTPTR"]
    #[inline(always)]
    pub fn ttptr(&self) -> TTPTR_R {
        TTPTR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - desc TTYPE"]
    #[inline(always)]
    pub fn ttype(&self) -> TTYPE_R {
        TTYPE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15 - desc TEW"]
    #[inline(always)]
    pub fn tew(&self) -> TEW_R {
        TEW_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc TTPTR"]
    #[inline(always)]
    pub fn ttptr(&mut self) -> TTPTR_W<0> {
        TTPTR_W::new(self)
    }
    #[doc = "Bits 8:10 - desc TTYPE"]
    #[inline(always)]
    pub fn ttype(&mut self) -> TTYPE_W<8> {
        TTYPE_W::new(self)
    }
    #[doc = "Bits 12:15 - desc TEW"]
    #[inline(always)]
    pub fn tew(&mut self) -> TEW_W<12> {
        TEW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TRG_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trg_cfg](index.html) module"]
pub struct TRG_CFG_SPEC;
impl crate::RegisterSpec for TRG_CFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [trg_cfg::R](R) reader structure"]
impl crate::Readable for TRG_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trg_cfg::W](W) writer structure"]
impl crate::Writable for TRG_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRG_CFG to value 0"]
impl crate::Resettable for TRG_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
