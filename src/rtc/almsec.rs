#[doc = "Register `ALMSEC` reader"]
pub struct R(crate::R<ALMSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALMSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALMSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALMSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALMSEC` writer"]
pub struct W(crate::W<ALMSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALMSEC_SPEC>;
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
impl From<crate::W<ALMSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALMSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALMSECL` reader - desc ALMSECL"]
pub type ALMSECL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALMSECL` writer - desc ALMSECL"]
pub type ALMSECL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALMSEC_SPEC, u8, u8, 4, O>;
#[doc = "Field `ALMSECH` reader - desc ALMSECH"]
pub type ALMSECH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALMSECH` writer - desc ALMSECH"]
pub type ALMSECH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALMSEC_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALMSEC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - desc ALMSECL"]
    #[inline(always)]
    pub fn almsecl(&self) -> ALMSECL_R {
        ALMSECL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - desc ALMSECH"]
    #[inline(always)]
    pub fn almsech(&self) -> ALMSECH_R {
        ALMSECH_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc ALMSECL"]
    #[inline(always)]
    pub fn almsecl(&mut self) -> ALMSECL_W<0> {
        ALMSECL_W::new(self)
    }
    #[doc = "Bits 4:6 - desc ALMSECH"]
    #[inline(always)]
    pub fn almsech(&mut self) -> ALMSECH_W<4> {
        ALMSECH_W::new(self)
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
#[doc = "desc ALMSEC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [almsec](index.html) module"]
pub struct ALMSEC_SPEC;
impl crate::RegisterSpec for ALMSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [almsec::R](R) reader structure"]
impl crate::Readable for ALMSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [almsec::W](W) writer structure"]
impl crate::Writable for ALMSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALMSEC to value 0"]
impl crate::Resettable for ALMSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
