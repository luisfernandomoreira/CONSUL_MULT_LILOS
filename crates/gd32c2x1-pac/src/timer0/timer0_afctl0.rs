#[doc = "Register `TIMER0_AFCTL0` reader"]
pub type R = crate::R<Timer0Afctl0Spec>;
#[doc = "Register `TIMER0_AFCTL0` writer"]
pub type W = crate::W<Timer0Afctl0Spec>;
#[doc = "Field `BRK0IN0EN` reader - BREAK0 BRKIN0 alternate function input enable"]
pub type Brk0in0enR = crate::BitReader;
#[doc = "Field `BRK0IN0EN` writer - BREAK0 BRKIN0 alternate function input enable"]
pub type Brk0in0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK0IN0P` reader - BREAK0 BRKIN0 alternate function input polarity"]
pub type Brk0in0pR = crate::BitReader;
#[doc = "Field `BRK0IN0P` writer - BREAK0 BRKIN0 alternate function input polarity"]
pub type Brk0in0pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETISEL` reader - ETI source selection"]
pub type EtiselR = crate::FieldReader;
#[doc = "Field `ETISEL` writer - ETI source selection"]
pub type EtiselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - BREAK0 BRKIN0 alternate function input enable"]
    #[inline(always)]
    pub fn brk0in0en(&self) -> Brk0in0enR {
        Brk0in0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - BREAK0 BRKIN0 alternate function input polarity"]
    #[inline(always)]
    pub fn brk0in0p(&self) -> Brk0in0pR {
        Brk0in0pR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 14:17 - ETI source selection"]
    #[inline(always)]
    pub fn etisel(&self) -> EtiselR {
        EtiselR::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BREAK0 BRKIN0 alternate function input enable"]
    #[inline(always)]
    pub fn brk0in0en(&mut self) -> Brk0in0enW<'_, Timer0Afctl0Spec> {
        Brk0in0enW::new(self, 0)
    }
    #[doc = "Bit 9 - BREAK0 BRKIN0 alternate function input polarity"]
    #[inline(always)]
    pub fn brk0in0p(&mut self) -> Brk0in0pW<'_, Timer0Afctl0Spec> {
        Brk0in0pW::new(self, 9)
    }
    #[doc = "Bits 14:17 - ETI source selection"]
    #[inline(always)]
    pub fn etisel(&mut self) -> EtiselW<'_, Timer0Afctl0Spec> {
        EtiselW::new(self, 14)
    }
}
#[doc = "Alternate function control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0_afctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0_afctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer0Afctl0Spec;
impl crate::RegisterSpec for Timer0Afctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer0_afctl0::R`](R) reader structure"]
impl crate::Readable for Timer0Afctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`timer0_afctl0::W`](W) writer structure"]
impl crate::Writable for Timer0Afctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER0_AFCTL0 to value 0x01"]
impl crate::Resettable for Timer0Afctl0Spec {
    const RESET_VALUE: u32 = 0x01;
}
