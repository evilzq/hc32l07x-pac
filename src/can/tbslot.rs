#[doc = "Register `TBSLOT` reader"]
pub struct R(crate::R<TBSLOT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBSLOT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBSLOT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBSLOT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBSLOT` writer"]
pub struct W(crate::W<TBSLOT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBSLOT_SPEC>;
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
impl From<crate::W<TBSLOT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBSLOT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBPTR` reader - desc TBPTR"]
pub type TBPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TBPTR` writer - desc TBPTR"]
pub type TBPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TBSLOT_SPEC, u8, u8, 6, O>;
#[doc = "Field `TBF` reader - desc TBF"]
pub type TBF_R = crate::BitReader<bool>;
#[doc = "Field `TBF` writer - desc TBF"]
pub type TBF_W<'a, const O: u8> = crate::BitWriter<'a, u8, TBSLOT_SPEC, bool, O>;
#[doc = "Field `TBE` reader - desc TBE"]
pub type TBE_R = crate::BitReader<bool>;
#[doc = "Field `TBE` writer - desc TBE"]
pub type TBE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TBSLOT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - desc TBPTR"]
    #[inline(always)]
    pub fn tbptr(&self) -> TBPTR_R {
        TBPTR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - desc TBF"]
    #[inline(always)]
    pub fn tbf(&self) -> TBF_R {
        TBF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TBE"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc TBPTR"]
    #[inline(always)]
    pub fn tbptr(&mut self) -> TBPTR_W<0> {
        TBPTR_W::new(self)
    }
    #[doc = "Bit 6 - desc TBF"]
    #[inline(always)]
    pub fn tbf(&mut self) -> TBF_W<6> {
        TBF_W::new(self)
    }
    #[doc = "Bit 7 - desc TBE"]
    #[inline(always)]
    pub fn tbe(&mut self) -> TBE_W<7> {
        TBE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc TBSLOT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbslot](index.html) module"]
pub struct TBSLOT_SPEC;
impl crate::RegisterSpec for TBSLOT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tbslot::R](R) reader structure"]
impl crate::Readable for TBSLOT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbslot::W](W) writer structure"]
impl crate::Writable for TBSLOT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBSLOT to value 0"]
impl crate::Resettable for TBSLOT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
