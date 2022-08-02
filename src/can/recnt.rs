#[doc = "Register `RECNT` reader"]
pub struct R(crate::R<RECNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECNT` writer"]
pub struct W(crate::W<RECNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECNT_SPEC>;
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
impl From<crate::W<RECNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECNT_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc RECNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [recnt](index.html) module"]
pub struct RECNT_SPEC;
impl crate::RegisterSpec for RECNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [recnt::R](R) reader structure"]
impl crate::Readable for RECNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [recnt::W](W) writer structure"]
impl crate::Writable for RECNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RECNT to value 0"]
impl crate::Resettable for RECNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
