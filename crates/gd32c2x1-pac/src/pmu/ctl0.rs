#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `LPMOD` reader - Select the low-power mode to enter when the Cortex-M23 enters SLEEPDEEP mode."]
pub type LpmodR = crate::FieldReader;
#[doc = "Field `LPMOD` writer - Select the low-power mode to enter when the Cortex-M23 enters SLEEPDEEP mode."]
pub type LpmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WURST` reader - Wakeup Flag Reset"]
pub type WurstR = crate::BitReader;
#[doc = "Field `WURST` writer - Wakeup Flag Reset"]
pub type WurstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBRST` reader - Standby Flag Reset"]
pub type StbrstR = crate::BitReader;
#[doc = "Field `STBRST` writer - Standby Flag Reset"]
pub type StbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPWEN` reader - RTC Backup register write enable"]
pub type BkpwenR = crate::BitReader;
#[doc = "Field `BKPWEN` writer - RTC Backup register write enable"]
pub type BkpwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSMODVS` reader - Deep-sleep mode voltage selection."]
pub type DsmodvsR = crate::FieldReader;
#[doc = "Field `DSMODVS` writer - Deep-sleep mode voltage selection."]
pub type DsmodvsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPLDOEN` reader - Low power LDO enable"]
pub type LpldoenR = crate::BitReader;
#[doc = "Field `LPLDOEN` writer - Low power LDO enable"]
pub type LpldoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Select the low-power mode to enter when the Cortex-M23 enters SLEEPDEEP mode."]
    #[inline(always)]
    pub fn lpmod(&self) -> LpmodR {
        LpmodR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Wakeup Flag Reset"]
    #[inline(always)]
    pub fn wurst(&self) -> WurstR {
        WurstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Standby Flag Reset"]
    #[inline(always)]
    pub fn stbrst(&self) -> StbrstR {
        StbrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC Backup register write enable"]
    #[inline(always)]
    pub fn bkpwen(&self) -> BkpwenR {
        BkpwenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Deep-sleep mode voltage selection."]
    #[inline(always)]
    pub fn dsmodvs(&self) -> DsmodvsR {
        DsmodvsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 18 - Low power LDO enable"]
    #[inline(always)]
    pub fn lpldoen(&self) -> LpldoenR {
        LpldoenR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select the low-power mode to enter when the Cortex-M23 enters SLEEPDEEP mode."]
    #[inline(always)]
    pub fn lpmod(&mut self) -> LpmodW<'_, Ctl0Spec> {
        LpmodW::new(self, 0)
    }
    #[doc = "Bit 2 - Wakeup Flag Reset"]
    #[inline(always)]
    pub fn wurst(&mut self) -> WurstW<'_, Ctl0Spec> {
        WurstW::new(self, 2)
    }
    #[doc = "Bit 3 - Standby Flag Reset"]
    #[inline(always)]
    pub fn stbrst(&mut self) -> StbrstW<'_, Ctl0Spec> {
        StbrstW::new(self, 3)
    }
    #[doc = "Bit 8 - RTC Backup register write enable"]
    #[inline(always)]
    pub fn bkpwen(&mut self) -> BkpwenW<'_, Ctl0Spec> {
        BkpwenW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Deep-sleep mode voltage selection."]
    #[inline(always)]
    pub fn dsmodvs(&mut self) -> DsmodvsW<'_, Ctl0Spec> {
        DsmodvsW::new(self, 12)
    }
    #[doc = "Bit 18 - Low power LDO enable"]
    #[inline(always)]
    pub fn lpldoen(&mut self) -> LpldoenW<'_, Ctl0Spec> {
        LpldoenW::new(self, 18)
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
#[doc = "`reset()` method sets CTL0 to value 0x0002_9000"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0x0002_9000;
}
