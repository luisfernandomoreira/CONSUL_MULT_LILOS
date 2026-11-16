#[doc = "Register `PD` reader"]
pub type R = crate::R<PdSpec>;
#[doc = "Register `PD` writer"]
pub type W = crate::W<PdSpec>;
#[doc = "Field `PD0` reader - Interrupt pending status"]
pub type Pd0R = crate::BitReader;
#[doc = "Field `PD0` writer - Interrupt pending status"]
pub type Pd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Interrupt pending status"]
pub type Pd1R = crate::BitReader;
#[doc = "Field `PD1` writer - Interrupt pending status"]
pub type Pd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - Interrupt pending status"]
pub type Pd2R = crate::BitReader;
#[doc = "Field `PD2` writer - Interrupt pending status"]
pub type Pd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - Interrupt pending status"]
pub type Pd3R = crate::BitReader;
#[doc = "Field `PD3` writer - Interrupt pending status"]
pub type Pd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD4` reader - Interrupt pending status"]
pub type Pd4R = crate::BitReader;
#[doc = "Field `PD4` writer - Interrupt pending status"]
pub type Pd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD5` reader - Interrupt pending status"]
pub type Pd5R = crate::BitReader;
#[doc = "Field `PD5` writer - Interrupt pending status"]
pub type Pd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD6` reader - Interrupt pending status"]
pub type Pd6R = crate::BitReader;
#[doc = "Field `PD6` writer - Interrupt pending status"]
pub type Pd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7` reader - Interrupt pending status"]
pub type Pd7R = crate::BitReader;
#[doc = "Field `PD7` writer - Interrupt pending status"]
pub type Pd7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD8` reader - Interrupt pending status"]
pub type Pd8R = crate::BitReader;
#[doc = "Field `PD8` writer - Interrupt pending status"]
pub type Pd8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD9` reader - Interrupt pending status"]
pub type Pd9R = crate::BitReader;
#[doc = "Field `PD9` writer - Interrupt pending status"]
pub type Pd9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD10` reader - Interrupt pending status"]
pub type Pd10R = crate::BitReader;
#[doc = "Field `PD10` writer - Interrupt pending status"]
pub type Pd10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD11` reader - Interrupt pending status"]
pub type Pd11R = crate::BitReader;
#[doc = "Field `PD11` writer - Interrupt pending status"]
pub type Pd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD12` reader - Interrupt pending status"]
pub type Pd12R = crate::BitReader;
#[doc = "Field `PD12` writer - Interrupt pending status"]
pub type Pd12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD13` reader - Interrupt pending status"]
pub type Pd13R = crate::BitReader;
#[doc = "Field `PD13` writer - Interrupt pending status"]
pub type Pd13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD14` reader - Interrupt pending status"]
pub type Pd14R = crate::BitReader;
#[doc = "Field `PD14` writer - Interrupt pending status"]
pub type Pd14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD15` reader - Interrupt pending status"]
pub type Pd15R = crate::BitReader;
#[doc = "Field `PD15` writer - Interrupt pending status"]
pub type Pd15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD16` reader - Interrupt pending status"]
pub type Pd16R = crate::BitReader;
#[doc = "Field `PD16` writer - Interrupt pending status"]
pub type Pd16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD17` reader - Interrupt pending status"]
pub type Pd17R = crate::BitReader;
#[doc = "Field `PD17` writer - Interrupt pending status"]
pub type Pd17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD18` reader - Interrupt pending status"]
pub type Pd18R = crate::BitReader;
#[doc = "Field `PD18` writer - Interrupt pending status"]
pub type Pd18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD19` reader - Interrupt pending status"]
pub type Pd19R = crate::BitReader;
#[doc = "Field `PD19` writer - Interrupt pending status"]
pub type Pd19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD20` reader - Interrupt pending status"]
pub type Pd20R = crate::BitReader;
#[doc = "Field `PD20` writer - Interrupt pending status"]
pub type Pd20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD21` reader - Interrupt pending status"]
pub type Pd21R = crate::BitReader;
#[doc = "Field `PD21` writer - Interrupt pending status"]
pub type Pd21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD22` reader - Interrupt pending status"]
pub type Pd22R = crate::BitReader;
#[doc = "Field `PD22` writer - Interrupt pending status"]
pub type Pd22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD23` reader - Interrupt pending status"]
pub type Pd23R = crate::BitReader;
#[doc = "Field `PD23` writer - Interrupt pending status"]
pub type Pd23W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd0(&self) -> Pd0R {
        Pd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd1(&self) -> Pd1R {
        Pd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd2(&self) -> Pd2R {
        Pd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd3(&self) -> Pd3R {
        Pd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd4(&self) -> Pd4R {
        Pd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd5(&self) -> Pd5R {
        Pd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd6(&self) -> Pd6R {
        Pd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd7(&self) -> Pd7R {
        Pd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd8(&self) -> Pd8R {
        Pd8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd9(&self) -> Pd9R {
        Pd9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd10(&self) -> Pd10R {
        Pd10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd11(&self) -> Pd11R {
        Pd11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd12(&self) -> Pd12R {
        Pd12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd13(&self) -> Pd13R {
        Pd13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd14(&self) -> Pd14R {
        Pd14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd15(&self) -> Pd15R {
        Pd15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd16(&self) -> Pd16R {
        Pd16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd17(&self) -> Pd17R {
        Pd17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd18(&self) -> Pd18R {
        Pd18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd19(&self) -> Pd19R {
        Pd19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd20(&self) -> Pd20R {
        Pd20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd21(&self) -> Pd21R {
        Pd21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd22(&self) -> Pd22R {
        Pd22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd23(&self) -> Pd23R {
        Pd23R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd0(&mut self) -> Pd0W<'_, PdSpec> {
        Pd0W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd1(&mut self) -> Pd1W<'_, PdSpec> {
        Pd1W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd2(&mut self) -> Pd2W<'_, PdSpec> {
        Pd2W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd3(&mut self) -> Pd3W<'_, PdSpec> {
        Pd3W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd4(&mut self) -> Pd4W<'_, PdSpec> {
        Pd4W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd5(&mut self) -> Pd5W<'_, PdSpec> {
        Pd5W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd6(&mut self) -> Pd6W<'_, PdSpec> {
        Pd6W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd7(&mut self) -> Pd7W<'_, PdSpec> {
        Pd7W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd8(&mut self) -> Pd8W<'_, PdSpec> {
        Pd8W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd9(&mut self) -> Pd9W<'_, PdSpec> {
        Pd9W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd10(&mut self) -> Pd10W<'_, PdSpec> {
        Pd10W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd11(&mut self) -> Pd11W<'_, PdSpec> {
        Pd11W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd12(&mut self) -> Pd12W<'_, PdSpec> {
        Pd12W::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd13(&mut self) -> Pd13W<'_, PdSpec> {
        Pd13W::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd14(&mut self) -> Pd14W<'_, PdSpec> {
        Pd14W::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd15(&mut self) -> Pd15W<'_, PdSpec> {
        Pd15W::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd16(&mut self) -> Pd16W<'_, PdSpec> {
        Pd16W::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd17(&mut self) -> Pd17W<'_, PdSpec> {
        Pd17W::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd18(&mut self) -> Pd18W<'_, PdSpec> {
        Pd18W::new(self, 18)
    }
    #[doc = "Bit 19 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd19(&mut self) -> Pd19W<'_, PdSpec> {
        Pd19W::new(self, 19)
    }
    #[doc = "Bit 20 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd20(&mut self) -> Pd20W<'_, PdSpec> {
        Pd20W::new(self, 20)
    }
    #[doc = "Bit 21 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd21(&mut self) -> Pd21W<'_, PdSpec> {
        Pd21W::new(self, 21)
    }
    #[doc = "Bit 22 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd22(&mut self) -> Pd22W<'_, PdSpec> {
        Pd22W::new(self, 22)
    }
    #[doc = "Bit 23 - Interrupt pending status"]
    #[inline(always)]
    pub fn pd23(&mut self) -> Pd23W<'_, PdSpec> {
        Pd23W::new(self, 23)
    }
}
#[doc = "Pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdSpec;
impl crate::RegisterSpec for PdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd::R`](R) reader structure"]
impl crate::Readable for PdSpec {}
#[doc = "`write(|w| ..)` method takes [`pd::W`](W) writer structure"]
impl crate::Writable for PdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD to value 0"]
impl crate::Resettable for PdSpec {}
