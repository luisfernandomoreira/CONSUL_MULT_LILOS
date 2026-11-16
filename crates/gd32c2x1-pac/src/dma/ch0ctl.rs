#[doc = "Register `CH0CTL` reader"]
pub type R = crate::R<Ch0ctlSpec>;
#[doc = "Register `CH0CTL` writer"]
pub type W = crate::W<Ch0ctlSpec>;
#[doc = "Field `CHEN` reader - Channel enable"]
pub type ChenR = crate::BitReader;
#[doc = "Field `CHEN` writer - Channel enable"]
pub type ChenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTFIE` reader - Full Transfer Finish interrupt enable"]
pub type FtfieR = crate::BitReader;
#[doc = "Field `FTFIE` writer - Full Transfer Finish interrupt enable"]
pub type FtfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTFIE` reader - Half Transfer Finish interrupt enable"]
pub type HtfieR = crate::BitReader;
#[doc = "Field `HTFIE` writer - Half Transfer Finish interrupt enable"]
pub type HtfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Transfer direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Transfer direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMEN` reader - Circular mode enable"]
pub type CmenR = crate::BitReader;
#[doc = "Field `CMEN` writer - Circular mode enable"]
pub type CmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PNAGA` reader - Next address generation algorithm of peripheral"]
pub type PnagaR = crate::BitReader;
#[doc = "Field `PNAGA` writer - Next address generation algorithm of peripheral"]
pub type PnagaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MNAGA` reader - Next address generation algorithm of memory"]
pub type MnagaR = crate::BitReader;
#[doc = "Field `MNAGA` writer - Next address generation algorithm of memory"]
pub type MnagaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWIDTH` reader - Transfer data size of peripheral"]
pub type PwidthR = crate::FieldReader;
#[doc = "Field `PWIDTH` writer - Transfer data size of peripheral"]
pub type PwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MWIDTH` reader - Transfer data size of memory"]
pub type MwidthR = crate::FieldReader;
#[doc = "Field `MWIDTH` writer - Transfer data size of memory"]
pub type MwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRIO` reader - Priority Level of this channel"]
pub type PrioR = crate::FieldReader;
#[doc = "Field `PRIO` writer - Priority Level of this channel"]
pub type PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M2M` reader - Memory to memory mode"]
pub type M2mR = crate::BitReader;
#[doc = "Field `M2M` writer - Memory to memory mode"]
pub type M2mW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&self) -> ChenR {
        ChenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full Transfer Finish interrupt enable"]
    #[inline(always)]
    pub fn ftfie(&self) -> FtfieR {
        FtfieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half Transfer Finish interrupt enable"]
    #[inline(always)]
    pub fn htfie(&self) -> HtfieR {
        HtfieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Circular mode enable"]
    #[inline(always)]
    pub fn cmen(&self) -> CmenR {
        CmenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    pub fn pnaga(&self) -> PnagaR {
        PnagaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Next address generation algorithm of memory"]
    #[inline(always)]
    pub fn mnaga(&self) -> MnagaR {
        MnagaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Transfer data size of peripheral"]
    #[inline(always)]
    pub fn pwidth(&self) -> PwidthR {
        PwidthR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Transfer data size of memory"]
    #[inline(always)]
    pub fn mwidth(&self) -> MwidthR {
        MwidthR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Priority Level of this channel"]
    #[inline(always)]
    pub fn prio(&self) -> PrioR {
        PrioR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    pub fn m2m(&self) -> M2mR {
        M2mR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&mut self) -> ChenW<'_, Ch0ctlSpec> {
        ChenW::new(self, 0)
    }
    #[doc = "Bit 1 - Full Transfer Finish interrupt enable"]
    #[inline(always)]
    pub fn ftfie(&mut self) -> FtfieW<'_, Ch0ctlSpec> {
        FtfieW::new(self, 1)
    }
    #[doc = "Bit 2 - Half Transfer Finish interrupt enable"]
    #[inline(always)]
    pub fn htfie(&mut self) -> HtfieW<'_, Ch0ctlSpec> {
        HtfieW::new(self, 2)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, Ch0ctlSpec> {
        ErrieW::new(self, 3)
    }
    #[doc = "Bit 4 - Transfer direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, Ch0ctlSpec> {
        DirW::new(self, 4)
    }
    #[doc = "Bit 5 - Circular mode enable"]
    #[inline(always)]
    pub fn cmen(&mut self) -> CmenW<'_, Ch0ctlSpec> {
        CmenW::new(self, 5)
    }
    #[doc = "Bit 6 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    pub fn pnaga(&mut self) -> PnagaW<'_, Ch0ctlSpec> {
        PnagaW::new(self, 6)
    }
    #[doc = "Bit 7 - Next address generation algorithm of memory"]
    #[inline(always)]
    pub fn mnaga(&mut self) -> MnagaW<'_, Ch0ctlSpec> {
        MnagaW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Transfer data size of peripheral"]
    #[inline(always)]
    pub fn pwidth(&mut self) -> PwidthW<'_, Ch0ctlSpec> {
        PwidthW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Transfer data size of memory"]
    #[inline(always)]
    pub fn mwidth(&mut self) -> MwidthW<'_, Ch0ctlSpec> {
        MwidthW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Priority Level of this channel"]
    #[inline(always)]
    pub fn prio(&mut self) -> PrioW<'_, Ch0ctlSpec> {
        PrioW::new(self, 12)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    pub fn m2m(&mut self) -> M2mW<'_, Ch0ctlSpec> {
        M2mW::new(self, 14)
    }
}
#[doc = "DMA channel configuration register (DMA_CH0CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0ctlSpec;
impl crate::RegisterSpec for Ch0ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0ctl::R`](R) reader structure"]
impl crate::Readable for Ch0ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0ctl::W`](W) writer structure"]
impl crate::Writable for Ch0ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH0CTL to value 0"]
impl crate::Resettable for Ch0ctlSpec {}
