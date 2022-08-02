#[doc = "Register `PERI_CLKEN1` reader"]
pub struct R(crate::R<PERI_CLKEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERI_CLKEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERI_CLKEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERI_CLKEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERI_CLKEN1` writer"]
pub struct W(crate::W<PERI_CLKEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERI_CLKEN1_SPEC>;
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
impl From<crate::W<PERI_CLKEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERI_CLKEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB` reader - desc USB"]
pub type USB_R = crate::BitReader<bool>;
#[doc = "Field `USB` writer - desc USB"]
pub type USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_CLKEN1_SPEC, bool, O>;
#[doc = "Field `CAN` reader - desc CAN"]
pub type CAN_R = crate::BitReader<bool>;
#[doc = "Field `CAN` writer - desc CAN"]
pub type CAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_CLKEN1_SPEC, bool, O>;
#[doc = "Field `CTS` reader - desc CTS"]
pub type CTS_R = crate::BitReader<bool>;
#[doc = "Field `CTS` writer - desc CTS"]
pub type CTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_CLKEN1_SPEC, bool, O>;
#[doc = "Field `DAC` reader - desc DAC"]
pub type DAC_R = crate::BitReader<bool>;
#[doc = "Field `DAC` writer - desc DAC"]
pub type DAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_CLKEN1_SPEC, bool, O>;
#[doc = "Field `LPTIM1` reader - desc LPTIM1"]
pub type LPTIM1_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1` writer - desc LPTIM1"]
pub type LPTIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_CLKEN1_SPEC, bool, O>;
#[doc = "Field `I2S0` reader - desc I2S0"]
pub type I2S0_R = crate::BitReader<bool>;
#[doc = "Field `I2S0` writer - desc I2S0"]
pub type I2S0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_CLKEN1_SPEC, bool, O>;
#[doc = "Field `I2S1` reader - desc I2S1"]
pub type I2S1_R = crate::BitReader<bool>;
#[doc = "Field `I2S1` writer - desc I2S1"]
pub type I2S1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_CLKEN1_SPEC, bool, O>;
#[doc = "Field `UART2` reader - desc UART2"]
pub type UART2_R = crate::BitReader<bool>;
#[doc = "Field `UART2` writer - desc UART2"]
pub type UART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_CLKEN1_SPEC, bool, O>;
#[doc = "Field `UART3` reader - desc UART3"]
pub type UART3_R = crate::BitReader<bool>;
#[doc = "Field `UART3` writer - desc UART3"]
pub type UART3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_CLKEN1_SPEC, bool, O>;
#[doc = "Field `RSV` reader - desc RSV"]
pub type RSV_R = crate::BitReader<bool>;
#[doc = "Field `RSV` writer - desc RSV"]
pub type RSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERI_CLKEN1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - desc USB"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CAN"]
    #[inline(always)]
    pub fn can(&self) -> CAN_R {
        CAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc DAC"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc LPTIM1"]
    #[inline(always)]
    pub fn lptim1(&self) -> LPTIM1_R {
        LPTIM1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc I2S0"]
    #[inline(always)]
    pub fn i2s0(&self) -> I2S0_R {
        I2S0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc I2S1"]
    #[inline(always)]
    pub fn i2s1(&self) -> I2S1_R {
        I2S1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc UART2"]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc UART3"]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&self) -> RSV_R {
        RSV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc USB"]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W<0> {
        USB_W::new(self)
    }
    #[doc = "Bit 1 - desc CAN"]
    #[inline(always)]
    pub fn can(&mut self) -> CAN_W<1> {
        CAN_W::new(self)
    }
    #[doc = "Bit 2 - desc CTS"]
    #[inline(always)]
    pub fn cts(&mut self) -> CTS_W<2> {
        CTS_W::new(self)
    }
    #[doc = "Bit 3 - desc DAC"]
    #[inline(always)]
    pub fn dac(&mut self) -> DAC_W<3> {
        DAC_W::new(self)
    }
    #[doc = "Bit 4 - desc LPTIM1"]
    #[inline(always)]
    pub fn lptim1(&mut self) -> LPTIM1_W<4> {
        LPTIM1_W::new(self)
    }
    #[doc = "Bit 5 - desc I2S0"]
    #[inline(always)]
    pub fn i2s0(&mut self) -> I2S0_W<5> {
        I2S0_W::new(self)
    }
    #[doc = "Bit 6 - desc I2S1"]
    #[inline(always)]
    pub fn i2s1(&mut self) -> I2S1_W<6> {
        I2S1_W::new(self)
    }
    #[doc = "Bit 8 - desc UART2"]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART2_W<8> {
        UART2_W::new(self)
    }
    #[doc = "Bit 9 - desc UART3"]
    #[inline(always)]
    pub fn uart3(&mut self) -> UART3_W<9> {
        UART3_W::new(self)
    }
    #[doc = "Bit 31 - desc RSV"]
    #[inline(always)]
    pub fn rsv(&mut self) -> RSV_W<31> {
        RSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "desc PERI_CLKEN1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_clken1](index.html) module"]
pub struct PERI_CLKEN1_SPEC;
impl crate::RegisterSpec for PERI_CLKEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peri_clken1::R](R) reader structure"]
impl crate::Readable for PERI_CLKEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peri_clken1::W](W) writer structure"]
impl crate::Writable for PERI_CLKEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERI_CLKEN1 to value 0"]
impl crate::Resettable for PERI_CLKEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
