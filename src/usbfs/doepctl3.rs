#[doc = "Register `DOEPCTL3` reader"]
pub struct R(crate::R<DOEPCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPCTL3` writer"]
pub struct W(crate::W<DOEPCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPCTL3_SPEC>;
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
impl From<crate::W<DOEPCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPSIZ` reader - desc MPSIZ"]
pub type MPSIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MPSIZ` writer - desc MPSIZ"]
pub type MPSIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPCTL3_SPEC, u8, u8, 2, O>;
#[doc = "Field `USBAEP` reader - desc USBAEP"]
pub type USBAEP_R = crate::BitReader<bool>;
#[doc = "Field `DPID` reader - desc DPID"]
pub type DPID_R = crate::BitReader<bool>;
#[doc = "Field `NAKSTS` reader - desc NAKSTS"]
pub type NAKSTS_R = crate::BitReader<bool>;
#[doc = "Field `EPTYP` reader - desc EPTYP"]
pub type EPTYP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNPM` reader - desc SNPM"]
pub type SNPM_R = crate::BitReader<bool>;
#[doc = "Field `SNPM` writer - desc SNPM"]
pub type SNPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL3_SPEC, bool, O>;
#[doc = "Field `STALL` reader - desc STALL"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - desc STALL"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL3_SPEC, bool, O>;
#[doc = "Field `CNAK` writer - desc CNAK"]
pub type CNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL3_SPEC, bool, O>;
#[doc = "Field `SNAK` writer - desc SNAK"]
pub type SNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL3_SPEC, bool, O>;
#[doc = "Field `SD0PID` reader - desc SD0PID"]
pub type SD0PID_R = crate::BitReader<bool>;
#[doc = "Field `SD0PID` writer - desc SD0PID"]
pub type SD0PID_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL3_SPEC, bool, O>;
#[doc = "Field `SD1PID` reader - desc SD1PID"]
pub type SD1PID_R = crate::BitReader<bool>;
#[doc = "Field `SD1PID` writer - desc SD1PID"]
pub type SD1PID_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL3_SPEC, bool, O>;
#[doc = "Field `EPDIS` reader - desc EPDIS"]
pub type EPDIS_R = crate::BitReader<bool>;
#[doc = "Field `EPENA` reader - desc EPENA"]
pub type EPENA_R = crate::BitReader<bool>;
#[doc = "Field `EPENA` writer - desc EPENA"]
pub type EPENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPCTL3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - desc MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - desc USBAEP"]
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc DPID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc NAKSTS"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - desc EPTYP"]
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - desc SNPM"]
    #[inline(always)]
    pub fn snpm(&self) -> SNPM_R {
        SNPM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc STALL"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - desc SD0PID"]
    #[inline(always)]
    pub fn sd0pid(&self) -> SD0PID_R {
        SD0PID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc SD1PID"]
    #[inline(always)]
    pub fn sd1pid(&self) -> SD1PID_R {
        SD1PID_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc EPDIS"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc EPENA"]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W<0> {
        MPSIZ_W::new(self)
    }
    #[doc = "Bit 20 - desc SNPM"]
    #[inline(always)]
    pub fn snpm(&mut self) -> SNPM_W<20> {
        SNPM_W::new(self)
    }
    #[doc = "Bit 21 - desc STALL"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<21> {
        STALL_W::new(self)
    }
    #[doc = "Bit 26 - desc CNAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<26> {
        CNAK_W::new(self)
    }
    #[doc = "Bit 27 - desc SNAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<27> {
        SNAK_W::new(self)
    }
    #[doc = "Bit 28 - desc SD0PID"]
    #[inline(always)]
    pub fn sd0pid(&mut self) -> SD0PID_W<28> {
        SD0PID_W::new(self)
    }
    #[doc = "Bit 29 - desc SD1PID"]
    #[inline(always)]
    pub fn sd1pid(&mut self) -> SD1PID_W<29> {
        SD1PID_W::new(self)
    }
    #[doc = "Bit 31 - desc EPENA"]
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<31> {
        EPENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc DOEPCTL3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl3](index.html) module"]
pub struct DOEPCTL3_SPEC;
impl crate::RegisterSpec for DOEPCTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepctl3::R](R) reader structure"]
impl crate::Readable for DOEPCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepctl3::W](W) writer structure"]
impl crate::Writable for DOEPCTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPCTL3 to value 0"]
impl crate::Resettable for DOEPCTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
