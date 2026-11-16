#[doc = "Register `TIMER0CFG0` reader"]
pub type R = crate::R<Timer0cfg0Spec>;
#[doc = "Register `TIMER0CFG0` writer"]
pub type W = crate::W<Timer0cfg0Spec>;
#[doc = "Field `TSCFG0` reader - Quadrature decoder mode 0 configuration"]
pub type Tscfg0R = crate::FieldReader;
#[doc = "Field `TSCFG0` writer - Quadrature decoder mode 0 configuration"]
pub type Tscfg0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSCFG1` reader - Quadrature decoder mode 1 configuration"]
pub type Tscfg1R = crate::FieldReader;
#[doc = "Field `TSCFG1` writer - Quadrature decoder mode 1 configuration"]
pub type Tscfg1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSCFG2` reader - Quadrature decoder mode 2 configuration"]
pub type Tscfg2R = crate::FieldReader;
#[doc = "Field `TSCFG2` writer - Quadrature decoder mode 2 configuration"]
pub type Tscfg2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSCFG3` reader - Restart mode configuration"]
pub type Tscfg3R = crate::FieldReader;
#[doc = "Field `TSCFG3` writer - Restart mode configuration"]
pub type Tscfg3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSCFG4` reader - Pause mode configuration"]
pub type Tscfg4R = crate::FieldReader;
#[doc = "Field `TSCFG4` writer - Pause mode configuration"]
pub type Tscfg4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSCFG5` reader - Event mode configuration"]
pub type Tscfg5R = crate::FieldReader;
#[doc = "Field `TSCFG5` writer - Event mode configuration"]
pub type Tscfg5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSCFG6` reader - External clock mode 0 configuration"]
pub type Tscfg6R = crate::FieldReader;
#[doc = "Field `TSCFG6` writer - External clock mode 0 configuration"]
pub type Tscfg6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSCFG7` reader - Restart event mode configuration"]
pub type Tscfg7R = crate::FieldReader;
#[doc = "Field `TSCFG7` writer - Restart event mode configuration"]
pub type Tscfg7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Quadrature decoder mode 0 configuration"]
    #[inline(always)]
    pub fn tscfg0(&self) -> Tscfg0R {
        Tscfg0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Quadrature decoder mode 1 configuration"]
    #[inline(always)]
    pub fn tscfg1(&self) -> Tscfg1R {
        Tscfg1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Quadrature decoder mode 2 configuration"]
    #[inline(always)]
    pub fn tscfg2(&self) -> Tscfg2R {
        Tscfg2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Restart mode configuration"]
    #[inline(always)]
    pub fn tscfg3(&self) -> Tscfg3R {
        Tscfg3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Pause mode configuration"]
    #[inline(always)]
    pub fn tscfg4(&self) -> Tscfg4R {
        Tscfg4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Event mode configuration"]
    #[inline(always)]
    pub fn tscfg5(&self) -> Tscfg5R {
        Tscfg5R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - External clock mode 0 configuration"]
    #[inline(always)]
    pub fn tscfg6(&self) -> Tscfg6R {
        Tscfg6R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Restart event mode configuration"]
    #[inline(always)]
    pub fn tscfg7(&self) -> Tscfg7R {
        Tscfg7R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Quadrature decoder mode 0 configuration"]
    #[inline(always)]
    pub fn tscfg0(&mut self) -> Tscfg0W<'_, Timer0cfg0Spec> {
        Tscfg0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Quadrature decoder mode 1 configuration"]
    #[inline(always)]
    pub fn tscfg1(&mut self) -> Tscfg1W<'_, Timer0cfg0Spec> {
        Tscfg1W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Quadrature decoder mode 2 configuration"]
    #[inline(always)]
    pub fn tscfg2(&mut self) -> Tscfg2W<'_, Timer0cfg0Spec> {
        Tscfg2W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Restart mode configuration"]
    #[inline(always)]
    pub fn tscfg3(&mut self) -> Tscfg3W<'_, Timer0cfg0Spec> {
        Tscfg3W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Pause mode configuration"]
    #[inline(always)]
    pub fn tscfg4(&mut self) -> Tscfg4W<'_, Timer0cfg0Spec> {
        Tscfg4W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Event mode configuration"]
    #[inline(always)]
    pub fn tscfg5(&mut self) -> Tscfg5W<'_, Timer0cfg0Spec> {
        Tscfg5W::new(self, 20)
    }
    #[doc = "Bits 24:26 - External clock mode 0 configuration"]
    #[inline(always)]
    pub fn tscfg6(&mut self) -> Tscfg6W<'_, Timer0cfg0Spec> {
        Tscfg6W::new(self, 24)
    }
    #[doc = "Bits 28:30 - Restart event mode configuration"]
    #[inline(always)]
    pub fn tscfg7(&mut self) -> Tscfg7W<'_, Timer0cfg0Spec> {
        Tscfg7W::new(self, 28)
    }
}
#[doc = "TIMERx configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timer0cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer0cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer0cfg0Spec;
impl crate::RegisterSpec for Timer0cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer0cfg0::R`](R) reader structure"]
impl crate::Readable for Timer0cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`timer0cfg0::W`](W) writer structure"]
impl crate::Writable for Timer0cfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER0CFG0 to value 0"]
impl crate::Resettable for Timer0cfg0Spec {}
