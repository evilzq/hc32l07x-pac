#[doc = "Register `POEN1` reader"]
pub struct R(crate::R<POEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POEN1` writer"]
pub struct W(crate::W<POEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POEN1_SPEC>;
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
impl From<crate::W<POEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S32` reader - desc S32"]
pub type S32_R = crate::BitReader<bool>;
#[doc = "Field `S32` writer - desc S32"]
pub type S32_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S33` reader - desc S33"]
pub type S33_R = crate::BitReader<bool>;
#[doc = "Field `S33` writer - desc S33"]
pub type S33_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S34` reader - desc S34"]
pub type S34_R = crate::BitReader<bool>;
#[doc = "Field `S34` writer - desc S34"]
pub type S34_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S35` reader - desc S35"]
pub type S35_R = crate::BitReader<bool>;
#[doc = "Field `S35` writer - desc S35"]
pub type S35_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S36C7` reader - desc S36C7"]
pub type S36C7_R = crate::BitReader<bool>;
#[doc = "Field `S36C7` writer - desc S36C7"]
pub type S36C7_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S37C6` reader - desc S37C6"]
pub type S37C6_R = crate::BitReader<bool>;
#[doc = "Field `S37C6` writer - desc S37C6"]
pub type S37C6_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S38C5` reader - desc S38C5"]
pub type S38C5_R = crate::BitReader<bool>;
#[doc = "Field `S38C5` writer - desc S38C5"]
pub type S38C5_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S39C4` reader - desc S39C4"]
pub type S39C4_R = crate::BitReader<bool>;
#[doc = "Field `S39C4` writer - desc S39C4"]
pub type S39C4_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S40` reader - desc S40"]
pub type S40_R = crate::BitReader<bool>;
#[doc = "Field `S40` writer - desc S40"]
pub type S40_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S41` reader - desc S41"]
pub type S41_R = crate::BitReader<bool>;
#[doc = "Field `S41` writer - desc S41"]
pub type S41_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S42` reader - desc S42"]
pub type S42_R = crate::BitReader<bool>;
#[doc = "Field `S42` writer - desc S42"]
pub type S42_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S43` reader - desc S43"]
pub type S43_R = crate::BitReader<bool>;
#[doc = "Field `S43` writer - desc S43"]
pub type S43_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `MUX` reader - desc MUX"]
pub type MUX_R = crate::BitReader<bool>;
#[doc = "Field `MUX` writer - desc MUX"]
pub type MUX_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S44` reader - desc S44"]
pub type S44_R = crate::BitReader<bool>;
#[doc = "Field `S44` writer - desc S44"]
pub type S44_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S45` reader - desc S45"]
pub type S45_R = crate::BitReader<bool>;
#[doc = "Field `S45` writer - desc S45"]
pub type S45_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S46` reader - desc S46"]
pub type S46_R = crate::BitReader<bool>;
#[doc = "Field `S46` writer - desc S46"]
pub type S46_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S47` reader - desc S47"]
pub type S47_R = crate::BitReader<bool>;
#[doc = "Field `S47` writer - desc S47"]
pub type S47_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S48` reader - desc S48"]
pub type S48_R = crate::BitReader<bool>;
#[doc = "Field `S48` writer - desc S48"]
pub type S48_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S49` reader - desc S49"]
pub type S49_R = crate::BitReader<bool>;
#[doc = "Field `S49` writer - desc S49"]
pub type S49_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S50` reader - desc S50"]
pub type S50_R = crate::BitReader<bool>;
#[doc = "Field `S50` writer - desc S50"]
pub type S50_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `S51` reader - desc S51"]
pub type S51_R = crate::BitReader<bool>;
#[doc = "Field `S51` writer - desc S51"]
pub type S51_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `C0` reader - desc C0"]
pub type C0_R = crate::BitReader<bool>;
#[doc = "Field `C0` writer - desc C0"]
pub type C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `C1` reader - desc C1"]
pub type C1_R = crate::BitReader<bool>;
#[doc = "Field `C1` writer - desc C1"]
pub type C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `C2` reader - desc C2"]
pub type C2_R = crate::BitReader<bool>;
#[doc = "Field `C2` writer - desc C2"]
pub type C2_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `C3` reader - desc C3"]
pub type C3_R = crate::BitReader<bool>;
#[doc = "Field `C3` writer - desc C3"]
pub type C3_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
#[doc = "Field `COMMUX` reader - desc COMMUX"]
pub type COMMUX_R = crate::BitReader<bool>;
#[doc = "Field `COMMUX` writer - desc COMMUX"]
pub type COMMUX_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc S32"]
    #[inline(always)]
    pub fn s32(&self) -> S32_R {
        S32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc S33"]
    #[inline(always)]
    pub fn s33(&self) -> S33_R {
        S33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc S34"]
    #[inline(always)]
    pub fn s34(&self) -> S34_R {
        S34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc S35"]
    #[inline(always)]
    pub fn s35(&self) -> S35_R {
        S35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc S36C7"]
    #[inline(always)]
    pub fn s36c7(&self) -> S36C7_R {
        S36C7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc S37C6"]
    #[inline(always)]
    pub fn s37c6(&self) -> S37C6_R {
        S37C6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc S38C5"]
    #[inline(always)]
    pub fn s38c5(&self) -> S38C5_R {
        S38C5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc S39C4"]
    #[inline(always)]
    pub fn s39c4(&self) -> S39C4_R {
        S39C4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc S40"]
    #[inline(always)]
    pub fn s40(&self) -> S40_R {
        S40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc S41"]
    #[inline(always)]
    pub fn s41(&self) -> S41_R {
        S41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc S42"]
    #[inline(always)]
    pub fn s42(&self) -> S42_R {
        S42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc S43"]
    #[inline(always)]
    pub fn s43(&self) -> S43_R {
        S43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - desc MUX"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc S44"]
    #[inline(always)]
    pub fn s44(&self) -> S44_R {
        S44_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc S45"]
    #[inline(always)]
    pub fn s45(&self) -> S45_R {
        S45_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc S46"]
    #[inline(always)]
    pub fn s46(&self) -> S46_R {
        S46_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc S47"]
    #[inline(always)]
    pub fn s47(&self) -> S47_R {
        S47_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc S48"]
    #[inline(always)]
    pub fn s48(&self) -> S48_R {
        S48_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc S49"]
    #[inline(always)]
    pub fn s49(&self) -> S49_R {
        S49_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - desc S50"]
    #[inline(always)]
    pub fn s50(&self) -> S50_R {
        S50_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - desc S51"]
    #[inline(always)]
    pub fn s51(&self) -> S51_R {
        S51_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc C0"]
    #[inline(always)]
    pub fn c0(&self) -> C0_R {
        C0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc C1"]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc C2"]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - desc C3"]
    #[inline(always)]
    pub fn c3(&self) -> C3_R {
        C3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - desc COMMUX"]
    #[inline(always)]
    pub fn commux(&self) -> COMMUX_R {
        COMMUX_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc S32"]
    #[inline(always)]
    pub fn s32(&mut self) -> S32_W<0> {
        S32_W::new(self)
    }
    #[doc = "Bit 1 - desc S33"]
    #[inline(always)]
    pub fn s33(&mut self) -> S33_W<1> {
        S33_W::new(self)
    }
    #[doc = "Bit 2 - desc S34"]
    #[inline(always)]
    pub fn s34(&mut self) -> S34_W<2> {
        S34_W::new(self)
    }
    #[doc = "Bit 3 - desc S35"]
    #[inline(always)]
    pub fn s35(&mut self) -> S35_W<3> {
        S35_W::new(self)
    }
    #[doc = "Bit 4 - desc S36C7"]
    #[inline(always)]
    pub fn s36c7(&mut self) -> S36C7_W<4> {
        S36C7_W::new(self)
    }
    #[doc = "Bit 5 - desc S37C6"]
    #[inline(always)]
    pub fn s37c6(&mut self) -> S37C6_W<5> {
        S37C6_W::new(self)
    }
    #[doc = "Bit 6 - desc S38C5"]
    #[inline(always)]
    pub fn s38c5(&mut self) -> S38C5_W<6> {
        S38C5_W::new(self)
    }
    #[doc = "Bit 7 - desc S39C4"]
    #[inline(always)]
    pub fn s39c4(&mut self) -> S39C4_W<7> {
        S39C4_W::new(self)
    }
    #[doc = "Bit 8 - desc S40"]
    #[inline(always)]
    pub fn s40(&mut self) -> S40_W<8> {
        S40_W::new(self)
    }
    #[doc = "Bit 9 - desc S41"]
    #[inline(always)]
    pub fn s41(&mut self) -> S41_W<9> {
        S41_W::new(self)
    }
    #[doc = "Bit 10 - desc S42"]
    #[inline(always)]
    pub fn s42(&mut self) -> S42_W<10> {
        S42_W::new(self)
    }
    #[doc = "Bit 11 - desc S43"]
    #[inline(always)]
    pub fn s43(&mut self) -> S43_W<11> {
        S43_W::new(self)
    }
    #[doc = "Bit 12 - desc MUX"]
    #[inline(always)]
    pub fn mux(&mut self) -> MUX_W<12> {
        MUX_W::new(self)
    }
    #[doc = "Bit 13 - desc S44"]
    #[inline(always)]
    pub fn s44(&mut self) -> S44_W<13> {
        S44_W::new(self)
    }
    #[doc = "Bit 14 - desc S45"]
    #[inline(always)]
    pub fn s45(&mut self) -> S45_W<14> {
        S45_W::new(self)
    }
    #[doc = "Bit 15 - desc S46"]
    #[inline(always)]
    pub fn s46(&mut self) -> S46_W<15> {
        S46_W::new(self)
    }
    #[doc = "Bit 16 - desc S47"]
    #[inline(always)]
    pub fn s47(&mut self) -> S47_W<16> {
        S47_W::new(self)
    }
    #[doc = "Bit 17 - desc S48"]
    #[inline(always)]
    pub fn s48(&mut self) -> S48_W<17> {
        S48_W::new(self)
    }
    #[doc = "Bit 18 - desc S49"]
    #[inline(always)]
    pub fn s49(&mut self) -> S49_W<18> {
        S49_W::new(self)
    }
    #[doc = "Bit 19 - desc S50"]
    #[inline(always)]
    pub fn s50(&mut self) -> S50_W<19> {
        S50_W::new(self)
    }
    #[doc = "Bit 20 - desc S51"]
    #[inline(always)]
    pub fn s51(&mut self) -> S51_W<20> {
        S51_W::new(self)
    }
    #[doc = "Bit 21 - desc C0"]
    #[inline(always)]
    pub fn c0(&mut self) -> C0_W<21> {
        C0_W::new(self)
    }
    #[doc = "Bit 22 - desc C1"]
    #[inline(always)]
    pub fn c1(&mut self) -> C1_W<22> {
        C1_W::new(self)
    }
    #[doc = "Bit 23 - desc C2"]
    #[inline(always)]
    pub fn c2(&mut self) -> C2_W<23> {
        C2_W::new(self)
    }
    #[doc = "Bit 24 - desc C3"]
    #[inline(always)]
    pub fn c3(&mut self) -> C3_W<24> {
        C3_W::new(self)
    }
    #[doc = "Bit 25 - desc COMMUX"]
    #[inline(always)]
    pub fn commux(&mut self) -> COMMUX_W<25> {
        COMMUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc POEN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poen1](index.html) module"]
pub struct POEN1_SPEC;
impl crate::RegisterSpec for POEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [poen1::R](R) reader structure"]
impl crate::Readable for POEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [poen1::W](W) writer structure"]
impl crate::Writable for POEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POEN1 to value 0x1fff"]
impl crate::Resettable for POEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1fff
    }
}
