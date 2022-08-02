#[doc = "Register `DOEPINT4` reader"]
pub struct R(crate::R<DOEPINT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPINT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPINT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPINT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPINT4` writer"]
pub struct W(crate::W<DOEPINT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPINT4_SPEC>;
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
impl From<crate::W<DOEPINT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPINT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRC` reader - desc XFRC"]
pub type XFRC_R = crate::BitReader<bool>;
#[doc = "Field `XFRC` writer - desc XFRC"]
pub type XFRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT4_SPEC, bool, O>;
#[doc = "Field `EPDISD` reader - desc EPDISD"]
pub type EPDISD_R = crate::BitReader<bool>;
#[doc = "Field `EPDISD` writer - desc EPDISD"]
pub type EPDISD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT4_SPEC, bool, O>;
#[doc = "Field `STUP` reader - desc STUP"]
pub type STUP_R = crate::BitReader<bool>;
#[doc = "Field `STUP` writer - desc STUP"]
pub type STUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT4_SPEC, bool, O>;
#[doc = "Field `OTEPDIS` reader - desc OTEPDIS"]
pub type OTEPDIS_R = crate::BitReader<bool>;
#[doc = "Field `OTEPDIS` writer - desc OTEPDIS"]
pub type OTEPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT4_SPEC, bool, O>;
#[doc = "Field `B2BSTUP` reader - desc B2BSTUP"]
pub type B2BSTUP_R = crate::BitReader<bool>;
#[doc = "Field `B2BSTUP` writer - desc B2BSTUP"]
pub type B2BSTUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc STUP"]
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc OTEPDIS"]
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc B2BSTUP"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc XFRC"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<0> {
        XFRC_W::new(self)
    }
    #[doc = "Bit 1 - desc EPDISD"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<1> {
        EPDISD_W::new(self)
    }
    #[doc = "Bit 3 - desc STUP"]
    #[inline(always)]
    pub fn stup(&mut self) -> STUP_W<3> {
        STUP_W::new(self)
    }
    #[doc = "Bit 4 - desc OTEPDIS"]
    #[inline(always)]
    pub fn otepdis(&mut self) -> OTEPDIS_W<4> {
        OTEPDIS_W::new(self)
    }
    #[doc = "Bit 6 - desc B2BSTUP"]
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<6> {
        B2BSTUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc DOEPINT4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint4](index.html) module"]
pub struct DOEPINT4_SPEC;
impl crate::RegisterSpec for DOEPINT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepint4::R](R) reader structure"]
impl crate::Readable for DOEPINT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepint4::W](W) writer structure"]
impl crate::Writable for DOEPINT4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPINT4 to value 0"]
impl crate::Resettable for DOEPINT4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
