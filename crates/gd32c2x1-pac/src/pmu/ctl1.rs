#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `EFPSLEEP` reader - EFLASH power off control when MCU enters to Run/Run1 mode."]
pub type EfpsleepR = crate::BitReader;
#[doc = "Field `EFPSLEEP` writer - EFLASH power off control when MCU enters to Run/Run1 mode."]
pub type EfpsleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFDSPSLEEP` reader - EFLASH power off contrl when MCU enters to Deep-sleep/Deep-sleep1 mode."]
pub type EfdspsleepR = crate::BitReader;
#[doc = "Field `EFDSPSLEEP` writer - EFLASH power off contrl when MCU enters to Deep-sleep/Deep-sleep1 mode."]
pub type EfdspsleepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - EFLASH power off control when MCU enters to Run/Run1 mode."]
    #[inline(always)]
    pub fn efpsleep(&self) -> EfpsleepR {
        EfpsleepR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EFLASH power off contrl when MCU enters to Deep-sleep/Deep-sleep1 mode."]
    #[inline(always)]
    pub fn efdspsleep(&self) -> EfdspsleepR {
        EfdspsleepR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - EFLASH power off control when MCU enters to Run/Run1 mode."]
    #[inline(always)]
    pub fn efpsleep(&mut self) -> EfpsleepW<'_, Ctl1Spec> {
        EfpsleepW::new(self, 4)
    }
    #[doc = "Bit 5 - EFLASH power off contrl when MCU enters to Deep-sleep/Deep-sleep1 mode."]
    #[inline(always)]
    pub fn efdspsleep(&mut self) -> EfdspsleepW<'_, Ctl1Spec> {
        EfdspsleepW::new(self, 5)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {}
