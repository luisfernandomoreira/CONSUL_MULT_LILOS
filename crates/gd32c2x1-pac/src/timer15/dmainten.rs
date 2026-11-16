#[doc = "Register `DMAINTEN` reader"]
pub type R = crate::R<DmaintenSpec>;
#[doc = "Register `DMAINTEN` writer"]
pub type W = crate::W<DmaintenSpec>;
#[doc = "Field `UPIE` reader - Update interrupt enable"]
pub type UpieR = crate::BitReader;
#[doc = "Field `UPIE` writer - Update interrupt enable"]
pub type UpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0IE` reader - Channel 0 capture/compare interrupt enable"]
pub type Ch0ieR = crate::BitReader;
#[doc = "Field `CH0IE` writer - Channel 0 capture/compare interrupt enable"]
pub type Ch0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMTIE` reader - Commutation interrupt enable"]
pub type CmtieR = crate::BitReader;
#[doc = "Field `CMTIE` writer - Commutation interrupt enable"]
pub type CmtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK0IE` reader - Break interrupt 0 enable"]
pub type Brk0ieR = crate::BitReader;
#[doc = "Field `BRK0IE` writer - Break interrupt 0 enable"]
pub type Brk0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDEN` reader - Update DMA request enable"]
pub type UpdenR = crate::BitReader;
#[doc = "Field `UPDEN` writer - Update DMA request enable"]
pub type UpdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0DEN` reader - Channel 0 capture/compare DMA request enable"]
pub type Ch0denR = crate::BitReader;
#[doc = "Field `CH0DEN` writer - Channel 0 capture/compare DMA request enable"]
pub type Ch0denW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UpieR {
        UpieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&self) -> Ch0ieR {
        Ch0ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Commutation interrupt enable"]
    #[inline(always)]
    pub fn cmtie(&self) -> CmtieR {
        CmtieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt 0 enable"]
    #[inline(always)]
    pub fn brk0ie(&self) -> Brk0ieR {
        Brk0ieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&self) -> UpdenR {
        UpdenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 0 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&self) -> Ch0denR {
        Ch0denR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn upie(&mut self) -> UpieW<'_, DmaintenSpec> {
        UpieW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 0 capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ch0ie(&mut self) -> Ch0ieW<'_, DmaintenSpec> {
        Ch0ieW::new(self, 1)
    }
    #[doc = "Bit 5 - Commutation interrupt enable"]
    #[inline(always)]
    pub fn cmtie(&mut self) -> CmtieW<'_, DmaintenSpec> {
        CmtieW::new(self, 5)
    }
    #[doc = "Bit 7 - Break interrupt 0 enable"]
    #[inline(always)]
    pub fn brk0ie(&mut self) -> Brk0ieW<'_, DmaintenSpec> {
        Brk0ieW::new(self, 7)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn upden(&mut self) -> UpdenW<'_, DmaintenSpec> {
        UpdenW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 0 capture/compare DMA request enable"]
    #[inline(always)]
    pub fn ch0den(&mut self) -> Ch0denW<'_, DmaintenSpec> {
        Ch0denW::new(self, 9)
    }
}
#[doc = "DMA and interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmainten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmainten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaintenSpec;
impl crate::RegisterSpec for DmaintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmainten::R`](R) reader structure"]
impl crate::Readable for DmaintenSpec {}
#[doc = "`write(|w| ..)` method takes [`dmainten::W`](W) writer structure"]
impl crate::Writable for DmaintenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAINTEN to value 0"]
impl crate::Resettable for DmaintenSpec {}
