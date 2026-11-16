#[doc = "Register `WD1HT` reader"]
pub type R = crate::R<Wd1htSpec>;
#[doc = "Register `WD1HT` writer"]
pub type W = crate::W<Wd1htSpec>;
#[doc = "Field `WD1HT` reader - High threshold for analog watchdog 1"]
pub type Wd1htR = crate::FieldReader<u16>;
#[doc = "Field `WD1HT` writer - High threshold for analog watchdog 1"]
pub type Wd1htW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - High threshold for analog watchdog 1"]
    #[inline(always)]
    pub fn wd1ht(&self) -> Wd1htR {
        Wd1htR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - High threshold for analog watchdog 1"]
    #[inline(always)]
    pub fn wd1ht(&mut self) -> Wd1htW<'_, Wd1htSpec> {
        Wd1htW::new(self, 0)
    }
}
#[doc = "Watchdog 1 high threshold register1\n\nYou can [`read`](crate::Reg::read) this register and get [`wd1ht::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd1ht::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wd1htSpec;
impl crate::RegisterSpec for Wd1htSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wd1ht::R`](R) reader structure"]
impl crate::Readable for Wd1htSpec {}
#[doc = "`write(|w| ..)` method takes [`wd1ht::W`](W) writer structure"]
impl crate::Writable for Wd1htSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WD1HT to value 0x0fff"]
impl crate::Resettable for Wd1htSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
