#[doc = "Register `INTC` writer"]
pub type W = crate::W<IntcSpec>;
#[doc = "Field `GIFC0` writer - Channel 0 Global interrupt flag clear"]
pub type Gifc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC0` writer - Channel 0 Full Transfer Finish clear"]
pub type Ftfifc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC0` writer - Channel 0 Half Transfer clear"]
pub type Htfifc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC0` writer - Channel 0 Error clear"]
pub type Errifc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIFC1` writer - Channel 1 Global interrupt flag clear"]
pub type Gifc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC1` writer - Channel 1 Full Transfer Finish clear"]
pub type Ftfifc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC1` writer - Channel 1 Half Transfer clear"]
pub type Htfifc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC1` writer - Channel 1 Error clear"]
pub type Errifc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GIFC2` writer - Channel 2 Global interrupt flag clear"]
pub type Gifc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIFC2` writer - Channel 2 Full Transfer Finish clear"]
pub type Ftfifc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIFC2` writer - Channel 2 Half Transfer clear"]
pub type Htfifc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIFC2` writer - Channel 2 Error clear"]
pub type Errifc2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Global interrupt flag clear"]
    #[inline(always)]
    pub fn gifc0(&mut self) -> Gifc0W<'_, IntcSpec> {
        Gifc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 Full Transfer Finish clear"]
    #[inline(always)]
    pub fn ftfifc0(&mut self) -> Ftfifc0W<'_, IntcSpec> {
        Ftfifc0W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 0 Half Transfer clear"]
    #[inline(always)]
    pub fn htfifc0(&mut self) -> Htfifc0W<'_, IntcSpec> {
        Htfifc0W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 0 Error clear"]
    #[inline(always)]
    pub fn errifc0(&mut self) -> Errifc0W<'_, IntcSpec> {
        Errifc0W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 1 Global interrupt flag clear"]
    #[inline(always)]
    pub fn gifc1(&mut self) -> Gifc1W<'_, IntcSpec> {
        Gifc1W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 1 Full Transfer Finish clear"]
    #[inline(always)]
    pub fn ftfifc1(&mut self) -> Ftfifc1W<'_, IntcSpec> {
        Ftfifc1W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 1 Half Transfer clear"]
    #[inline(always)]
    pub fn htfifc1(&mut self) -> Htfifc1W<'_, IntcSpec> {
        Htfifc1W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 1 Error clear"]
    #[inline(always)]
    pub fn errifc1(&mut self) -> Errifc1W<'_, IntcSpec> {
        Errifc1W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 2 Global interrupt flag clear"]
    #[inline(always)]
    pub fn gifc2(&mut self) -> Gifc2W<'_, IntcSpec> {
        Gifc2W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 2 Full Transfer Finish clear"]
    #[inline(always)]
    pub fn ftfifc2(&mut self) -> Ftfifc2W<'_, IntcSpec> {
        Ftfifc2W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 Half Transfer clear"]
    #[inline(always)]
    pub fn htfifc2(&mut self) -> Htfifc2W<'_, IntcSpec> {
        Htfifc2W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 2 Error clear"]
    #[inline(always)]
    pub fn errifc2(&mut self) -> Errifc2W<'_, IntcSpec> {
        Errifc2W::new(self, 11)
    }
}
#[doc = "DMA interrupt flag clear register (DMA_INTC)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcSpec;
impl crate::RegisterSpec for IntcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intc::W`](W) writer structure"]
impl crate::Writable for IntcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC to value 0"]
impl crate::Resettable for IntcSpec {}
