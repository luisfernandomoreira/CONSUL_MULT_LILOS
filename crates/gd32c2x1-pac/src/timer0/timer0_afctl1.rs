#[doc = "Register `TIMER0_AFCTL1` reader"]
pub type R = crate::R<Timer0Afctl1Spec>;
#[doc = "Register `TIMER0_AFCTL1` writer"]
pub type W = crate::W<Timer0Afctl1Spec>;
#[doc = "Field `BRK0IN1EN` reader - BREAK0 BRKIN1 alternate function input enable"]
pub type Brk0in1enR = crate::BitReader;
#[doc = "Field `BRK0IN1EN` writer - BREAK0 BRKIN1 alternate function input enable"]
pub type Brk0in1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK0IN1P` reader - BREAK0 BRKIN1 alternate function input polarity"]
pub type Brk0in1pR = crate::BitReader;
#[doc = "Field `BRK0IN1P` writer - BREAK0 BRKIN1 alternate function input polarity"]
pub type Brk0in1pW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BREAK0 BRKIN1 alternate function input enable"]
    #[inline(always)]
    pub fn brk0in1en(&self) -> Brk0in1enR {
        Brk0in1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - BREAK0 BRKIN1 alternate function input polarity"]
    #[inline(always)]
    pub fn brk0in1p(&self) -> Brk0in1pR {
        Brk0in1pR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BREAK0 BRKIN1 alternate function input enable"]
    #[inline(always)]
    pub fn brk0in1en(&mut self) -> Brk0in1enW<'_, Timer0Afctl1Spec> {
        Brk0in1enW::new(self, 0)
    }
    #[doc = "Bit 9 - BREAK0 BRKIN1 alternate function input polarity"]
    #[inline(always)]
    pub fn brk0in1p(&mut self) -> Brk0in1pW<'_, Timer0Afctl1Spec> {
        Brk0in1pW::new(self, 9)
    }
}
#[doc = "Alternate function control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_afctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_afctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer0Afctl1Spec;
impl crate::RegisterSpec for Timer0Afctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer0_afctl1::R`](R) reader structure"]
impl crate::Readable for Timer0Afctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`timer0_afctl1::W`](W) writer structure"]
impl crate::Writable for Timer0Afctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER0_AFCTL1 to value 0x01"]
impl crate::Resettable for Timer0Afctl1Spec {
    const RESET_VALUE: u32 = 0x01;
}
