#[doc = "Register `GINTMSK` reader"]
pub struct R(crate::R<GINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GINTMSK` writer"]
pub struct W(crate::W<GINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTMSK_SPEC>;
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
impl From<crate::W<GINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFM` reader - desc SOFM"]
pub type SOFM_R = crate::BitReader<bool>;
#[doc = "Field `SOFM` writer - desc SOFM"]
pub type SOFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `RXFNEM` reader - desc RXFNEM"]
pub type RXFNEM_R = crate::BitReader<bool>;
#[doc = "Field `RXFNEM` writer - desc RXFNEM"]
pub type RXFNEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `GINAKEFFM` reader - desc GINAKEFFM"]
pub type GINAKEFFM_R = crate::BitReader<bool>;
#[doc = "Field `GINAKEFFM` writer - desc GINAKEFFM"]
pub type GINAKEFFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `GOUTNAKEFFM` reader - desc GOUTNAKEFFM"]
pub type GOUTNAKEFFM_R = crate::BitReader<bool>;
#[doc = "Field `GOUTNAKEFFM` writer - desc GOUTNAKEFFM"]
pub type GOUTNAKEFFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `ESUSPM` reader - desc ESUSPM"]
pub type ESUSPM_R = crate::BitReader<bool>;
#[doc = "Field `ESUSPM` writer - desc ESUSPM"]
pub type ESUSPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `USBSUSPM` reader - desc USBSUSPM"]
pub type USBSUSPM_R = crate::BitReader<bool>;
#[doc = "Field `USBSUSPM` writer - desc USBSUSPM"]
pub type USBSUSPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `USBRSTM` reader - desc USBRSTM"]
pub type USBRSTM_R = crate::BitReader<bool>;
#[doc = "Field `USBRSTM` writer - desc USBRSTM"]
pub type USBRSTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `ENUMDNEM` reader - desc ENUMDNEM"]
pub type ENUMDNEM_R = crate::BitReader<bool>;
#[doc = "Field `ENUMDNEM` writer - desc ENUMDNEM"]
pub type ENUMDNEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `ISOODRPM` reader - desc ISOODRPM"]
pub type ISOODRPM_R = crate::BitReader<bool>;
#[doc = "Field `ISOODRPM` writer - desc ISOODRPM"]
pub type ISOODRPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `EOPFM` reader - desc EOPFM"]
pub type EOPFM_R = crate::BitReader<bool>;
#[doc = "Field `EOPFM` writer - desc EOPFM"]
pub type EOPFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `IEPIM` reader - desc IEPIM"]
pub type IEPIM_R = crate::BitReader<bool>;
#[doc = "Field `IEPIM` writer - desc IEPIM"]
pub type IEPIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `OEPIM` reader - desc OEPIM"]
pub type OEPIM_R = crate::BitReader<bool>;
#[doc = "Field `OEPIM` writer - desc OEPIM"]
pub type OEPIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `IISOIXFRM` reader - desc IISOIXFRM"]
pub type IISOIXFRM_R = crate::BitReader<bool>;
#[doc = "Field `IISOIXFRM` writer - desc IISOIXFRM"]
pub type IISOIXFRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `IPXFRM_INCOMPISOOUTM` reader - desc IPXFRM_INCOMPISOOUTM"]
pub type IPXFRM_INCOMPISOOUTM_R = crate::BitReader<bool>;
#[doc = "Field `IPXFRM_INCOMPISOOUTM` writer - desc IPXFRM_INCOMPISOOUTM"]
pub type IPXFRM_INCOMPISOOUTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `DATAFSUSPM` reader - desc DATAFSUSPM"]
pub type DATAFSUSPM_R = crate::BitReader<bool>;
#[doc = "Field `DATAFSUSPM` writer - desc DATAFSUSPM"]
pub type DATAFSUSPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `VBUSVIM` reader - desc VBUSVIM"]
pub type VBUSVIM_R = crate::BitReader<bool>;
#[doc = "Field `VBUSVIM` writer - desc VBUSVIM"]
pub type VBUSVIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `WKUIM` reader - desc WKUIM"]
pub type WKUIM_R = crate::BitReader<bool>;
#[doc = "Field `WKUIM` writer - desc WKUIM"]
pub type WKUIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - desc SOFM"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RXFNEM"]
    #[inline(always)]
    pub fn rxfnem(&self) -> RXFNEM_R {
        RXFNEM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc GINAKEFFM"]
    #[inline(always)]
    pub fn ginakeffm(&self) -> GINAKEFFM_R {
        GINAKEFFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc GOUTNAKEFFM"]
    #[inline(always)]
    pub fn goutnakeffm(&self) -> GOUTNAKEFFM_R {
        GOUTNAKEFFM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ESUSPM"]
    #[inline(always)]
    pub fn esuspm(&self) -> ESUSPM_R {
        ESUSPM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc USBSUSPM"]
    #[inline(always)]
    pub fn usbsuspm(&self) -> USBSUSPM_R {
        USBSUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc USBRSTM"]
    #[inline(always)]
    pub fn usbrstm(&self) -> USBRSTM_R {
        USBRSTM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc ENUMDNEM"]
    #[inline(always)]
    pub fn enumdnem(&self) -> ENUMDNEM_R {
        ENUMDNEM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc ISOODRPM"]
    #[inline(always)]
    pub fn isoodrpm(&self) -> ISOODRPM_R {
        ISOODRPM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EOPFM"]
    #[inline(always)]
    pub fn eopfm(&self) -> EOPFM_R {
        EOPFM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - desc IEPIM"]
    #[inline(always)]
    pub fn iepim(&self) -> IEPIM_R {
        IEPIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc OEPIM"]
    #[inline(always)]
    pub fn oepim(&self) -> OEPIM_R {
        OEPIM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc IISOIXFRM"]
    #[inline(always)]
    pub fn iisoixfrm(&self) -> IISOIXFRM_R {
        IISOIXFRM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc IPXFRM_INCOMPISOOUTM"]
    #[inline(always)]
    pub fn ipxfrm_incompisooutm(&self) -> IPXFRM_INCOMPISOOUTM_R {
        IPXFRM_INCOMPISOOUTM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc DATAFSUSPM"]
    #[inline(always)]
    pub fn datafsuspm(&self) -> DATAFSUSPM_R {
        DATAFSUSPM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 30 - desc VBUSVIM"]
    #[inline(always)]
    pub fn vbusvim(&self) -> VBUSVIM_R {
        VBUSVIM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc WKUIM"]
    #[inline(always)]
    pub fn wkuim(&self) -> WKUIM_R {
        WKUIM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - desc SOFM"]
    #[inline(always)]
    pub fn sofm(&mut self) -> SOFM_W<3> {
        SOFM_W::new(self)
    }
    #[doc = "Bit 4 - desc RXFNEM"]
    #[inline(always)]
    pub fn rxfnem(&mut self) -> RXFNEM_W<4> {
        RXFNEM_W::new(self)
    }
    #[doc = "Bit 6 - desc GINAKEFFM"]
    #[inline(always)]
    pub fn ginakeffm(&mut self) -> GINAKEFFM_W<6> {
        GINAKEFFM_W::new(self)
    }
    #[doc = "Bit 7 - desc GOUTNAKEFFM"]
    #[inline(always)]
    pub fn goutnakeffm(&mut self) -> GOUTNAKEFFM_W<7> {
        GOUTNAKEFFM_W::new(self)
    }
    #[doc = "Bit 10 - desc ESUSPM"]
    #[inline(always)]
    pub fn esuspm(&mut self) -> ESUSPM_W<10> {
        ESUSPM_W::new(self)
    }
    #[doc = "Bit 11 - desc USBSUSPM"]
    #[inline(always)]
    pub fn usbsuspm(&mut self) -> USBSUSPM_W<11> {
        USBSUSPM_W::new(self)
    }
    #[doc = "Bit 12 - desc USBRSTM"]
    #[inline(always)]
    pub fn usbrstm(&mut self) -> USBRSTM_W<12> {
        USBRSTM_W::new(self)
    }
    #[doc = "Bit 13 - desc ENUMDNEM"]
    #[inline(always)]
    pub fn enumdnem(&mut self) -> ENUMDNEM_W<13> {
        ENUMDNEM_W::new(self)
    }
    #[doc = "Bit 14 - desc ISOODRPM"]
    #[inline(always)]
    pub fn isoodrpm(&mut self) -> ISOODRPM_W<14> {
        ISOODRPM_W::new(self)
    }
    #[doc = "Bit 15 - desc EOPFM"]
    #[inline(always)]
    pub fn eopfm(&mut self) -> EOPFM_W<15> {
        EOPFM_W::new(self)
    }
    #[doc = "Bit 18 - desc IEPIM"]
    #[inline(always)]
    pub fn iepim(&mut self) -> IEPIM_W<18> {
        IEPIM_W::new(self)
    }
    #[doc = "Bit 19 - desc OEPIM"]
    #[inline(always)]
    pub fn oepim(&mut self) -> OEPIM_W<19> {
        OEPIM_W::new(self)
    }
    #[doc = "Bit 20 - desc IISOIXFRM"]
    #[inline(always)]
    pub fn iisoixfrm(&mut self) -> IISOIXFRM_W<20> {
        IISOIXFRM_W::new(self)
    }
    #[doc = "Bit 21 - desc IPXFRM_INCOMPISOOUTM"]
    #[inline(always)]
    pub fn ipxfrm_incompisooutm(&mut self) -> IPXFRM_INCOMPISOOUTM_W<21> {
        IPXFRM_INCOMPISOOUTM_W::new(self)
    }
    #[doc = "Bit 22 - desc DATAFSUSPM"]
    #[inline(always)]
    pub fn datafsuspm(&mut self) -> DATAFSUSPM_W<22> {
        DATAFSUSPM_W::new(self)
    }
    #[doc = "Bit 30 - desc VBUSVIM"]
    #[inline(always)]
    pub fn vbusvim(&mut self) -> VBUSVIM_W<30> {
        VBUSVIM_W::new(self)
    }
    #[doc = "Bit 31 - desc WKUIM"]
    #[inline(always)]
    pub fn wkuim(&mut self) -> WKUIM_W<31> {
        WKUIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc GINTMSK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintmsk](index.html) module"]
pub struct GINTMSK_SPEC;
impl crate::RegisterSpec for GINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintmsk::R](R) reader structure"]
impl crate::Readable for GINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gintmsk::W](W) writer structure"]
impl crate::Writable for GINTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GINTMSK to value 0"]
impl crate::Resettable for GINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
