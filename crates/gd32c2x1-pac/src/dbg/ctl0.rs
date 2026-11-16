#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `SLP_HOLD` reader - Sleep mode hold bit"]
pub type SlpHoldR = crate::BitReader;
#[doc = "Field `SLP_HOLD` writer - Sleep mode hold bit"]
pub type SlpHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSLP_HOLD` reader - Deep-sleep mode hold bit"]
pub type DslpHoldR = crate::BitReader;
#[doc = "Field `DSLP_HOLD` writer - Deep-sleep mode hold bit"]
pub type DslpHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STB_HOLD` reader - Standby mode hold bit"]
pub type StbHoldR = crate::BitReader;
#[doc = "Field `STB_HOLD` writer - Standby mode hold bit"]
pub type StbHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWDGT_HOLD` reader - FWDGT hold bit"]
pub type FwdgtHoldR = crate::BitReader;
#[doc = "Field `FWDGT_HOLD` writer - FWDGT hold bit"]
pub type FwdgtHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGT_HOLD` reader - WWDGT hold bit"]
pub type WwdgtHoldR = crate::BitReader;
#[doc = "Field `WWDGT_HOLD` writer - WWDGT hold bit"]
pub type WwdgtHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_HOLD` reader - TIMER 0 hold bit"]
pub type Timer0HoldR = crate::BitReader;
#[doc = "Field `TIMER0_HOLD` writer - TIMER 0 hold bit"]
pub type Timer0HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_HOLD` reader - TIMER 2 hold bit"]
pub type Timer2HoldR = crate::BitReader;
#[doc = "Field `TIMER2_HOLD` writer - TIMER 2 hold bit"]
pub type Timer2HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER13_HOLD` reader - TIMER 13 hold bit"]
pub type Timer13HoldR = crate::BitReader;
#[doc = "Field `TIMER13_HOLD` writer - TIMER 13 hold bit"]
pub type Timer13HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0_HOLD` reader - I2C0 hold bit"]
pub type I2c0HoldR = crate::BitReader;
#[doc = "Field `I2C0_HOLD` writer - I2C0 hold bit"]
pub type I2c0HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_HOLD` reader - I2C1 hold bit"]
pub type I2c1HoldR = crate::BitReader;
#[doc = "Field `I2C1_HOLD` writer - I2C1 hold bit"]
pub type I2c1HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER15_HOLD` reader - TIMER 15 hold bit"]
pub type Timer15HoldR = crate::BitReader;
#[doc = "Field `TIMER15_HOLD` writer - TIMER 15 hold bit"]
pub type Timer15HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER16_HOLD` reader - TIMER 16 hold bit"]
pub type Timer16HoldR = crate::BitReader;
#[doc = "Field `TIMER16_HOLD` writer - TIMER 16 hold bit"]
pub type Timer16HoldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sleep mode hold bit"]
    #[inline(always)]
    pub fn slp_hold(&self) -> SlpHoldR {
        SlpHoldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Deep-sleep mode hold bit"]
    #[inline(always)]
    pub fn dslp_hold(&self) -> DslpHoldR {
        DslpHoldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Standby mode hold bit"]
    #[inline(always)]
    pub fn stb_hold(&self) -> StbHoldR {
        StbHoldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - FWDGT hold bit"]
    #[inline(always)]
    pub fn fwdgt_hold(&self) -> FwdgtHoldR {
        FwdgtHoldR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WWDGT hold bit"]
    #[inline(always)]
    pub fn wwdgt_hold(&self) -> WwdgtHoldR {
        WwdgtHoldR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TIMER 0 hold bit"]
    #[inline(always)]
    pub fn timer0_hold(&self) -> Timer0HoldR {
        Timer0HoldR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIMER 2 hold bit"]
    #[inline(always)]
    pub fn timer2_hold(&self) -> Timer2HoldR {
        Timer2HoldR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIMER 13 hold bit"]
    #[inline(always)]
    pub fn timer13_hold(&self) -> Timer13HoldR {
        Timer13HoldR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C0 hold bit"]
    #[inline(always)]
    pub fn i2c0_hold(&self) -> I2c0HoldR {
        I2c0HoldR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C1 hold bit"]
    #[inline(always)]
    pub fn i2c1_hold(&self) -> I2c1HoldR {
        I2c1HoldR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - TIMER 15 hold bit"]
    #[inline(always)]
    pub fn timer15_hold(&self) -> Timer15HoldR {
        Timer15HoldR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TIMER 16 hold bit"]
    #[inline(always)]
    pub fn timer16_hold(&self) -> Timer16HoldR {
        Timer16HoldR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep mode hold bit"]
    #[inline(always)]
    pub fn slp_hold(&mut self) -> SlpHoldW<'_, Ctl0Spec> {
        SlpHoldW::new(self, 0)
    }
    #[doc = "Bit 1 - Deep-sleep mode hold bit"]
    #[inline(always)]
    pub fn dslp_hold(&mut self) -> DslpHoldW<'_, Ctl0Spec> {
        DslpHoldW::new(self, 1)
    }
    #[doc = "Bit 2 - Standby mode hold bit"]
    #[inline(always)]
    pub fn stb_hold(&mut self) -> StbHoldW<'_, Ctl0Spec> {
        StbHoldW::new(self, 2)
    }
    #[doc = "Bit 8 - FWDGT hold bit"]
    #[inline(always)]
    pub fn fwdgt_hold(&mut self) -> FwdgtHoldW<'_, Ctl0Spec> {
        FwdgtHoldW::new(self, 8)
    }
    #[doc = "Bit 9 - WWDGT hold bit"]
    #[inline(always)]
    pub fn wwdgt_hold(&mut self) -> WwdgtHoldW<'_, Ctl0Spec> {
        WwdgtHoldW::new(self, 9)
    }
    #[doc = "Bit 11 - TIMER 0 hold bit"]
    #[inline(always)]
    pub fn timer0_hold(&mut self) -> Timer0HoldW<'_, Ctl0Spec> {
        Timer0HoldW::new(self, 11)
    }
    #[doc = "Bit 12 - TIMER 2 hold bit"]
    #[inline(always)]
    pub fn timer2_hold(&mut self) -> Timer2HoldW<'_, Ctl0Spec> {
        Timer2HoldW::new(self, 12)
    }
    #[doc = "Bit 13 - TIMER 13 hold bit"]
    #[inline(always)]
    pub fn timer13_hold(&mut self) -> Timer13HoldW<'_, Ctl0Spec> {
        Timer13HoldW::new(self, 13)
    }
    #[doc = "Bit 15 - I2C0 hold bit"]
    #[inline(always)]
    pub fn i2c0_hold(&mut self) -> I2c0HoldW<'_, Ctl0Spec> {
        I2c0HoldW::new(self, 15)
    }
    #[doc = "Bit 16 - I2C1 hold bit"]
    #[inline(always)]
    pub fn i2c1_hold(&mut self) -> I2c1HoldW<'_, Ctl0Spec> {
        I2c1HoldW::new(self, 16)
    }
    #[doc = "Bit 20 - TIMER 15 hold bit"]
    #[inline(always)]
    pub fn timer15_hold(&mut self) -> Timer15HoldW<'_, Ctl0Spec> {
        Timer15HoldW::new(self, 20)
    }
    #[doc = "Bit 21 - TIMER 16 hold bit"]
    #[inline(always)]
    pub fn timer16_hold(&mut self) -> Timer16HoldW<'_, Ctl0Spec> {
        Timer16HoldW::new(self, 21)
    }
}
#[doc = "Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {}
