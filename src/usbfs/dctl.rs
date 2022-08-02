#[doc = "Register `DCTL` reader"]
pub struct R(crate::R<DCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCTL` writer"]
pub struct W(crate::W<DCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCTL_SPEC>;
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
impl From<crate::W<DCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWUSIG` reader - desc RWUSIG"]
pub type RWUSIG_R = crate::BitReader<bool>;
#[doc = "Field `RWUSIG` writer - desc RWUSIG"]
pub type RWUSIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, O>;
#[doc = "Field `SDIS` reader - desc SDIS"]
pub type SDIS_R = crate::BitReader<bool>;
#[doc = "Field `SDIS` writer - desc SDIS"]
pub type SDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, O>;
#[doc = "Field `GINSTS` reader - desc GINSTS"]
pub type GINSTS_R = crate::BitReader<bool>;
#[doc = "Field `GONSTS` reader - desc GONSTS"]
pub type GONSTS_R = crate::BitReader<bool>;
#[doc = "Field `TCTL` reader - desc TCTL"]
pub type TCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCTL` writer - desc TCTL"]
pub type TCTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SGINAK` writer - desc SGINAK"]
pub type SGINAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, O>;
#[doc = "Field `CGINAK` writer - desc CGINAK"]
pub type CGINAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, O>;
#[doc = "Field `SGONAK` writer - desc SGONAK"]
pub type SGONAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, O>;
#[doc = "Field `CGONAK` writer - desc CGONAK"]
pub type CGONAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, O>;
#[doc = "Field `POPRGDNE` reader - desc POPRGDNE"]
pub type POPRGDNE_R = crate::BitReader<bool>;
#[doc = "Field `POPRGDNE` writer - desc POPRGDNE"]
pub type POPRGDNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc RWUSIG"]
    #[inline(always)]
    pub fn rwusig(&self) -> RWUSIG_R {
        RWUSIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SDIS"]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc GINSTS"]
    #[inline(always)]
    pub fn ginsts(&self) -> GINSTS_R {
        GINSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc GONSTS"]
    #[inline(always)]
    pub fn gonsts(&self) -> GONSTS_R {
        GONSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc TCTL"]
    #[inline(always)]
    pub fn tctl(&self) -> TCTL_R {
        TCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11 - desc POPRGDNE"]
    #[inline(always)]
    pub fn poprgdne(&self) -> POPRGDNE_R {
        POPRGDNE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc RWUSIG"]
    #[inline(always)]
    pub fn rwusig(&mut self) -> RWUSIG_W<0> {
        RWUSIG_W::new(self)
    }
    #[doc = "Bit 1 - desc SDIS"]
    #[inline(always)]
    pub fn sdis(&mut self) -> SDIS_W<1> {
        SDIS_W::new(self)
    }
    #[doc = "Bits 4:6 - desc TCTL"]
    #[inline(always)]
    pub fn tctl(&mut self) -> TCTL_W<4> {
        TCTL_W::new(self)
    }
    #[doc = "Bit 7 - desc SGINAK"]
    #[inline(always)]
    pub fn sginak(&mut self) -> SGINAK_W<7> {
        SGINAK_W::new(self)
    }
    #[doc = "Bit 8 - desc CGINAK"]
    #[inline(always)]
    pub fn cginak(&mut self) -> CGINAK_W<8> {
        CGINAK_W::new(self)
    }
    #[doc = "Bit 9 - desc SGONAK"]
    #[inline(always)]
    pub fn sgonak(&mut self) -> SGONAK_W<9> {
        SGONAK_W::new(self)
    }
    #[doc = "Bit 10 - desc CGONAK"]
    #[inline(always)]
    pub fn cgonak(&mut self) -> CGONAK_W<10> {
        CGONAK_W::new(self)
    }
    #[doc = "Bit 11 - desc POPRGDNE"]
    #[inline(always)]
    pub fn poprgdne(&mut self) -> POPRGDNE_W<11> {
        POPRGDNE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc DCTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctl](index.html) module"]
pub struct DCTL_SPEC;
impl crate::RegisterSpec for DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dctl::R](R) reader structure"]
impl crate::Readable for DCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dctl::W](W) writer structure"]
impl crate::Writable for DCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCTL to value 0x02"]
impl crate::Resettable for DCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
