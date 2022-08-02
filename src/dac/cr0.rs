#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
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
impl From<crate::W<CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN0` reader - desc EN0"]
pub type EN0_R = crate::BitReader<bool>;
#[doc = "Field `EN0` writer - desc EN0"]
pub type EN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `BOFF0` reader - desc BOFF0"]
pub type BOFF0_R = crate::BitReader<bool>;
#[doc = "Field `BOFF0` writer - desc BOFF0"]
pub type BOFF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `TEN0` reader - desc TEN0"]
pub type TEN0_R = crate::BitReader<bool>;
#[doc = "Field `TEN0` writer - desc TEN0"]
pub type TEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `TSEL0` reader - desc TSEL0"]
pub type TSEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEL0` writer - desc TSEL0"]
pub type TSEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 3, O>;
#[doc = "Field `WAVE0` reader - desc WAVE0"]
pub type WAVE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAVE0` writer - desc WAVE0"]
pub type WAVE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `MAMP0` reader - desc MAMP0"]
pub type MAMP0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAMP0` writer - desc MAMP0"]
pub type MAMP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `DMAEN0` reader - desc DMAEN0"]
pub type DMAEN0_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN0` writer - desc DMAEN0"]
pub type DMAEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `DMAUDRIE0` reader - desc DMAUDRIE0"]
pub type DMAUDRIE0_R = crate::BitReader<bool>;
#[doc = "Field `DMAUDRIE0` writer - desc DMAUDRIE0"]
pub type DMAUDRIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `SREF0` reader - desc SREF0"]
pub type SREF0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SREF0` writer - desc SREF0"]
pub type SREF0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `EN1` reader - desc EN1"]
pub type EN1_R = crate::BitReader<bool>;
#[doc = "Field `EN1` writer - desc EN1"]
pub type EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `BOFF1` reader - desc BOFF1"]
pub type BOFF1_R = crate::BitReader<bool>;
#[doc = "Field `BOFF1` writer - desc BOFF1"]
pub type BOFF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `TEN1` reader - desc TEN1"]
pub type TEN1_R = crate::BitReader<bool>;
#[doc = "Field `TEN1` writer - desc TEN1"]
pub type TEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `TSEL1` reader - desc TSEL1"]
pub type TSEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEL1` writer - desc TSEL1"]
pub type TSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 3, O>;
#[doc = "Field `WAVE1` reader - desc WAVE1"]
pub type WAVE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAVE1` writer - desc WAVE1"]
pub type WAVE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `MAMP1` reader - desc MAMP1"]
pub type MAMP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAMP1` writer - desc MAMP1"]
pub type MAMP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `DMAEN1` reader - desc DMAEN1"]
pub type DMAEN1_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN1` writer - desc DMAEN1"]
pub type DMAEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `DMAUDRIE1` reader - desc DMAUDRIE1"]
pub type DMAUDRIE1_R = crate::BitReader<bool>;
#[doc = "Field `DMAUDRIE1` writer - desc DMAUDRIE1"]
pub type DMAUDRIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `SREF1` reader - desc SREF1"]
pub type SREF1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SREF1` writer - desc SREF1"]
pub type SREF1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - desc EN0"]
    #[inline(always)]
    pub fn en0(&self) -> EN0_R {
        EN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc BOFF0"]
    #[inline(always)]
    pub fn boff0(&self) -> BOFF0_R {
        BOFF0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc TEN0"]
    #[inline(always)]
    pub fn ten0(&self) -> TEN0_R {
        TEN0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - desc TSEL0"]
    #[inline(always)]
    pub fn tsel0(&self) -> TSEL0_R {
        TSEL0_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - desc WAVE0"]
    #[inline(always)]
    pub fn wave0(&self) -> WAVE0_R {
        WAVE0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - desc MAMP0"]
    #[inline(always)]
    pub fn mamp0(&self) -> MAMP0_R {
        MAMP0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - desc DMAEN0"]
    #[inline(always)]
    pub fn dmaen0(&self) -> DMAEN0_R {
        DMAEN0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc DMAUDRIE0"]
    #[inline(always)]
    pub fn dmaudrie0(&self) -> DMAUDRIE0_R {
        DMAUDRIE0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - desc SREF0"]
    #[inline(always)]
    pub fn sref0(&self) -> SREF0_R {
        SREF0_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - desc EN1"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc BOFF1"]
    #[inline(always)]
    pub fn boff1(&self) -> BOFF1_R {
        BOFF1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc TEN1"]
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - desc TSEL1"]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - desc WAVE1"]
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - desc MAMP1"]
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - desc DMAEN1"]
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc DMAUDRIE1"]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - desc SREF1"]
    #[inline(always)]
    pub fn sref1(&self) -> SREF1_R {
        SREF1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc EN0"]
    #[inline(always)]
    pub fn en0(&mut self) -> EN0_W<0> {
        EN0_W::new(self)
    }
    #[doc = "Bit 1 - desc BOFF0"]
    #[inline(always)]
    pub fn boff0(&mut self) -> BOFF0_W<1> {
        BOFF0_W::new(self)
    }
    #[doc = "Bit 2 - desc TEN0"]
    #[inline(always)]
    pub fn ten0(&mut self) -> TEN0_W<2> {
        TEN0_W::new(self)
    }
    #[doc = "Bits 3:5 - desc TSEL0"]
    #[inline(always)]
    pub fn tsel0(&mut self) -> TSEL0_W<3> {
        TSEL0_W::new(self)
    }
    #[doc = "Bits 6:7 - desc WAVE0"]
    #[inline(always)]
    pub fn wave0(&mut self) -> WAVE0_W<6> {
        WAVE0_W::new(self)
    }
    #[doc = "Bits 8:11 - desc MAMP0"]
    #[inline(always)]
    pub fn mamp0(&mut self) -> MAMP0_W<8> {
        MAMP0_W::new(self)
    }
    #[doc = "Bit 12 - desc DMAEN0"]
    #[inline(always)]
    pub fn dmaen0(&mut self) -> DMAEN0_W<12> {
        DMAEN0_W::new(self)
    }
    #[doc = "Bit 13 - desc DMAUDRIE0"]
    #[inline(always)]
    pub fn dmaudrie0(&mut self) -> DMAUDRIE0_W<13> {
        DMAUDRIE0_W::new(self)
    }
    #[doc = "Bits 14:15 - desc SREF0"]
    #[inline(always)]
    pub fn sref0(&mut self) -> SREF0_W<14> {
        SREF0_W::new(self)
    }
    #[doc = "Bit 16 - desc EN1"]
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W<16> {
        EN1_W::new(self)
    }
    #[doc = "Bit 17 - desc BOFF1"]
    #[inline(always)]
    pub fn boff1(&mut self) -> BOFF1_W<17> {
        BOFF1_W::new(self)
    }
    #[doc = "Bit 18 - desc TEN1"]
    #[inline(always)]
    pub fn ten1(&mut self) -> TEN1_W<18> {
        TEN1_W::new(self)
    }
    #[doc = "Bits 19:21 - desc TSEL1"]
    #[inline(always)]
    pub fn tsel1(&mut self) -> TSEL1_W<19> {
        TSEL1_W::new(self)
    }
    #[doc = "Bits 22:23 - desc WAVE1"]
    #[inline(always)]
    pub fn wave1(&mut self) -> WAVE1_W<22> {
        WAVE1_W::new(self)
    }
    #[doc = "Bits 24:27 - desc MAMP1"]
    #[inline(always)]
    pub fn mamp1(&mut self) -> MAMP1_W<24> {
        MAMP1_W::new(self)
    }
    #[doc = "Bit 28 - desc DMAEN1"]
    #[inline(always)]
    pub fn dmaen1(&mut self) -> DMAEN1_W<28> {
        DMAEN1_W::new(self)
    }
    #[doc = "Bit 29 - desc DMAUDRIE1"]
    #[inline(always)]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W<29> {
        DMAUDRIE1_W::new(self)
    }
    #[doc = "Bits 30:31 - desc SREF1"]
    #[inline(always)]
    pub fn sref1(&mut self) -> SREF1_W<30> {
        SREF1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR0 to value 0xc000_c000"]
impl crate::Resettable for CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_c000
    }
}
