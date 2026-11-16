#[doc = "Register `AFSEL0` reader"]
pub type R = crate::R<Afsel0Spec>;
#[doc = "Register `AFSEL0` writer"]
pub type W = crate::W<Afsel0Spec>;
#[doc = "Field `SEL0` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel0R = crate::FieldReader;
#[doc = "Field `SEL0` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL1` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel1R = crate::FieldReader;
#[doc = "Field `SEL1` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL2` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel2R = crate::FieldReader;
#[doc = "Field `SEL2` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL3` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel3R = crate::FieldReader;
#[doc = "Field `SEL3` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL4` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel4R = crate::FieldReader;
#[doc = "Field `SEL4` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL5` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel5R = crate::FieldReader;
#[doc = "Field `SEL5` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL6` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel6R = crate::FieldReader;
#[doc = "Field `SEL6` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL7` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel7R = crate::FieldReader;
#[doc = "Field `SEL7` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type Sel7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel0(&self) -> Sel0R {
        Sel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel1(&self) -> Sel1R {
        Sel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel2(&self) -> Sel2R {
        Sel2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel3(&self) -> Sel3R {
        Sel3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel4(&self) -> Sel4R {
        Sel4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel5(&self) -> Sel5R {
        Sel5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel6(&self) -> Sel6R {
        Sel6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel7(&self) -> Sel7R {
        Sel7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel0(&mut self) -> Sel0W<'_, Afsel0Spec> {
        Sel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel1(&mut self) -> Sel1W<'_, Afsel0Spec> {
        Sel1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel2(&mut self) -> Sel2W<'_, Afsel0Spec> {
        Sel2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel3(&mut self) -> Sel3W<'_, Afsel0Spec> {
        Sel3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel4(&mut self) -> Sel4W<'_, Afsel0Spec> {
        Sel4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel5(&mut self) -> Sel5W<'_, Afsel0Spec> {
        Sel5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel6(&mut self) -> Sel6W<'_, Afsel0Spec> {
        Sel6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn sel7(&mut self) -> Sel7W<'_, Afsel0Spec> {
        Sel7W::new(self, 28)
    }
}
#[doc = "GPIO alternate function low register\n\nYou can [`read`](crate::Reg::read) this register and get [`afsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Afsel0Spec;
impl crate::RegisterSpec for Afsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afsel0::R`](R) reader structure"]
impl crate::Readable for Afsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`afsel0::W`](W) writer structure"]
impl crate::Writable for Afsel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFSEL0 to value 0"]
impl crate::Resettable for Afsel0Spec {}
