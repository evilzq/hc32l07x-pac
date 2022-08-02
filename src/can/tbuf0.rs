#[doc = "Register `TBUF0` reader"]
pub struct R(crate::R<TBUF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBUF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBUF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBUF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBUF0` writer"]
pub struct W(crate::W<TBUF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBUF0_SPEC>;
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
impl From<crate::W<TBUF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBUF0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TBUF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbuf0](index.html) module"]
pub struct TBUF0_SPEC;
impl crate::RegisterSpec for TBUF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbuf0::R](R) reader structure"]
impl crate::Readable for TBUF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbuf0::W](W) writer structure"]
impl crate::Writable for TBUF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBUF0 to value 0"]
impl crate::Resettable for TBUF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
