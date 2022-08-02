#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIASSEL0` reader - desc BIASSEL0"]
pub type BIASSEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASSEL0` writer - desc BIASSEL0"]
pub type BIASSEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `BIASSEL1` reader - desc BIASSEL1"]
pub type BIASSEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASSEL1` writer - desc BIASSEL1"]
pub type BIASSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `BIASSEL2` reader - desc BIASSEL2"]
pub type BIASSEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASSEL2` writer - desc BIASSEL2"]
pub type BIASSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `BIASSEL3` reader - desc BIASSEL3"]
pub type BIASSEL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASSEL3` writer - desc BIASSEL3"]
pub type BIASSEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `BIASSEL4` reader - desc BIASSEL4"]
pub type BIASSEL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASSEL4` writer - desc BIASSEL4"]
pub type BIASSEL4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - desc BIASSEL0"]
    #[inline(always)]
    pub fn biassel0(&self) -> BIASSEL0_R {
        BIASSEL0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - desc BIASSEL1"]
    #[inline(always)]
    pub fn biassel1(&self) -> BIASSEL1_R {
        BIASSEL1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - desc BIASSEL2"]
    #[inline(always)]
    pub fn biassel2(&self) -> BIASSEL2_R {
        BIASSEL2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - desc BIASSEL3"]
    #[inline(always)]
    pub fn biassel3(&self) -> BIASSEL3_R {
        BIASSEL3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc BIASSEL4"]
    #[inline(always)]
    pub fn biassel4(&self) -> BIASSEL4_R {
        BIASSEL4_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc BIASSEL0"]
    #[inline(always)]
    pub fn biassel0(&mut self) -> BIASSEL0_W<0> {
        BIASSEL0_W::new(self)
    }
    #[doc = "Bits 3:5 - desc BIASSEL1"]
    #[inline(always)]
    pub fn biassel1(&mut self) -> BIASSEL1_W<3> {
        BIASSEL1_W::new(self)
    }
    #[doc = "Bits 6:8 - desc BIASSEL2"]
    #[inline(always)]
    pub fn biassel2(&mut self) -> BIASSEL2_W<6> {
        BIASSEL2_W::new(self)
    }
    #[doc = "Bits 9:11 - desc BIASSEL3"]
    #[inline(always)]
    pub fn biassel3(&mut self) -> BIASSEL3_W<9> {
        BIASSEL3_W::new(self)
    }
    #[doc = "Bits 12:14 - desc BIASSEL4"]
    #[inline(always)]
    pub fn biassel4(&mut self) -> BIASSEL4_W<12> {
        BIASSEL4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0x36db"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x36db
    }
}
