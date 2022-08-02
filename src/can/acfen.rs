#[doc = "Register `ACFEN` reader"]
pub struct R(crate::R<ACFEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACFEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACFEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACFEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACFEN` writer"]
pub struct W(crate::W<ACFEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACFEN_SPEC>;
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
impl From<crate::W<ACFEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACFEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AE_1` reader - desc AE_1"]
pub type AE_1_R = crate::BitReader<bool>;
#[doc = "Field `AE_1` writer - desc AE_1"]
pub type AE_1_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACFEN_SPEC, bool, O>;
#[doc = "Field `AE_2` reader - desc AE_2"]
pub type AE_2_R = crate::BitReader<bool>;
#[doc = "Field `AE_2` writer - desc AE_2"]
pub type AE_2_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACFEN_SPEC, bool, O>;
#[doc = "Field `AE_3` reader - desc AE_3"]
pub type AE_3_R = crate::BitReader<bool>;
#[doc = "Field `AE_3` writer - desc AE_3"]
pub type AE_3_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACFEN_SPEC, bool, O>;
#[doc = "Field `AE_4` reader - desc AE_4"]
pub type AE_4_R = crate::BitReader<bool>;
#[doc = "Field `AE_4` writer - desc AE_4"]
pub type AE_4_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACFEN_SPEC, bool, O>;
#[doc = "Field `AE_5` reader - desc AE_5"]
pub type AE_5_R = crate::BitReader<bool>;
#[doc = "Field `AE_5` writer - desc AE_5"]
pub type AE_5_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACFEN_SPEC, bool, O>;
#[doc = "Field `AE_6` reader - desc AE_6"]
pub type AE_6_R = crate::BitReader<bool>;
#[doc = "Field `AE_6` writer - desc AE_6"]
pub type AE_6_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACFEN_SPEC, bool, O>;
#[doc = "Field `AE_7` reader - desc AE_7"]
pub type AE_7_R = crate::BitReader<bool>;
#[doc = "Field `AE_7` writer - desc AE_7"]
pub type AE_7_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACFEN_SPEC, bool, O>;
#[doc = "Field `AE_8` reader - desc AE_8"]
pub type AE_8_R = crate::BitReader<bool>;
#[doc = "Field `AE_8` writer - desc AE_8"]
pub type AE_8_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACFEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc AE_1"]
    #[inline(always)]
    pub fn ae_1(&self) -> AE_1_R {
        AE_1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc AE_2"]
    #[inline(always)]
    pub fn ae_2(&self) -> AE_2_R {
        AE_2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc AE_3"]
    #[inline(always)]
    pub fn ae_3(&self) -> AE_3_R {
        AE_3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc AE_4"]
    #[inline(always)]
    pub fn ae_4(&self) -> AE_4_R {
        AE_4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc AE_5"]
    #[inline(always)]
    pub fn ae_5(&self) -> AE_5_R {
        AE_5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc AE_6"]
    #[inline(always)]
    pub fn ae_6(&self) -> AE_6_R {
        AE_6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc AE_7"]
    #[inline(always)]
    pub fn ae_7(&self) -> AE_7_R {
        AE_7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc AE_8"]
    #[inline(always)]
    pub fn ae_8(&self) -> AE_8_R {
        AE_8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc AE_1"]
    #[inline(always)]
    pub fn ae_1(&mut self) -> AE_1_W<0> {
        AE_1_W::new(self)
    }
    #[doc = "Bit 1 - desc AE_2"]
    #[inline(always)]
    pub fn ae_2(&mut self) -> AE_2_W<1> {
        AE_2_W::new(self)
    }
    #[doc = "Bit 2 - desc AE_3"]
    #[inline(always)]
    pub fn ae_3(&mut self) -> AE_3_W<2> {
        AE_3_W::new(self)
    }
    #[doc = "Bit 3 - desc AE_4"]
    #[inline(always)]
    pub fn ae_4(&mut self) -> AE_4_W<3> {
        AE_4_W::new(self)
    }
    #[doc = "Bit 4 - desc AE_5"]
    #[inline(always)]
    pub fn ae_5(&mut self) -> AE_5_W<4> {
        AE_5_W::new(self)
    }
    #[doc = "Bit 5 - desc AE_6"]
    #[inline(always)]
    pub fn ae_6(&mut self) -> AE_6_W<5> {
        AE_6_W::new(self)
    }
    #[doc = "Bit 6 - desc AE_7"]
    #[inline(always)]
    pub fn ae_7(&mut self) -> AE_7_W<6> {
        AE_7_W::new(self)
    }
    #[doc = "Bit 7 - desc AE_8"]
    #[inline(always)]
    pub fn ae_8(&mut self) -> AE_8_W<7> {
        AE_8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc ACFEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acfen](index.html) module"]
pub struct ACFEN_SPEC;
impl crate::RegisterSpec for ACFEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [acfen::R](R) reader structure"]
impl crate::Readable for ACFEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acfen::W](W) writer structure"]
impl crate::Writable for ACFEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACFEN to value 0x01"]
impl crate::Resettable for ACFEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
