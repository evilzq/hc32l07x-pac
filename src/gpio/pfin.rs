#[doc = "Register `PFIN` reader"]
pub struct R(crate::R<PFIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PF00` reader - desc PF00"]
pub type PF00_R = crate::BitReader<bool>;
#[doc = "Field `PF01` reader - desc PF01"]
pub type PF01_R = crate::BitReader<bool>;
#[doc = "Field `PF02` reader - desc PF02"]
pub type PF02_R = crate::BitReader<bool>;
#[doc = "Field `PF03` reader - desc PF03"]
pub type PF03_R = crate::BitReader<bool>;
#[doc = "Field `PF04` reader - desc PF04"]
pub type PF04_R = crate::BitReader<bool>;
#[doc = "Field `PF05` reader - desc PF05"]
pub type PF05_R = crate::BitReader<bool>;
#[doc = "Field `PF06` reader - desc PF06"]
pub type PF06_R = crate::BitReader<bool>;
#[doc = "Field `PF07` reader - desc PF07"]
pub type PF07_R = crate::BitReader<bool>;
#[doc = "Field `PF08` reader - desc PF08"]
pub type PF08_R = crate::BitReader<bool>;
#[doc = "Field `PF09` reader - desc PF09"]
pub type PF09_R = crate::BitReader<bool>;
#[doc = "Field `PF10` reader - desc PF10"]
pub type PF10_R = crate::BitReader<bool>;
#[doc = "Field `PF11` reader - desc PF11"]
pub type PF11_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
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
#[doc = "desc PFIN\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfin](index.html) module"]
pub struct PFIN_SPEC;
impl crate::RegisterSpec for PFIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfin::R](R) reader structure"]
impl crate::Readable for PFIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PFIN to value 0"]
impl crate::Resettable for PFIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
