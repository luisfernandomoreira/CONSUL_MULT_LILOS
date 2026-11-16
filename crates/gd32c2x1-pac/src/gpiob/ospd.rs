#[doc = "Register `OSPD` reader"]
pub type R = crate::R<OspdSpec>;
#[doc = "Register `OSPD` writer"]
pub type W = crate::W<OspdSpec>;
#[doc = "Field `OSPD0` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd0R = crate::BitReader;
#[doc = "Field `OSPD0` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD1` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd1R = crate::BitReader;
#[doc = "Field `OSPD1` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD2` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd2R = crate::BitReader;
#[doc = "Field `OSPD2` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD3` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd3R = crate::BitReader;
#[doc = "Field `OSPD3` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD4` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd4R = crate::BitReader;
#[doc = "Field `OSPD4` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD5` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd5R = crate::BitReader;
#[doc = "Field `OSPD5` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD6` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd6R = crate::BitReader;
#[doc = "Field `OSPD6` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD7` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd7R = crate::BitReader;
#[doc = "Field `OSPD7` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD8` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd8R = crate::BitReader;
#[doc = "Field `OSPD8` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD9` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd9R = crate::BitReader;
#[doc = "Field `OSPD9` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD10` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd10R = crate::BitReader;
#[doc = "Field `OSPD10` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD11` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd11R = crate::BitReader;
#[doc = "Field `OSPD11` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD12` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd12R = crate::BitReader;
#[doc = "Field `OSPD12` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD13` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd13R = crate::BitReader;
#[doc = "Field `OSPD13` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD14` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd14R = crate::BitReader;
#[doc = "Field `OSPD14` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPD15` reader - Port x configuration bits (y = 0..15)"]
pub type Ospd15R = crate::BitReader;
#[doc = "Field `OSPD15` writer - Port x configuration bits (y = 0..15)"]
pub type Ospd15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd0(&self) -> Ospd0R {
        Ospd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd1(&self) -> Ospd1R {
        Ospd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd2(&self) -> Ospd2R {
        Ospd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd3(&self) -> Ospd3R {
        Ospd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd4(&self) -> Ospd4R {
        Ospd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd5(&self) -> Ospd5R {
        Ospd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd6(&self) -> Ospd6R {
        Ospd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd7(&self) -> Ospd7R {
        Ospd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd8(&self) -> Ospd8R {
        Ospd8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd9(&self) -> Ospd9R {
        Ospd9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd10(&self) -> Ospd10R {
        Ospd10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd11(&self) -> Ospd11R {
        Ospd11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd12(&self) -> Ospd12R {
        Ospd12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd13(&self) -> Ospd13R {
        Ospd13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd14(&self) -> Ospd14R {
        Ospd14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd15(&self) -> Ospd15R {
        Ospd15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd0(&mut self) -> Ospd0W<'_, OspdSpec> {
        Ospd0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd1(&mut self) -> Ospd1W<'_, OspdSpec> {
        Ospd1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd2(&mut self) -> Ospd2W<'_, OspdSpec> {
        Ospd2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd3(&mut self) -> Ospd3W<'_, OspdSpec> {
        Ospd3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd4(&mut self) -> Ospd4W<'_, OspdSpec> {
        Ospd4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd5(&mut self) -> Ospd5W<'_, OspdSpec> {
        Ospd5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd6(&mut self) -> Ospd6W<'_, OspdSpec> {
        Ospd6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd7(&mut self) -> Ospd7W<'_, OspdSpec> {
        Ospd7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd8(&mut self) -> Ospd8W<'_, OspdSpec> {
        Ospd8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd9(&mut self) -> Ospd9W<'_, OspdSpec> {
        Ospd9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd10(&mut self) -> Ospd10W<'_, OspdSpec> {
        Ospd10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd11(&mut self) -> Ospd11W<'_, OspdSpec> {
        Ospd11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd12(&mut self) -> Ospd12W<'_, OspdSpec> {
        Ospd12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd13(&mut self) -> Ospd13W<'_, OspdSpec> {
        Ospd13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd14(&mut self) -> Ospd14W<'_, OspdSpec> {
        Ospd14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospd15(&mut self) -> Ospd15W<'_, OspdSpec> {
        Ospd15W::new(self, 15)
    }
}
#[doc = "GPIO port output speed register\n\nYou can [`read`](crate::Reg::read) this register and get [`ospd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OspdSpec;
impl crate::RegisterSpec for OspdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospd::R`](R) reader structure"]
impl crate::Readable for OspdSpec {}
#[doc = "`write(|w| ..)` method takes [`ospd::W`](W) writer structure"]
impl crate::Writable for OspdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSPD to value 0"]
impl crate::Resettable for OspdSpec {}
