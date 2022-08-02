#[doc = "Register `TCMD` reader"]
pub struct R(crate::R<TCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCMD` writer"]
pub struct W(crate::W<TCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCMD_SPEC>;
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
impl From<crate::W<TCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSA` reader - desc TSA"]
pub type TSA_R = crate::BitReader<bool>;
#[doc = "Field `TSA` writer - desc TSA"]
pub type TSA_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCMD_SPEC, bool, O>;
#[doc = "Field `TSALL` reader - desc TSALL"]
pub type TSALL_R = crate::BitReader<bool>;
#[doc = "Field `TSALL` writer - desc TSALL"]
pub type TSALL_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCMD_SPEC, bool, O>;
#[doc = "Field `TSONE` reader - desc TSONE"]
pub type TSONE_R = crate::BitReader<bool>;
#[doc = "Field `TSONE` writer - desc TSONE"]
pub type TSONE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCMD_SPEC, bool, O>;
#[doc = "Field `TPA` reader - desc TPA"]
pub type TPA_R = crate::BitReader<bool>;
#[doc = "Field `TPA` writer - desc TPA"]
pub type TPA_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCMD_SPEC, bool, O>;
#[doc = "Field `TPE` reader - desc TPE"]
pub type TPE_R = crate::BitReader<bool>;
#[doc = "Field `TPE` writer - desc TPE"]
pub type TPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCMD_SPEC, bool, O>;
#[doc = "Field `STBY` reader - desc STBY"]
pub type STBY_R = crate::BitReader<bool>;
#[doc = "Field `STBY` writer - desc STBY"]
pub type STBY_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCMD_SPEC, bool, O>;
#[doc = "Field `LOM` reader - desc LOM"]
pub type LOM_R = crate::BitReader<bool>;
#[doc = "Field `LOM` writer - desc LOM"]
pub type LOM_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCMD_SPEC, bool, O>;
#[doc = "Field `TBSEL` reader - desc TBSEL"]
pub type TBSEL_R = crate::BitReader<bool>;
#[doc = "Field `TBSEL` writer - desc TBSEL"]
pub type TBSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc TSA"]
    #[inline(always)]
    pub fn tsa(&self) -> TSA_R {
        TSA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TSALL"]
    #[inline(always)]
    pub fn tsall(&self) -> TSALL_R {
        TSALL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TSONE"]
    #[inline(always)]
    pub fn tsone(&self) -> TSONE_R {
        TSONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TPA"]
    #[inline(always)]
    pub fn tpa(&self) -> TPA_R {
        TPA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc TPE"]
    #[inline(always)]
    pub fn tpe(&self) -> TPE_R {
        TPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc STBY"]
    #[inline(always)]
    pub fn stby(&self) -> STBY_R {
        STBY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc LOM"]
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TBSEL"]
    #[inline(always)]
    pub fn tbsel(&self) -> TBSEL_R {
        TBSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc TSA"]
    #[inline(always)]
    pub fn tsa(&mut self) -> TSA_W<0> {
        TSA_W::new(self)
    }
    #[doc = "Bit 1 - desc TSALL"]
    #[inline(always)]
    pub fn tsall(&mut self) -> TSALL_W<1> {
        TSALL_W::new(self)
    }
    #[doc = "Bit 2 - desc TSONE"]
    #[inline(always)]
    pub fn tsone(&mut self) -> TSONE_W<2> {
        TSONE_W::new(self)
    }
    #[doc = "Bit 3 - desc TPA"]
    #[inline(always)]
    pub fn tpa(&mut self) -> TPA_W<3> {
        TPA_W::new(self)
    }
    #[doc = "Bit 4 - desc TPE"]
    #[inline(always)]
    pub fn tpe(&mut self) -> TPE_W<4> {
        TPE_W::new(self)
    }
    #[doc = "Bit 5 - desc STBY"]
    #[inline(always)]
    pub fn stby(&mut self) -> STBY_W<5> {
        STBY_W::new(self)
    }
    #[doc = "Bit 6 - desc LOM"]
    #[inline(always)]
    pub fn lom(&mut self) -> LOM_W<6> {
        LOM_W::new(self)
    }
    #[doc = "Bit 7 - desc TBSEL"]
    #[inline(always)]
    pub fn tbsel(&mut self) -> TBSEL_W<7> {
        TBSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TCMD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcmd](index.html) module"]
pub struct TCMD_SPEC;
impl crate::RegisterSpec for TCMD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tcmd::R](R) reader structure"]
impl crate::Readable for TCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcmd::W](W) writer structure"]
impl crate::Writable for TCMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCMD to value 0"]
impl crate::Resettable for TCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
