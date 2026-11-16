#[doc = "Register `WD1LT` reader"]
pub type R = crate::R<Wd1ltSpec>;
#[doc = "Register `WD1LT` writer"]
pub type W = crate::W<Wd1ltSpec>;
#[doc = "Field `WD1LT` reader - Low threshold for analog watchdog 1"]
pub type Wd1ltR = crate::FieldReader<u16>;
#[doc = "Field `WD1LT` writer - Low threshold for analog watchdog 1"]
pub type Wd1ltW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Low threshold for analog watchdog 1"]
    #[inline(always)]
    pub fn wd1lt(&self) -> Wd1ltR {
        Wd1ltR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Low threshold for analog watchdog 1"]
    #[inline(always)]
    pub fn wd1lt(&mut self) -> Wd1ltW<'_, Wd1ltSpec> {
        Wd1ltW::new(self, 0)
    }
}
#[doc = "Watchdog 1 low threshold register1\n\nYou can [`read`](crate::Reg::read) this register and get [`wd1lt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd1lt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wd1ltSpec;
impl crate::RegisterSpec for Wd1ltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wd1lt::R`](R) reader structure"]
impl crate::Readable for Wd1ltSpec {}
#[doc = "`write(|w| ..)` method takes [`wd1lt::W`](W) writer structure"]
impl crate::Writable for Wd1ltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WD1LT to value 0"]
impl crate::Resettable for Wd1ltSpec {}
