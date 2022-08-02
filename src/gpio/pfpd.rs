#[doc = "Register `PFPD` reader"]
pub struct R(crate::R<PFPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFPD` writer"]
pub struct W(crate::W<PFPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFPD_SPEC>;
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
impl From<crate::W<PFPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PF00` reader - desc PF00"]
pub type PF00_R = crate::BitReader<bool>;
#[doc = "Field `PF00` writer - desc PF00"]
pub type PF00_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFPD_SPEC, bool, O>;
#[doc = "Field `PF01` reader - desc PF01"]
pub type PF01_R = crate::BitReader<bool>;
#[doc = "Field `PF01` writer - desc PF01"]
pub type PF01_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFPD_SPEC, bool, O>;
#[doc = "Field `PF02` reader - desc PF02"]
pub type PF02_R = crate::BitReader<bool>;
#[doc = "Field `PF02` writer - desc PF02"]
pub type PF02_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFPD_SPEC, bool, O>;
#[doc = "Field `PF03` reader - desc PF03"]
pub type PF03_R = crate::BitReader<bool>;
#[doc = "Field `PF03` writer - desc PF03"]
pub type PF03_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFPD_SPEC, bool, O>;
#[doc = "Field `PF04` reader - desc PF04"]
pub type PF04_R = crate::BitReader<bool>;
#[doc = "Field `PF04` writer - desc PF04"]
pub type PF04_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFPD_SPEC, bool, O>;
#[doc = "Field `PF05` reader - desc PF05"]
pub type PF05_R = crate::BitReader<bool>;
#[doc = "Field `PF05` writer - desc PF05"]
pub type PF05_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFPD_SPEC, bool, O>;
#[doc = "Field `PF06` reader - desc PF06"]
pub type PF06_R = crate::BitReader<bool>;
#[doc = "Field `PF06` writer - desc PF06"]
pub type PF06_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFPD_SPEC, bool, O>;
#[doc = "Field `PF07` reader - desc PF07"]
pub type PF07_R = crate::BitReader<bool>;
#[doc = "Field `PF07` writer - desc PF07"]
pub type PF07_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFPD_SPEC, bool, O>;
#[doc = "Field `PF08` reader - desc PF08"]
pub type PF08_R = crate::BitReader<bool>;
#[doc = "Field `PF08` writer - desc PF08"]
pub type PF08_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFPD_SPEC, bool, O>;
#[doc = "Field `PF09` reader - desc PF09"]
pub type PF09_R = crate::BitReader<bool>;
#[doc = "Field `PF09` writer - desc PF09"]
pub type PF09_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFPD_SPEC, bool, O>;
#[doc = "Field `PF10` reader - desc PF10"]
pub type PF10_R = crate::BitReader<bool>;
#[doc = "Field `PF10` writer - desc PF10"]
pub type PF10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFPD_SPEC, bool, O>;
#[doc = "Field `PF11` reader - desc PF11"]
pub type PF11_R = crate::BitReader<bool>;
#[doc = "Field `PF11` writer - desc PF11"]
pub type PF11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFPD_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFPD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc PF00"]
    #[inline(always)]
    pub fn pf00(&self) -> PF00_R {
        PF00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PF01"]
    #[inline(always)]
    pub fn pf01(&self) -> PF01_R {
        PF01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc PF02"]
    #[inline(always)]
    pub fn pf02(&self) -> PF02_R {
        PF02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc PF03"]
    #[inline(always)]
    pub fn pf03(&self) -> PF03_R {
        PF03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc PF04"]
    #[inline(always)]
    pub fn pf04(&self) -> PF04_R {
        PF04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc PF05"]
    #[inline(always)]
    pub fn pf05(&self) -> PF05_R {
        PF05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PF06"]
    #[inline(always)]
    pub fn pf06(&self) -> PF06_R {
        PF06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PF07"]
    #[inline(always)]
    pub fn pf07(&self) -> PF07_R {
        PF07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PF08"]
    #[inline(always)]
    pub fn pf08(&self) -> PF08_R {
        PF08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PF09"]
    #[inline(always)]
    pub fn pf09(&self) -> PF09_R {
        PF09_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PF10"]
    #[inline(always)]
    pub fn pf10(&self) -> PF10_R {
        PF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc PF11"]
    #[inline(always)]
    pub fn pf11(&self) -> PF11_R {
        PF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PF00"]
    #[inline(always)]
    pub fn pf00(&mut self) -> PF00_W<0> {
        PF00_W::new(self)
    }
    #[doc = "Bit 1 - desc PF01"]
    #[inline(always)]
    pub fn pf01(&mut self) -> PF01_W<1> {
        PF01_W::new(self)
    }
    #[doc = "Bit 2 - desc PF02"]
    #[inline(always)]
    pub fn pf02(&mut self) -> PF02_W<2> {
        PF02_W::new(self)
    }
    #[doc = "Bit 3 - desc PF03"]
    #[inline(always)]
    pub fn pf03(&mut self) -> PF03_W<3> {
        PF03_W::new(self)
    }
    #[doc = "Bit 4 - desc PF04"]
    #[inline(always)]
    pub fn pf04(&mut self) -> PF04_W<4> {
        PF04_W::new(self)
    }
    #[doc = "Bit 5 - desc PF05"]
    #[inline(always)]
    pub fn pf05(&mut self) -> PF05_W<5> {
        PF05_W::new(self)
    }
    #[doc = "Bit 6 - desc PF06"]
    #[inline(always)]
    pub fn pf06(&mut self) -> PF06_W<6> {
        PF06_W::new(self)
    }
    #[doc = "Bit 7 - desc PF07"]
    #[inline(always)]
    pub fn pf07(&mut self) -> PF07_W<7> {
        PF07_W::new(self)
    }
    #[doc = "Bit 8 - desc PF08"]
    #[inline(always)]
    pub fn pf08(&mut self) -> PF08_W<8> {
        PF08_W::new(self)
    }
    #[doc = "Bit 9 - desc PF09"]
    #[inline(always)]
    pub fn pf09(&mut self) -> PF09_W<9> {
        PF09_W::new(self)
    }
    #[doc = "Bit 10 - desc PF10"]
    #[inline(always)]
    pub fn pf10(&mut self) -> PF10_W<10> {
        PF10_W::new(self)
    }
    #[doc = "Bit 11 - desc PF11"]
    #[inline(always)]
    pub fn pf11(&mut self) -> PF11_W<11> {
        PF11_W::new(self)
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
#[doc = "desc PFPD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfpd](index.html) module"]
pub struct PFPD_SPEC;
impl crate::RegisterSpec for PFPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfpd::R](R) reader structure"]
impl crate::Readable for PFPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfpd::W](W) writer structure"]
impl crate::Writable for PFPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PFPD to value 0xffff_ffff"]
impl crate::Resettable for PFPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
