#[doc = "Register `WD0HT` reader"]
pub type R = crate::R<Wd0htSpec>;
#[doc = "Register `WD0HT` writer"]
pub type W = crate::W<Wd0htSpec>;
#[doc = "Field `WD0HT` reader - High threshold for analog watchdog 0"]
pub type Wd0htR = crate::FieldReader<u16>;
#[doc = "Field `WD0HT` writer - High threshold for analog watchdog 0"]
pub type Wd0htW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - High threshold for analog watchdog 0"]
    #[inline(always)]
    pub fn wd0ht(&self) -> Wd0htR {
        Wd0htR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - High threshold for analog watchdog 0"]
    #[inline(always)]
    pub fn wd0ht(&mut self) -> Wd0htW<'_, Wd0htSpec> {
        Wd0htW::new(self, 0)
    }
}
#[doc = "Watchdog 0 high threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`wd0ht::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd0ht::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wd0htSpec;
impl crate::RegisterSpec for Wd0htSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wd0ht::R`](R) reader structure"]
impl crate::Readable for Wd0htSpec {}
#[doc = "`write(|w| ..)` method takes [`wd0ht::W`](W) writer structure"]
impl crate::Writable for Wd0htSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WD0HT to value 0x0fff"]
impl crate::Resettable for Wd0htSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
