#[doc = "Register `BT` reader"]
pub struct R(crate::R<BT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BT` writer"]
pub struct W(crate::W<BT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BT_SPEC>;
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
impl From<crate::W<BT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEG_1` reader - desc SEG_1"]
pub type SEG_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEG_1` writer - desc SEG_1"]
pub type SEG_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BT_SPEC, u8, u8, 8, O>;
#[doc = "Field `SEG_2` reader - desc SEG_2"]
pub type SEG_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEG_2` writer - desc SEG_2"]
pub type SEG_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BT_SPEC, u8, u8, 7, O>;
#[doc = "Field `SJW` reader - desc SJW"]
pub type SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SJW` writer - desc SJW"]
pub type SJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BT_SPEC, u8, u8, 7, O>;
#[doc = "Field `PRESC` reader - desc PRESC"]
pub type PRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESC` writer - desc PRESC"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - desc SEG_1"]
    #[inline(always)]
    pub fn seg_1(&self) -> SEG_1_R {
        SEG_1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - desc SEG_2"]
    #[inline(always)]
    pub fn seg_2(&self) -> SEG_2_R {
        SEG_2_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - desc SJW"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - desc PRESC"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc SEG_1"]
    #[inline(always)]
    pub fn seg_1(&mut self) -> SEG_1_W<0> {
        SEG_1_W::new(self)
    }
    #[doc = "Bits 8:14 - desc SEG_2"]
    #[inline(always)]
    pub fn seg_2(&mut self) -> SEG_2_W<8> {
        SEG_2_W::new(self)
    }
    #[doc = "Bits 16:22 - desc SJW"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W<16> {
        SJW_W::new(self)
    }
    #[doc = "Bits 24:31 - desc PRESC"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<24> {
        PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc BT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt](index.html) module"]
pub struct BT_SPEC;
impl crate::RegisterSpec for BT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bt::R](R) reader structure"]
impl crate::Readable for BT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bt::W](W) writer structure"]
impl crate::Writable for BT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BT to value 0x0102_0203"]
impl crate::Resettable for BT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0102_0203
    }
}
