#[doc = "Register `CONFA0` reader"]
pub struct R(crate::R<CONFA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFA0` writer"]
pub struct W(crate::W<CONFA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFA0_SPEC>;
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
impl From<crate::W<CONFA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TC` reader - desc TC"]
pub type TC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TC` writer - desc TC"]
pub type TC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFA0_SPEC, u16, u16, 16, O>;
#[doc = "Field `BC` reader - desc BC"]
pub type BC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BC` writer - desc BC"]
pub type BC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFA0_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRI_SEL` reader - desc TRI_SEL"]
pub type TRI_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRI_SEL` writer - desc TRI_SEL"]
pub type TRI_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFA0_SPEC, u8, u8, 7, O>;
#[doc = "Field `ST` reader - desc ST"]
pub type ST_R = crate::BitReader<bool>;
#[doc = "Field `ST` writer - desc ST"]
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFA0_SPEC, bool, O>;
#[doc = "Field `PAS` reader - desc PAS"]
pub type PAS_R = crate::BitReader<bool>;
#[doc = "Field `PAS` writer - desc PAS"]
pub type PAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFA0_SPEC, bool, O>;
#[doc = "Field `ENS` reader - desc ENS"]
pub type ENS_R = crate::BitReader<bool>;
#[doc = "Field `ENS` writer - desc ENS"]
pub type ENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFA0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - desc TC"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - desc BC"]
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 22:28 - desc TRI_SEL"]
    #[inline(always)]
    pub fn tri_sel(&self) -> TRI_SEL_R {
        TRI_SEL_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - desc ST"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc PAS"]
    #[inline(always)]
    pub fn pas(&self) -> PAS_R {
        PAS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc ENS"]
    #[inline(always)]
    pub fn ens(&self) -> ENS_R {
        ENS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc TC"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<0> {
        TC_W::new(self)
    }
    #[doc = "Bits 16:19 - desc BC"]
    #[inline(always)]
    pub fn bc(&mut self) -> BC_W<16> {
        BC_W::new(self)
    }
    #[doc = "Bits 22:28 - desc TRI_SEL"]
    #[inline(always)]
    pub fn tri_sel(&mut self) -> TRI_SEL_W<22> {
        TRI_SEL_W::new(self)
    }
    #[doc = "Bit 29 - desc ST"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<29> {
        ST_W::new(self)
    }
    #[doc = "Bit 30 - desc PAS"]
    #[inline(always)]
    pub fn pas(&mut self) -> PAS_W<30> {
        PAS_W::new(self)
    }
    #[doc = "Bit 31 - desc ENS"]
    #[inline(always)]
    pub fn ens(&mut self) -> ENS_W<31> {
        ENS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CONFA0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confa0](index.html) module"]
pub struct CONFA0_SPEC;
impl crate::RegisterSpec for CONFA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [confa0::R](R) reader structure"]
impl crate::Readable for CONFA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [confa0::W](W) writer structure"]
impl crate::Writable for CONFA0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFA0 to value 0"]
impl crate::Resettable for CONFA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
