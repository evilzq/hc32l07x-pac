#[doc = "Register `DBG` reader"]
pub struct R(crate::R<DBG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG` writer"]
pub struct W(crate::W<DBG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_SPEC>;
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
impl From<crate::W<DBG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG` reader - desc DBG"]
pub type DBG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBG` writer - desc DBG"]
pub type DBG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBG_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - desc DBG"]
    #[inline(always)]
    pub fn dbg(&self) -> DBG_R {
        DBG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc DBG"]
    #[inline(always)]
    pub fn dbg(&mut self) -> DBG_W<0> {
        DBG_W::new(self)
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
#[doc = "desc DBG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg](index.html) module"]
pub struct DBG_SPEC;
impl crate::RegisterSpec for DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg::R](R) reader structure"]
impl crate::Readable for DBG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg::W](W) writer structure"]
impl crate::Writable for DBG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBG to value 0"]
impl crate::Resettable for DBG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
