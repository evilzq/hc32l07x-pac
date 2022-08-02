#[doc = "Register `ACF` reader"]
pub struct R(crate::R<ACF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACF` writer"]
pub struct W(crate::W<ACF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACF_SPEC>;
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
impl From<crate::W<ACF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACODEORAMASK` reader - desc ACODEORAMASK"]
pub type ACODEORAMASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ACODEORAMASK` writer - desc ACODEORAMASK"]
pub type ACODEORAMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACF_SPEC, u32, u32, 29, O>;
#[doc = "Field `AIDE` reader - desc AIDE"]
pub type AIDE_R = crate::BitReader<bool>;
#[doc = "Field `AIDE` writer - desc AIDE"]
pub type AIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACF_SPEC, bool, O>;
#[doc = "Field `AIDEE` reader - desc AIDEE"]
pub type AIDEE_R = crate::BitReader<bool>;
#[doc = "Field `AIDEE` writer - desc AIDEE"]
pub type AIDEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:28 - desc ACODEORAMASK"]
    #[inline(always)]
    pub fn acodeoramask(&self) -> ACODEORAMASK_R {
        ACODEORAMASK_R::new((self.bits & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 29 - desc AIDE"]
    #[inline(always)]
    pub fn aide(&self) -> AIDE_R {
        AIDE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc AIDEE"]
    #[inline(always)]
    pub fn aidee(&self) -> AIDEE_R {
        AIDEE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - desc ACODEORAMASK"]
    #[inline(always)]
    pub fn acodeoramask(&mut self) -> ACODEORAMASK_W<0> {
        ACODEORAMASK_W::new(self)
    }
    #[doc = "Bit 29 - desc AIDE"]
    #[inline(always)]
    pub fn aide(&mut self) -> AIDE_W<29> {
        AIDE_W::new(self)
    }
    #[doc = "Bit 30 - desc AIDEE"]
    #[inline(always)]
    pub fn aidee(&mut self) -> AIDEE_W<30> {
        AIDEE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc ACF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acf](index.html) module"]
pub struct ACF_SPEC;
impl crate::RegisterSpec for ACF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acf::R](R) reader structure"]
impl crate::Readable for ACF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acf::W](W) writer structure"]
impl crate::Writable for ACF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACF to value 0"]
impl crate::Resettable for ACF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
