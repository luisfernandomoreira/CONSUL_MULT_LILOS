#[doc = "Register `PUD` reader"]
pub type R = crate::R<PudSpec>;
#[doc = "Register `PUD` writer"]
pub type W = crate::W<PudSpec>;
#[doc = "Field `PUD0` reader - Port x configuration bits (y = 0..15)"]
pub type Pud0R = crate::FieldReader;
#[doc = "Field `PUD0` writer - Port x configuration bits (y = 0..15)"]
pub type Pud0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD1` reader - Port x configuration bits (y = 0..15)"]
pub type Pud1R = crate::FieldReader;
#[doc = "Field `PUD1` writer - Port x configuration bits (y = 0..15)"]
pub type Pud1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD2` reader - Port x configuration bits (y = 0..15)"]
pub type Pud2R = crate::FieldReader;
#[doc = "Field `PUD2` writer - Port x configuration bits (y = 0..15)"]
pub type Pud2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD3` reader - Port x configuration bits (y = 0..15)"]
pub type Pud3R = crate::FieldReader;
#[doc = "Field `PUD3` writer - Port x configuration bits (y = 0..15)"]
pub type Pud3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD4` reader - Port x configuration bits (y = 0..15)"]
pub type Pud4R = crate::FieldReader;
#[doc = "Field `PUD4` writer - Port x configuration bits (y = 0..15)"]
pub type Pud4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD5` reader - Port x configuration bits (y = 0..15)"]
pub type Pud5R = crate::FieldReader;
#[doc = "Field `PUD5` writer - Port x configuration bits (y = 0..15)"]
pub type Pud5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD6` reader - Port x configuration bits (y = 0..15)"]
pub type Pud6R = crate::FieldReader;
#[doc = "Field `PUD6` writer - Port x configuration bits (y = 0..15)"]
pub type Pud6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD7` reader - Port x configuration bits (y = 0..15)"]
pub type Pud7R = crate::FieldReader;
#[doc = "Field `PUD7` writer - Port x configuration bits (y = 0..15)"]
pub type Pud7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD8` reader - Port x configuration bits (y = 0..15)"]
pub type Pud8R = crate::FieldReader;
#[doc = "Field `PUD8` writer - Port x configuration bits (y = 0..15)"]
pub type Pud8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD9` reader - Port x configuration bits (y = 0..15)"]
pub type Pud9R = crate::FieldReader;
#[doc = "Field `PUD9` writer - Port x configuration bits (y = 0..15)"]
pub type Pud9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD10` reader - Port x configuration bits (y = 0..15)"]
pub type Pud10R = crate::FieldReader;
#[doc = "Field `PUD10` writer - Port x configuration bits (y = 0..15)"]
pub type Pud10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD11` reader - Port x configuration bits (y = 0..15)"]
pub type Pud11R = crate::FieldReader;
#[doc = "Field `PUD11` writer - Port x configuration bits (y = 0..15)"]
pub type Pud11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD12` reader - Port x configuration bits (y = 0..15)"]
pub type Pud12R = crate::FieldReader;
#[doc = "Field `PUD12` writer - Port x configuration bits (y = 0..15)"]
pub type Pud12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD13` reader - Port x configuration bits (y = 0..15)"]
pub type Pud13R = crate::FieldReader;
#[doc = "Field `PUD13` writer - Port x configuration bits (y = 0..15)"]
pub type Pud13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD14` reader - Port x configuration bits (y = 0..15)"]
pub type Pud14R = crate::FieldReader;
#[doc = "Field `PUD14` writer - Port x configuration bits (y = 0..15)"]
pub type Pud14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PUD15` reader - Port x configuration bits (y = 0..15)"]
pub type Pud15R = crate::FieldReader;
#[doc = "Field `PUD15` writer - Port x configuration bits (y = 0..15)"]
pub type Pud15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud0(&self) -> Pud0R {
        Pud0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud1(&self) -> Pud1R {
        Pud1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud2(&self) -> Pud2R {
        Pud2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud3(&self) -> Pud3R {
        Pud3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud4(&self) -> Pud4R {
        Pud4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud5(&self) -> Pud5R {
        Pud5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud6(&self) -> Pud6R {
        Pud6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud7(&self) -> Pud7R {
        Pud7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud8(&self) -> Pud8R {
        Pud8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud9(&self) -> Pud9R {
        Pud9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud10(&self) -> Pud10R {
        Pud10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud11(&self) -> Pud11R {
        Pud11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud12(&self) -> Pud12R {
        Pud12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud13(&self) -> Pud13R {
        Pud13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud14(&self) -> Pud14R {
        Pud14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud15(&self) -> Pud15R {
        Pud15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud0(&mut self) -> Pud0W<'_, PudSpec> {
        Pud0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud1(&mut self) -> Pud1W<'_, PudSpec> {
        Pud1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud2(&mut self) -> Pud2W<'_, PudSpec> {
        Pud2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud3(&mut self) -> Pud3W<'_, PudSpec> {
        Pud3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud4(&mut self) -> Pud4W<'_, PudSpec> {
        Pud4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud5(&mut self) -> Pud5W<'_, PudSpec> {
        Pud5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud6(&mut self) -> Pud6W<'_, PudSpec> {
        Pud6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud7(&mut self) -> Pud7W<'_, PudSpec> {
        Pud7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud8(&mut self) -> Pud8W<'_, PudSpec> {
        Pud8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud9(&mut self) -> Pud9W<'_, PudSpec> {
        Pud9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud10(&mut self) -> Pud10W<'_, PudSpec> {
        Pud10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud11(&mut self) -> Pud11W<'_, PudSpec> {
        Pud11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud12(&mut self) -> Pud12W<'_, PudSpec> {
        Pud12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud13(&mut self) -> Pud13W<'_, PudSpec> {
        Pud13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud14(&mut self) -> Pud14W<'_, PudSpec> {
        Pud14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn pud15(&mut self) -> Pud15W<'_, PudSpec> {
        Pud15W::new(self, 30)
    }
}
#[doc = "GPIO port pull-up/pull-down register\n\nYou can [`read`](crate::Reg::read) this register and get [`pud::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pud::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PudSpec;
impl crate::RegisterSpec for PudSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pud::R`](R) reader structure"]
impl crate::Readable for PudSpec {}
#[doc = "`write(|w| ..)` method takes [`pud::W`](W) writer structure"]
impl crate::Writable for PudSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUD to value 0"]
impl crate::Resettable for PudSpec {}
