#[doc = "Register `WD0LT` reader"]
pub type R = crate::R<Wd0ltSpec>;
#[doc = "Register `WD0LT` writer"]
pub type W = crate::W<Wd0ltSpec>;
#[doc = "Field `WD0LT` reader - Low threshold for analog watchdog 0"]
pub type Wd0ltR = crate::FieldReader<u16>;
#[doc = "Field `WD0LT` writer - Low threshold for analog watchdog 0"]
pub type Wd0ltW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Low threshold for analog watchdog 0"]
    #[inline(always)]
    pub fn wd0lt(&self) -> Wd0ltR {
        Wd0ltR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Low threshold for analog watchdog 0"]
    #[inline(always)]
    pub fn wd0lt(&mut self) -> Wd0ltW<'_, Wd0ltSpec> {
        Wd0ltW::new(self, 0)
    }
}
#[doc = "Watchdog 0 low threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`wd0lt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd0lt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wd0ltSpec;
impl crate::RegisterSpec for Wd0ltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wd0lt::R`](R) reader structure"]
impl crate::Readable for Wd0ltSpec {}
#[doc = "`write(|w| ..)` method takes [`wd0lt::W`](W) writer structure"]
impl crate::Writable for Wd0ltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WD0LT to value 0"]
impl crate::Resettable for Wd0ltSpec {}
