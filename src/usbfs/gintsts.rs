#[doc = "Register `GINTSTS` reader"]
pub struct R(crate::R<GINTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SOF` reader - desc SOF"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `RXFLNE` reader - desc RXFLNE"]
pub type RXFLNE_R = crate::BitReader<bool>;
#[doc = "Field `NPTXFE` reader - desc NPTXFE"]
pub type NPTXFE_R = crate::BitReader<bool>;
#[doc = "Field `GINAKEFF` reader - desc GINAKEFF"]
pub type GINAKEFF_R = crate::BitReader<bool>;
#[doc = "Field `GOUTNAKEFF` reader - desc GOUTNAKEFF"]
pub type GOUTNAKEFF_R = crate::BitReader<bool>;
#[doc = "Field `ESUSP` reader - desc ESUSP"]
pub type ESUSP_R = crate::BitReader<bool>;
#[doc = "Field `USBSUSP` reader - desc USBSUSP"]
pub type USBSUSP_R = crate::BitReader<bool>;
#[doc = "Field `USBRST` reader - desc USBRST"]
pub type USBRST_R = crate::BitReader<bool>;
#[doc = "Field `ENUMDNE` reader - desc ENUMDNE"]
pub type ENUMDNE_R = crate::BitReader<bool>;
#[doc = "Field `ISOODRP` reader - desc ISOODRP"]
pub type ISOODRP_R = crate::BitReader<bool>;
#[doc = "Field `EOPF` reader - desc EOPF"]
pub type EOPF_R = crate::BitReader<bool>;
#[doc = "Field `IEPINT` reader - desc IEPINT"]
pub type IEPINT_R = crate::BitReader<bool>;
#[doc = "Field `OEPINT` reader - desc OEPINT"]
pub type OEPINT_R = crate::BitReader<bool>;
#[doc = "Field `IISOIXFR` reader - desc IISOIXFR"]
pub type IISOIXFR_R = crate::BitReader<bool>;
#[doc = "Field `IPXFR_INCOMPISO` reader - desc IPXFR_INCOMPISO"]
pub type IPXFR_INCOMPISO_R = crate::BitReader<bool>;
#[doc = "Field `DATAFSUSP` reader - desc DATAFSUSP"]
pub type DATAFSUSP_R = crate::BitReader<bool>;
#[doc = "Field `VBUSVINT` reader - desc VBUSVINT"]
pub type VBUSVINT_R = crate::BitReader<bool>;
#[doc = "Field `WKUINT` reader - desc WKUINT"]
pub type WKUINT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 3 - desc SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc RXFLNE"]
    #[inline(always)]
    pub fn rxflne(&self) -> RXFLNE_R {
        RXFLNE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc NPTXFE"]
    #[inline(always)]
    pub fn nptxfe(&self) -> NPTXFE_R {
        NPTXFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc GINAKEFF"]
    #[inline(always)]
    pub fn ginakeff(&self) -> GINAKEFF_R {
        GINAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc GOUTNAKEFF"]
    #[inline(always)]
    pub fn goutnakeff(&self) -> GOUTNAKEFF_R {
        GOUTNAKEFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ESUSP"]
    #[inline(always)]
    pub fn esusp(&self) -> ESUSP_R {
        ESUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc USBSUSP"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc ENUMDNE"]
    #[inline(always)]
    pub fn enumdne(&self) -> ENUMDNE_R {
        ENUMDNE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc ISOODRP"]
    #[inline(always)]
    pub fn isoodrp(&self) -> ISOODRP_R {
        ISOODRP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc EOPF"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - desc IEPINT"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc OEPINT"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc IISOIXFR"]
    #[inline(always)]
    pub fn iisoixfr(&self) -> IISOIXFR_R {
        IISOIXFR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc IPXFR_INCOMPISO"]
    #[inline(always)]
    pub fn ipxfr_incompiso(&self) -> IPXFR_INCOMPISO_R {
        IPXFR_INCOMPISO_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc DATAFSUSP"]
    #[inline(always)]
    pub fn datafsusp(&self) -> DATAFSUSP_R {
        DATAFSUSP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 30 - desc VBUSVINT"]
    #[inline(always)]
    pub fn vbusvint(&self) -> VBUSVINT_R {
        VBUSVINT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc WKUINT"]
    #[inline(always)]
    pub fn wkuint(&self) -> WKUINT_R {
        WKUINT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "desc GINTSTS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintsts](index.html) module"]
pub struct GINTSTS_SPEC;
impl crate::RegisterSpec for GINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintsts::R](R) reader structure"]
impl crate::Readable for GINTSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GINTSTS to value 0x1400_0020"]
impl crate::Resettable for GINTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1400_0020
    }
}
