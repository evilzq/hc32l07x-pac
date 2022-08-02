#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OKIE` reader - desc OKIE"]
pub type OKIE_R = crate::BitReader<bool>;
#[doc = "Field `OKIE` writer - desc OKIE"]
pub type OKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WARNIE` reader - desc WARNIE"]
pub type WARNIE_R = crate::BitReader<bool>;
#[doc = "Field `WARNIE` writer - desc WARNIE"]
pub type WARNIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ERRIE` reader - desc ERRIE"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - desc ERRIE"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `UDFIE` reader - desc UDFIE"]
pub type UDFIE_R = crate::BitReader<bool>;
#[doc = "Field `UDFIE` writer - desc UDFIE"]
pub type UDFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CAPIE` reader - desc CAPIE"]
pub type CAPIE_R = crate::BitReader<bool>;
#[doc = "Field `CAPIE` writer - desc CAPIE"]
pub type CAPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CEN` reader - desc CEN"]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - desc CEN"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `AUTO_TRIM_EN` reader - desc AUTO_TRIM_EN"]
pub type AUTO_TRIM_EN_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_TRIM_EN` writer - desc AUTO_TRIM_EN"]
pub type AUTO_TRIM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SW_SYNC` reader - desc SW_SYNC"]
pub type SW_SYNC_R = crate::BitReader<bool>;
#[doc = "Field `SW_SYNC` writer - desc SW_SYNC"]
pub type SW_SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TRIM` reader - desc TRIM"]
pub type TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM` writer - desc TRIM"]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 7, O>;
#[doc = "Field `TIM_EN` reader - desc TIM_EN"]
pub type TIM_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM_EN` writer - desc TIM_EN"]
pub type TIM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc OKIE"]
    #[inline(always)]
    pub fn okie(&self) -> OKIE_R {
        OKIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc WARNIE"]
    #[inline(always)]
    pub fn warnie(&self) -> WARNIE_R {
        WARNIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc ERRIE"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc UDFIE"]
    #[inline(always)]
    pub fn udfie(&self) -> UDFIE_R {
        UDFIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc CAPIE"]
    #[inline(always)]
    pub fn capie(&self) -> CAPIE_R {
        CAPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc CEN"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc AUTO_TRIM_EN"]
    #[inline(always)]
    pub fn auto_trim_en(&self) -> AUTO_TRIM_EN_R {
        AUTO_TRIM_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SW_SYNC"]
    #[inline(always)]
    pub fn sw_sync(&self) -> SW_SYNC_R {
        SW_SYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - desc TIM_EN"]
    #[inline(always)]
    pub fn tim_en(&self) -> TIM_EN_R {
        TIM_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc OKIE"]
    #[inline(always)]
    pub fn okie(&mut self) -> OKIE_W<0> {
        OKIE_W::new(self)
    }
    #[doc = "Bit 1 - desc WARNIE"]
    #[inline(always)]
    pub fn warnie(&mut self) -> WARNIE_W<1> {
        WARNIE_W::new(self)
    }
    #[doc = "Bit 2 - desc ERRIE"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<2> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 3 - desc UDFIE"]
    #[inline(always)]
    pub fn udfie(&mut self) -> UDFIE_W<3> {
        UDFIE_W::new(self)
    }
    #[doc = "Bit 4 - desc CAPIE"]
    #[inline(always)]
    pub fn capie(&mut self) -> CAPIE_W<4> {
        CAPIE_W::new(self)
    }
    #[doc = "Bit 5 - desc CEN"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W<5> {
        CEN_W::new(self)
    }
    #[doc = "Bit 6 - desc AUTO_TRIM_EN"]
    #[inline(always)]
    pub fn auto_trim_en(&mut self) -> AUTO_TRIM_EN_W<6> {
        AUTO_TRIM_EN_W::new(self)
    }
    #[doc = "Bit 7 - desc SW_SYNC"]
    #[inline(always)]
    pub fn sw_sync(&mut self) -> SW_SYNC_W<7> {
        SW_SYNC_W::new(self)
    }
    #[doc = "Bits 8:14 - desc TRIM"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<8> {
        TRIM_W::new(self)
    }
    #[doc = "Bit 15 - desc TIM_EN"]
    #[inline(always)]
    pub fn tim_en(&mut self) -> TIM_EN_W<15> {
        TIM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
