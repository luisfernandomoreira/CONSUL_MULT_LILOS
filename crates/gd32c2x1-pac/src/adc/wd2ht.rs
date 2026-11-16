#[doc = "Register `WD2HT` reader"]
pub type R = crate::R<Wd2htSpec>;
#[doc = "Register `WD2HT` writer"]
pub type W = crate::W<Wd2htSpec>;
#[doc = "Field `WD2HT` reader - High threshold for analog watchdog 2"]
pub type Wd2htR = crate::FieldReader<u16>;
#[doc = "Field `WD2HT` writer - High threshold for analog watchdog 2"]
pub type Wd2htW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - High threshold for analog watchdog 2"]
    #[inline(always)]
    pub fn wd2ht(&self) -> Wd2htR {
        Wd2htR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - High threshold for analog watchdog 2"]
    #[inline(always)]
    pub fn wd2ht(&mut self) -> Wd2htW<'_, Wd2htSpec> {
        Wd2htW::new(self, 0)
    }
}
#[doc = "Watchdog 2 high threshold register2\n\nYou can [`read`](crate::Reg::read) this register and get [`wd2ht::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd2ht::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wd2htSpec;
impl crate::RegisterSpec for Wd2htSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wd2ht::R`](R) reader structure"]
impl crate::Readable for Wd2htSpec {}
#[doc = "`write(|w| ..)` method takes [`wd2ht::W`](W) writer structure"]
impl crate::Writable for Wd2htSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WD2HT to value 0x0fff"]
impl crate::Resettable for Wd2htSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
