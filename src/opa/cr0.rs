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
#[doc = "Field `OP0OEN1` reader - desc OP0OEN1"]
pub type OP0OEN1_R = crate::BitReader<bool>;
#[doc = "Field `OP0OEN1` writer - desc OP0OEN1"]
pub type OP0OEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `OP0OEN2` reader - desc OP0OEN2"]
pub type OP0OEN2_R = crate::BitReader<bool>;
#[doc = "Field `OP0OEN2` writer - desc OP0OEN2"]
pub type OP0OEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `OP0OEN3` reader - desc OP0OEN3"]
pub type OP0OEN3_R = crate::BitReader<bool>;
#[doc = "Field `OP0OEN3` writer - desc OP0OEN3"]
pub type OP0OEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `OP0OEN4` reader - desc OP0OEN4"]
pub type OP0OEN4_R = crate::BitReader<bool>;
#[doc = "Field `OP0OEN4` writer - desc OP0OEN4"]
pub type OP0OEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `OP1OEN1` reader - desc OP1OEN1"]
pub type OP1OEN1_R = crate::BitReader<bool>;
#[doc = "Field `OP1OEN1` writer - desc OP1OEN1"]
pub type OP1OEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `OP1OEN2` reader - desc OP1OEN2"]
pub type OP1OEN2_R = crate::BitReader<bool>;
#[doc = "Field `OP1OEN2` writer - desc OP1OEN2"]
pub type OP1OEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `OP1OEN3` reader - desc OP1OEN3"]
pub type OP1OEN3_R = crate::BitReader<bool>;
#[doc = "Field `OP1OEN3` writer - desc OP1OEN3"]
pub type OP1OEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `OP1OEN4` reader - desc OP1OEN4"]
pub type OP1OEN4_R = crate::BitReader<bool>;
#[doc = "Field `OP1OEN4` writer - desc OP1OEN4"]
pub type OP1OEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `OP2OEN1` reader - desc OP2OEN1"]
pub type OP2OEN1_R = crate::BitReader<bool>;
#[doc = "Field `OP2OEN1` writer - desc OP2OEN1"]
pub type OP2OEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `OP2OEN2` reader - desc OP2OEN2"]
pub type OP2OEN2_R = crate::BitReader<bool>;
#[doc = "Field `OP2OEN2` writer - desc OP2OEN2"]
pub type OP2OEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `OP2OEN3` reader - desc OP2OEN3"]
pub type OP2OEN3_R = crate::BitReader<bool>;
#[doc = "Field `OP2OEN3` writer - desc OP2OEN3"]
pub type OP2OEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `OP2OEN4` reader - desc OP2OEN4"]
pub type OP2OEN4_R = crate::BitReader<bool>;
#[doc = "Field `OP2OEN4` writer - desc OP2OEN4"]
pub type OP2OEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc OP0OEN1"]
    #[inline(always)]
    pub fn op0oen1(&self) -> OP0OEN1_R {
        OP0OEN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc OP0OEN2"]
    #[inline(always)]
    pub fn op0oen2(&self) -> OP0OEN2_R {
        OP0OEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc OP0OEN3"]
    #[inline(always)]
    pub fn op0oen3(&self) -> OP0OEN3_R {
        OP0OEN3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc OP0OEN4"]
    #[inline(always)]
    pub fn op0oen4(&self) -> OP0OEN4_R {
        OP0OEN4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc OP1OEN1"]
    #[inline(always)]
    pub fn op1oen1(&self) -> OP1OEN1_R {
        OP1OEN1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc OP1OEN2"]
    #[inline(always)]
    pub fn op1oen2(&self) -> OP1OEN2_R {
        OP1OEN2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc OP1OEN3"]
    #[inline(always)]
    pub fn op1oen3(&self) -> OP1OEN3_R {
        OP1OEN3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc OP1OEN4"]
    #[inline(always)]
    pub fn op1oen4(&self) -> OP1OEN4_R {
        OP1OEN4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc OP2OEN1"]
    #[inline(always)]
    pub fn op2oen1(&self) -> OP2OEN1_R {
        OP2OEN1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc OP2OEN2"]
    #[inline(always)]
    pub fn op2oen2(&self) -> OP2OEN2_R {
        OP2OEN2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc OP2OEN3"]
    #[inline(always)]
    pub fn op2oen3(&self) -> OP2OEN3_R {
        OP2OEN3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc OP2OEN4"]
    #[inline(always)]
    pub fn op2oen4(&self) -> OP2OEN4_R {
        OP2OEN4_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc OP0OEN1"]
    #[inline(always)]
    pub fn op0oen1(&mut self) -> OP0OEN1_W<0> {
        OP0OEN1_W::new(self)
    }
    #[doc = "Bit 1 - desc OP0OEN2"]
    #[inline(always)]
    pub fn op0oen2(&mut self) -> OP0OEN2_W<1> {
        OP0OEN2_W::new(self)
    }
    #[doc = "Bit 2 - desc OP0OEN3"]
    #[inline(always)]
    pub fn op0oen3(&mut self) -> OP0OEN3_W<2> {
        OP0OEN3_W::new(self)
    }
    #[doc = "Bit 3 - desc OP0OEN4"]
    #[inline(always)]
    pub fn op0oen4(&mut self) -> OP0OEN4_W<3> {
        OP0OEN4_W::new(self)
    }
    #[doc = "Bit 4 - desc OP1OEN1"]
    #[inline(always)]
    pub fn op1oen1(&mut self) -> OP1OEN1_W<4> {
        OP1OEN1_W::new(self)
    }
    #[doc = "Bit 5 - desc OP1OEN2"]
    #[inline(always)]
    pub fn op1oen2(&mut self) -> OP1OEN2_W<5> {
        OP1OEN2_W::new(self)
    }
    #[doc = "Bit 6 - desc OP1OEN3"]
    #[inline(always)]
    pub fn op1oen3(&mut self) -> OP1OEN3_W<6> {
        OP1OEN3_W::new(self)
    }
    #[doc = "Bit 7 - desc OP1OEN4"]
    #[inline(always)]
    pub fn op1oen4(&mut self) -> OP1OEN4_W<7> {
        OP1OEN4_W::new(self)
    }
    #[doc = "Bit 8 - desc OP2OEN1"]
    #[inline(always)]
    pub fn op2oen1(&mut self) -> OP2OEN1_W<8> {
        OP2OEN1_W::new(self)
    }
    #[doc = "Bit 9 - desc OP2OEN2"]
    #[inline(always)]
    pub fn op2oen2(&mut self) -> OP2OEN2_W<9> {
        OP2OEN2_W::new(self)
    }
    #[doc = "Bit 10 - desc OP2OEN3"]
    #[inline(always)]
    pub fn op2oen3(&mut self) -> OP2OEN3_W<10> {
        OP2OEN3_W::new(self)
    }
    #[doc = "Bit 11 - desc OP2OEN4"]
    #[inline(always)]
    pub fn op2oen4(&mut self) -> OP2OEN4_W<11> {
        OP2OEN4_W::new(self)
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
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
