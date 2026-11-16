#[doc = "Register `TIMER2CFG1` reader"]
pub type R = crate::R<Timer2cfg1Spec>;
#[doc = "Register `TIMER2CFG1` writer"]
pub type W = crate::W<Timer2cfg1Spec>;
#[doc = "Field `TSCFG8` reader - Internal trigger input source configuration"]
pub type Tscfg8R = crate::FieldReader;
#[doc = "Field `TSCFG8` writer - Internal trigger input source configuration"]
pub type Tscfg8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Internal trigger input source configuration"]
    #[inline(always)]
    pub fn tscfg8(&self) -> Tscfg8R {
        Tscfg8R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Internal trigger input source configuration"]
    #[inline(always)]
    pub fn tscfg8(&mut self) -> Tscfg8W<'_, Timer2cfg1Spec> {
        Tscfg8W::new(self, 0)
    }
}
#[doc = "TIMER2 configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`timer2cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer2cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2cfg1Spec;
impl crate::RegisterSpec for Timer2cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2cfg1::R`](R) reader structure"]
impl crate::Readable for Timer2cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`timer2cfg1::W`](W) writer structure"]
impl crate::Writable for Timer2cfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER2CFG1 to value 0"]
impl crate::Resettable for Timer2cfg1Spec {}
