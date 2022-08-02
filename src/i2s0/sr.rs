#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXNE_L` reader - desc RXNE_L"]
pub type RXNE_L_R = crate::BitReader<bool>;
#[doc = "Field `TXE_L` reader - desc TXE_L"]
pub type TXE_L_R = crate::BitReader<bool>;
#[doc = "Field `UDR_L` reader - desc UDR_L"]
pub type UDR_L_R = crate::BitReader<bool>;
#[doc = "Field `UDR_R` reader - desc UDR_R"]
pub type UDR_R_R = crate::BitReader<bool>;
#[doc = "Field `OVR_L` reader - desc OVR_L"]
pub type OVR_L_R = crate::BitReader<bool>;
#[doc = "Field `BSY` reader - desc BSY"]
pub type BSY_R = crate::BitReader<bool>;
#[doc = "Field `FRE` reader - desc FRE"]
pub type FRE_R = crate::BitReader<bool>;
#[doc = "Field `OVR_R` reader - desc OVR_R"]
pub type OVR_R_R = crate::BitReader<bool>;
#[doc = "Field `RXNE_R` reader - desc RXNE_R"]
pub type RXNE_R_R = crate::BitReader<bool>;
#[doc = "Field `TXE_R` reader - desc TXE_R"]
pub type TXE_R_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - desc RXNE_L"]
    #[inline(always)]
    pub fn rxne_l(&self) -> RXNE_L_R {
        RXNE_L_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc TXE_L"]
    #[inline(always)]
    pub fn txe_l(&self) -> TXE_L_R {
        TXE_L_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc UDR_L"]
    #[inline(always)]
    pub fn udr_l(&self) -> UDR_L_R {
        UDR_L_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc UDR_R"]
    #[inline(always)]
    pub fn udr_r(&self) -> UDR_R_R {
        UDR_R_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - desc OVR_L"]
    #[inline(always)]
    pub fn ovr_l(&self) -> OVR_L_R {
        OVR_L_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc BSY"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc FRE"]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - desc OVR_R"]
    #[inline(always)]
    pub fn ovr_r(&self) -> OVR_R_R {
        OVR_R_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc RXNE_R"]
    #[inline(always)]
    pub fn rxne_r(&self) -> RXNE_R_R {
        RXNE_R_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc TXE_R"]
    #[inline(always)]
    pub fn txe_r(&self) -> TXE_R_R {
        TXE_R_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0x8002"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8002
    }
}
