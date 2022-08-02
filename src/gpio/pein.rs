#[doc = "Register `PEIN` reader"]
pub struct R(crate::R<PEIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PE00` reader - desc PE00"]
pub type PE00_R = crate::BitReader<bool>;
#[doc = "Field `PE01` reader - desc PE01"]
pub type PE01_R = crate::BitReader<bool>;
#[doc = "Field `PE02` reader - desc PE02"]
pub type PE02_R = crate::BitReader<bool>;
#[doc = "Field `PE03` reader - desc PE03"]
pub type PE03_R = crate::BitReader<bool>;
#[doc = "Field `PE04` reader - desc PE04"]
pub type PE04_R = crate::BitReader<bool>;
#[doc = "Field `PE05` reader - desc PE05"]
pub type PE05_R = crate::BitReader<bool>;
#[doc = "Field `PE06` reader - desc PE06"]
pub type PE06_R = crate::BitReader<bool>;
#[doc = "Field `PE07` reader - desc PE07"]
pub type PE07_R = crate::BitReader<bool>;
#[doc = "Field `PE08` reader - desc PE08"]
pub type PE08_R = crate::BitReader<bool>;
#[doc = "Field `PE09` reader - desc PE09"]
pub type PE09_R = crate::BitReader<bool>;
#[doc = "Field `PE10` reader - desc PE10"]
pub type PE10_R = crate::BitReader<bool>;
#[doc = "Field `PE11` reader - desc PE11"]
pub type PE11_R = crate::BitReader<bool>;
#[doc = "Field `PE12` reader - desc PE12"]
pub type PE12_R = crate::BitReader<bool>;
#[doc = "Field `PE13` reader - desc PE13"]
pub type PE13_R = crate::BitReader<bool>;
#[doc = "Field `PE14` reader - desc PE14"]
pub type PE14_R = crate::BitReader<bool>;
#[doc = "Field `PE15` reader - desc PE15"]
pub type PE15_R = crate::BitReader<bool>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
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
#[doc = "desc PEIN\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pein](index.html) module"]
pub struct PEIN_SPEC;
impl crate::RegisterSpec for PEIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pein::R](R) reader structure"]
impl crate::Readable for PEIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PEIN to value 0"]
impl crate::Resettable for PEIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
