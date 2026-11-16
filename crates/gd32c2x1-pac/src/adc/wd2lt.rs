#[doc = "Register `WD2LT` reader"]
pub type R = crate::R<Wd2ltSpec>;
#[doc = "Register `WD2LT` writer"]
pub type W = crate::W<Wd2ltSpec>;
#[doc = "Field `WD2LT` reader - Low threshold for analog watchdog 2"]
pub type Wd2ltR = crate::FieldReader<u16>;
#[doc = "Field `WD2LT` writer - Low threshold for analog watchdog 2"]
pub type Wd2ltW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Low threshold for analog watchdog 2"]
    #[inline(always)]
    pub fn wd2lt(&self) -> Wd2ltR {
        Wd2ltR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Low threshold for analog watchdog 2"]
    #[inline(always)]
    pub fn wd2lt(&mut self) -> Wd2ltW<'_, Wd2ltSpec> {
        Wd2ltW::new(self, 0)
    }
}
#[doc = "Watchdog 2 low threshold register2\n\nYou can [`read`](crate::Reg::read) this register and get [`wd2lt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd2lt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wd2ltSpec;
impl crate::RegisterSpec for Wd2ltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wd2lt::R`](R) reader structure"]
impl crate::Readable for Wd2ltSpec {}
#[doc = "`write(|w| ..)` method takes [`wd2lt::W`](W) writer structure"]
impl crate::Writable for Wd2ltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WD2LT to value 0"]
impl crate::Resettable for Wd2ltSpec {}
