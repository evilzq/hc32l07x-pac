#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AZEN0` reader - desc AZEN0"]
pub type AZEN0_R = crate::BitReader<bool>;
#[doc = "Field `AZEN0` writer - desc AZEN0"]
pub type AZEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `AZEN1` reader - desc AZEN1"]
pub type AZEN1_R = crate::BitReader<bool>;
#[doc = "Field `AZEN1` writer - desc AZEN1"]
pub type AZEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `AZEN2` reader - desc AZEN2"]
pub type AZEN2_R = crate::BitReader<bool>;
#[doc = "Field `AZEN2` writer - desc AZEN2"]
pub type AZEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `AZEN3` reader - desc AZEN3"]
pub type AZEN3_R = crate::BitReader<bool>;
#[doc = "Field `AZEN3` writer - desc AZEN3"]
pub type AZEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `AZEN4` reader - desc AZEN4"]
pub type AZEN4_R = crate::BitReader<bool>;
#[doc = "Field `AZEN4` writer - desc AZEN4"]
pub type AZEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `EN0` reader - desc EN0"]
pub type EN0_R = crate::BitReader<bool>;
#[doc = "Field `EN0` writer - desc EN0"]
pub type EN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `EN1` reader - desc EN1"]
pub type EN1_R = crate::BitReader<bool>;
#[doc = "Field `EN1` writer - desc EN1"]
pub type EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `EN2` reader - desc EN2"]
pub type EN2_R = crate::BitReader<bool>;
#[doc = "Field `EN2` writer - desc EN2"]
pub type EN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `EN3` reader - desc EN3"]
pub type EN3_R = crate::BitReader<bool>;
#[doc = "Field `EN3` writer - desc EN3"]
pub type EN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `EN4` reader - desc EN4"]
pub type EN4_R = crate::BitReader<bool>;
#[doc = "Field `EN4` writer - desc EN4"]
pub type EN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `OP3BUFEN` reader - desc OP3BUFEN"]
pub type OP3BUFEN_R = crate::BitReader<bool>;
#[doc = "Field `OP3BUFEN` writer - desc OP3BUFEN"]
pub type OP3BUFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `OP4BUFEN` reader - desc OP4BUFEN"]
pub type OP4BUFEN_R = crate::BitReader<bool>;
#[doc = "Field `OP4BUFEN` writer - desc OP4BUFEN"]
pub type OP4BUFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc AZEN0"]
    #[inline(always)]
    pub fn azen0(&self) -> AZEN0_R {
        AZEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc AZEN1"]
    #[inline(always)]
    pub fn azen1(&self) -> AZEN1_R {
        AZEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc AZEN2"]
    #[inline(always)]
    pub fn azen2(&self) -> AZEN2_R {
        AZEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc AZEN3"]
    #[inline(always)]
    pub fn azen3(&self) -> AZEN3_R {
        AZEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc AZEN4"]
    #[inline(always)]
    pub fn azen4(&self) -> AZEN4_R {
        AZEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc EN0"]
    #[inline(always)]
    pub fn en0(&self) -> EN0_R {
        EN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc EN1"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc EN2"]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc EN3"]
    #[inline(always)]
    pub fn en3(&self) -> EN3_R {
        EN3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc EN4"]
    #[inline(always)]
    pub fn en4(&self) -> EN4_R {
        EN4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc OP3BUFEN"]
    #[inline(always)]
    pub fn op3bufen(&self) -> OP3BUFEN_R {
        OP3BUFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc OP4BUFEN"]
    #[inline(always)]
    pub fn op4bufen(&self) -> OP4BUFEN_R {
        OP4BUFEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc AZEN0"]
    #[inline(always)]
    pub fn azen0(&mut self) -> AZEN0_W<0> {
        AZEN0_W::new(self)
    }
    #[doc = "Bit 1 - desc AZEN1"]
    #[inline(always)]
    pub fn azen1(&mut self) -> AZEN1_W<1> {
        AZEN1_W::new(self)
    }
    #[doc = "Bit 2 - desc AZEN2"]
    #[inline(always)]
    pub fn azen2(&mut self) -> AZEN2_W<2> {
        AZEN2_W::new(self)
    }
    #[doc = "Bit 3 - desc AZEN3"]
    #[inline(always)]
    pub fn azen3(&mut self) -> AZEN3_W<3> {
        AZEN3_W::new(self)
    }
    #[doc = "Bit 4 - desc AZEN4"]
    #[inline(always)]
    pub fn azen4(&mut self) -> AZEN4_W<4> {
        AZEN4_W::new(self)
    }
    #[doc = "Bit 5 - desc EN0"]
    #[inline(always)]
    pub fn en0(&mut self) -> EN0_W<5> {
        EN0_W::new(self)
    }
    #[doc = "Bit 6 - desc EN1"]
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W<6> {
        EN1_W::new(self)
    }
    #[doc = "Bit 7 - desc EN2"]
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W<7> {
        EN2_W::new(self)
    }
    #[doc = "Bit 8 - desc EN3"]
    #[inline(always)]
    pub fn en3(&mut self) -> EN3_W<8> {
        EN3_W::new(self)
    }
    #[doc = "Bit 9 - desc EN4"]
    #[inline(always)]
    pub fn en4(&mut self) -> EN4_W<9> {
        EN4_W::new(self)
    }
    #[doc = "Bit 10 - desc OP3BUFEN"]
    #[inline(always)]
    pub fn op3bufen(&mut self) -> OP3BUFEN_W<10> {
        OP3BUFEN_W::new(self)
    }
    #[doc = "Bit 11 - desc OP4BUFEN"]
    #[inline(always)]
    pub fn op4bufen(&mut self) -> OP4BUFEN_W<11> {
        OP4BUFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc CR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
