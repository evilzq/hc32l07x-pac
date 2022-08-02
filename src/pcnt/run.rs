#[doc = "Register `RUN` reader"]
pub struct R(crate::R<RUN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RUN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RUN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RUN` writer"]
pub struct W(crate::W<RUN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RUN_SPEC>;
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
impl From<crate::W<RUN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RUN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUN` reader - desc RUN"]
pub type RUN_R = crate::BitReader<bool>;
#[doc = "Field `RUN` writer - desc RUN"]
pub type RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RUN_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RUN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc RUN"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc RUN"]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W<0> {
        RUN_W::new(self)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<31> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc RUN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [run](index.html) module"]
pub struct RUN_SPEC;
impl crate::RegisterSpec for RUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [run::R](R) reader structure"]
impl crate::Readable for RUN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [run::W](W) writer structure"]
impl crate::Writable for RUN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RUN to value 0"]
impl crate::Resettable for RUN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
