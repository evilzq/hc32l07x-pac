#[doc = "Register `PEFIE` reader"]
pub struct R(crate::R<PEFIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEFIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEFIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEFIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEFIE` writer"]
pub struct W(crate::W<PEFIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEFIE_SPEC>;
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
impl From<crate::W<PEFIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEFIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PE00` reader - desc PE00"]
pub type PE00_R = crate::BitReader<bool>;
#[doc = "Field `PE00` writer - desc PE00"]
pub type PE00_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE01` reader - desc PE01"]
pub type PE01_R = crate::BitReader<bool>;
#[doc = "Field `PE01` writer - desc PE01"]
pub type PE01_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE02` reader - desc PE02"]
pub type PE02_R = crate::BitReader<bool>;
#[doc = "Field `PE02` writer - desc PE02"]
pub type PE02_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE03` reader - desc PE03"]
pub type PE03_R = crate::BitReader<bool>;
#[doc = "Field `PE03` writer - desc PE03"]
pub type PE03_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE04` reader - desc PE04"]
pub type PE04_R = crate::BitReader<bool>;
#[doc = "Field `PE04` writer - desc PE04"]
pub type PE04_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE05` reader - desc PE05"]
pub type PE05_R = crate::BitReader<bool>;
#[doc = "Field `PE05` writer - desc PE05"]
pub type PE05_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE06` reader - desc PE06"]
pub type PE06_R = crate::BitReader<bool>;
#[doc = "Field `PE06` writer - desc PE06"]
pub type PE06_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE07` reader - desc PE07"]
pub type PE07_R = crate::BitReader<bool>;
#[doc = "Field `PE07` writer - desc PE07"]
pub type PE07_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE08` reader - desc PE08"]
pub type PE08_R = crate::BitReader<bool>;
#[doc = "Field `PE08` writer - desc PE08"]
pub type PE08_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE09` reader - desc PE09"]
pub type PE09_R = crate::BitReader<bool>;
#[doc = "Field `PE09` writer - desc PE09"]
pub type PE09_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE10` reader - desc PE10"]
pub type PE10_R = crate::BitReader<bool>;
#[doc = "Field `PE10` writer - desc PE10"]
pub type PE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE11` reader - desc PE11"]
pub type PE11_R = crate::BitReader<bool>;
#[doc = "Field `PE11` writer - desc PE11"]
pub type PE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE12` reader - desc PE12"]
pub type PE12_R = crate::BitReader<bool>;
#[doc = "Field `PE12` writer - desc PE12"]
pub type PE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE13` reader - desc PE13"]
pub type PE13_R = crate::BitReader<bool>;
#[doc = "Field `PE13` writer - desc PE13"]
pub type PE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE14` reader - desc PE14"]
pub type PE14_R = crate::BitReader<bool>;
#[doc = "Field `PE14` writer - desc PE14"]
pub type PE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `PE15` reader - desc PE15"]
pub type PE15_R = crate::BitReader<bool>;
#[doc = "Field `PE15` writer - desc PE15"]
pub type PE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEFIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc PE00"]
    #[inline(always)]
    pub fn pe00(&self) -> PE00_R {
        PE00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PE01"]
    #[inline(always)]
    pub fn pe01(&self) -> PE01_R {
        PE01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PE02"]
    #[inline(always)]
    pub fn pe02(&self) -> PE02_R {
        PE02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PE03"]
    #[inline(always)]
    pub fn pe03(&self) -> PE03_R {
        PE03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PE04"]
    #[inline(always)]
    pub fn pe04(&self) -> PE04_R {
        PE04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PE05"]
    #[inline(always)]
    pub fn pe05(&self) -> PE05_R {
        PE05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PE06"]
    #[inline(always)]
    pub fn pe06(&self) -> PE06_R {
        PE06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PE07"]
    #[inline(always)]
    pub fn pe07(&self) -> PE07_R {
        PE07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PE08"]
    #[inline(always)]
    pub fn pe08(&self) -> PE08_R {
        PE08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PE09"]
    #[inline(always)]
    pub fn pe09(&self) -> PE09_R {
        PE09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PE10"]
    #[inline(always)]
    pub fn pe10(&self) -> PE10_R {
        PE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc PE11"]
    #[inline(always)]
    pub fn pe11(&self) -> PE11_R {
        PE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc PE12"]
    #[inline(always)]
    pub fn pe12(&self) -> PE12_R {
        PE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc PE13"]
    #[inline(always)]
    pub fn pe13(&self) -> PE13_R {
        PE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc PE14"]
    #[inline(always)]
    pub fn pe14(&self) -> PE14_R {
        PE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc PE15"]
    #[inline(always)]
    pub fn pe15(&self) -> PE15_R {
        PE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PE00"]
    #[inline(always)]
    pub fn pe00(&mut self) -> PE00_W<0> {
        PE00_W::new(self)
    }
    #[doc = "Bit 1 - desc PE01"]
    #[inline(always)]
    pub fn pe01(&mut self) -> PE01_W<1> {
        PE01_W::new(self)
    }
    #[doc = "Bit 2 - desc PE02"]
    #[inline(always)]
    pub fn pe02(&mut self) -> PE02_W<2> {
        PE02_W::new(self)
    }
    #[doc = "Bit 3 - desc PE03"]
    #[inline(always)]
    pub fn pe03(&mut self) -> PE03_W<3> {
        PE03_W::new(self)
    }
    #[doc = "Bit 4 - desc PE04"]
    #[inline(always)]
    pub fn pe04(&mut self) -> PE04_W<4> {
        PE04_W::new(self)
    }
    #[doc = "Bit 5 - desc PE05"]
    #[inline(always)]
    pub fn pe05(&mut self) -> PE05_W<5> {
        PE05_W::new(self)
    }
    #[doc = "Bit 6 - desc PE06"]
    #[inline(always)]
    pub fn pe06(&mut self) -> PE06_W<6> {
        PE06_W::new(self)
    }
    #[doc = "Bit 7 - desc PE07"]
    #[inline(always)]
    pub fn pe07(&mut self) -> PE07_W<7> {
        PE07_W::new(self)
    }
    #[doc = "Bit 8 - desc PE08"]
    #[inline(always)]
    pub fn pe08(&mut self) -> PE08_W<8> {
        PE08_W::new(self)
    }
    #[doc = "Bit 9 - desc PE09"]
    #[inline(always)]
    pub fn pe09(&mut self) -> PE09_W<9> {
        PE09_W::new(self)
    }
    #[doc = "Bit 10 - desc PE10"]
    #[inline(always)]
    pub fn pe10(&mut self) -> PE10_W<10> {
        PE10_W::new(self)
    }
    #[doc = "Bit 11 - desc PE11"]
    #[inline(always)]
    pub fn pe11(&mut self) -> PE11_W<11> {
        PE11_W::new(self)
    }
    #[doc = "Bit 12 - desc PE12"]
    #[inline(always)]
    pub fn pe12(&mut self) -> PE12_W<12> {
        PE12_W::new(self)
    }
    #[doc = "Bit 13 - desc PE13"]
    #[inline(always)]
    pub fn pe13(&mut self) -> PE13_W<13> {
        PE13_W::new(self)
    }
    #[doc = "Bit 14 - desc PE14"]
    #[inline(always)]
    pub fn pe14(&mut self) -> PE14_W<14> {
        PE14_W::new(self)
    }
    #[doc = "Bit 15 - desc PE15"]
    #[inline(always)]
    pub fn pe15(&mut self) -> PE15_W<15> {
        PE15_W::new(self)
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
#[doc = "desc PEFIE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pefie](index.html) module"]
pub struct PEFIE_SPEC;
impl crate::RegisterSpec for PEFIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pefie::R](R) reader structure"]
impl crate::Readable for PEFIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pefie::W](W) writer structure"]
impl crate::Writable for PEFIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEFIE to value 0xffff_ffff"]
impl crate::Resettable for PEFIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
