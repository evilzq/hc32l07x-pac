#[doc = "Register `PDADS` reader"]
pub struct R(crate::R<PDADS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDADS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDADS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDADS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDADS` writer"]
pub struct W(crate::W<PDADS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDADS_SPEC>;
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
impl From<crate::W<PDADS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDADS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD00` reader - desc PD00"]
pub type PD00_R = crate::BitReader<bool>;
#[doc = "Field `PD00` writer - desc PD00"]
pub type PD00_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD01` reader - desc PD01"]
pub type PD01_R = crate::BitReader<bool>;
#[doc = "Field `PD01` writer - desc PD01"]
pub type PD01_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD02` reader - desc PD02"]
pub type PD02_R = crate::BitReader<bool>;
#[doc = "Field `PD02` writer - desc PD02"]
pub type PD02_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD03` reader - desc PD03"]
pub type PD03_R = crate::BitReader<bool>;
#[doc = "Field `PD03` writer - desc PD03"]
pub type PD03_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD04` reader - desc PD04"]
pub type PD04_R = crate::BitReader<bool>;
#[doc = "Field `PD04` writer - desc PD04"]
pub type PD04_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD05` reader - desc PD05"]
pub type PD05_R = crate::BitReader<bool>;
#[doc = "Field `PD05` writer - desc PD05"]
pub type PD05_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD06` reader - desc PD06"]
pub type PD06_R = crate::BitReader<bool>;
#[doc = "Field `PD06` writer - desc PD06"]
pub type PD06_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD07` reader - desc PD07"]
pub type PD07_R = crate::BitReader<bool>;
#[doc = "Field `PD07` writer - desc PD07"]
pub type PD07_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD08` reader - desc PD08"]
pub type PD08_R = crate::BitReader<bool>;
#[doc = "Field `PD08` writer - desc PD08"]
pub type PD08_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD09` reader - desc PD09"]
pub type PD09_R = crate::BitReader<bool>;
#[doc = "Field `PD09` writer - desc PD09"]
pub type PD09_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD10` reader - desc PD10"]
pub type PD10_R = crate::BitReader<bool>;
#[doc = "Field `PD10` writer - desc PD10"]
pub type PD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD11` reader - desc PD11"]
pub type PD11_R = crate::BitReader<bool>;
#[doc = "Field `PD11` writer - desc PD11"]
pub type PD11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD12` reader - desc PD12"]
pub type PD12_R = crate::BitReader<bool>;
#[doc = "Field `PD12` writer - desc PD12"]
pub type PD12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD13` reader - desc PD13"]
pub type PD13_R = crate::BitReader<bool>;
#[doc = "Field `PD13` writer - desc PD13"]
pub type PD13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD14` reader - desc PD14"]
pub type PD14_R = crate::BitReader<bool>;
#[doc = "Field `PD14` writer - desc PD14"]
pub type PD14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `PD15` reader - desc PD15"]
pub type PD15_R = crate::BitReader<bool>;
#[doc = "Field `PD15` writer - desc PD15"]
pub type PD15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDADS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc PD00"]
    #[inline(always)]
    pub fn pd00(&self) -> PD00_R {
        PD00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PD01"]
    #[inline(always)]
    pub fn pd01(&self) -> PD01_R {
        PD01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PD02"]
    #[inline(always)]
    pub fn pd02(&self) -> PD02_R {
        PD02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PD03"]
    #[inline(always)]
    pub fn pd03(&self) -> PD03_R {
        PD03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PD04"]
    #[inline(always)]
    pub fn pd04(&self) -> PD04_R {
        PD04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PD05"]
    #[inline(always)]
    pub fn pd05(&self) -> PD05_R {
        PD05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PD06"]
    #[inline(always)]
    pub fn pd06(&self) -> PD06_R {
        PD06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PD07"]
    #[inline(always)]
    pub fn pd07(&self) -> PD07_R {
        PD07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PD08"]
    #[inline(always)]
    pub fn pd08(&self) -> PD08_R {
        PD08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PD09"]
    #[inline(always)]
    pub fn pd09(&self) -> PD09_R {
        PD09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PD10"]
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc PD11"]
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PD12"]
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PD13"]
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PD14"]
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PD15"]
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PD00"]
    #[inline(always)]
    pub fn pd00(&mut self) -> PD00_W<0> {
        PD00_W::new(self)
    }
    #[doc = "Bit 1 - desc PD01"]
    #[inline(always)]
    pub fn pd01(&mut self) -> PD01_W<1> {
        PD01_W::new(self)
    }
    #[doc = "Bit 2 - desc PD02"]
    #[inline(always)]
    pub fn pd02(&mut self) -> PD02_W<2> {
        PD02_W::new(self)
    }
    #[doc = "Bit 3 - desc PD03"]
    #[inline(always)]
    pub fn pd03(&mut self) -> PD03_W<3> {
        PD03_W::new(self)
    }
    #[doc = "Bit 4 - desc PD04"]
    #[inline(always)]
    pub fn pd04(&mut self) -> PD04_W<4> {
        PD04_W::new(self)
    }
    #[doc = "Bit 5 - desc PD05"]
    #[inline(always)]
    pub fn pd05(&mut self) -> PD05_W<5> {
        PD05_W::new(self)
    }
    #[doc = "Bit 6 - desc PD06"]
    #[inline(always)]
    pub fn pd06(&mut self) -> PD06_W<6> {
        PD06_W::new(self)
    }
    #[doc = "Bit 7 - desc PD07"]
    #[inline(always)]
    pub fn pd07(&mut self) -> PD07_W<7> {
        PD07_W::new(self)
    }
    #[doc = "Bit 8 - desc PD08"]
    #[inline(always)]
    pub fn pd08(&mut self) -> PD08_W<8> {
        PD08_W::new(self)
    }
    #[doc = "Bit 9 - desc PD09"]
    #[inline(always)]
    pub fn pd09(&mut self) -> PD09_W<9> {
        PD09_W::new(self)
    }
    #[doc = "Bit 10 - desc PD10"]
    #[inline(always)]
    pub fn pd10(&mut self) -> PD10_W<10> {
        PD10_W::new(self)
    }
    #[doc = "Bit 11 - desc PD11"]
    #[inline(always)]
    pub fn pd11(&mut self) -> PD11_W<11> {
        PD11_W::new(self)
    }
    #[doc = "Bit 12 - desc PD12"]
    #[inline(always)]
    pub fn pd12(&mut self) -> PD12_W<12> {
        PD12_W::new(self)
    }
    #[doc = "Bit 13 - desc PD13"]
    #[inline(always)]
    pub fn pd13(&mut self) -> PD13_W<13> {
        PD13_W::new(self)
    }
    #[doc = "Bit 14 - desc PD14"]
    #[inline(always)]
    pub fn pd14(&mut self) -> PD14_W<14> {
        PD14_W::new(self)
    }
    #[doc = "Bit 15 - desc PD15"]
    #[inline(always)]
    pub fn pd15(&mut self) -> PD15_W<15> {
        PD15_W::new(self)
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
#[doc = "desc PDADS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdads](index.html) module"]
pub struct PDADS_SPEC;
impl crate::RegisterSpec for PDADS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdads::R](R) reader structure"]
impl crate::Readable for PDADS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdads::W](W) writer structure"]
impl crate::Writable for PDADS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDADS to value 0xffff_ffff"]
impl crate::Resettable for PDADS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
