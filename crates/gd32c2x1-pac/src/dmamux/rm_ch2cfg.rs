#[doc = "Register `RM_CH2CFG` reader"]
pub type R = crate::R<RmCh2cfgSpec>;
#[doc = "Register `RM_CH2CFG` writer"]
pub type W = crate::W<RmCh2cfgSpec>;
#[doc = "Field `MUXINID` reader - Multiplexer input identification"]
pub type MuxinidR = crate::FieldReader;
#[doc = "Field `MUXINID` writer - Multiplexer input identification"]
pub type MuxinidW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SOIE` reader - Synchronization overrun interrupt enable"]
pub type SoieR = crate::BitReader;
#[doc = "Field `SOIE` writer - Synchronization overrun interrupt enable"]
pub type SoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVGEN` reader - Event generation enable"]
pub type EvgenR = crate::BitReader;
#[doc = "Field `EVGEN` writer - Event generation enable"]
pub type EvgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCEN` reader - Synchronization enable"]
pub type SyncenR = crate::BitReader;
#[doc = "Field `SYNCEN` writer - Synchronization enable"]
pub type SyncenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCP` reader - Synchronization input polarity"]
pub type SyncpR = crate::FieldReader;
#[doc = "Field `SYNCP` writer - Synchronization input polarity"]
pub type SyncpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NBR` reader - Number of DMA requests to forward"]
pub type NbrR = crate::FieldReader;
#[doc = "Field `NBR` writer - Number of DMA requests to forward"]
pub type NbrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYNCID` reader - Synchronization input identification"]
pub type SyncidR = crate::FieldReader;
#[doc = "Field `SYNCID` writer - Synchronization input identification"]
pub type SyncidW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - Multiplexer input identification"]
    #[inline(always)]
    pub fn muxinid(&self) -> MuxinidR {
        MuxinidR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn soie(&self) -> SoieR {
        SoieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    pub fn evgen(&self) -> EvgenR {
        EvgenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    pub fn syncen(&self) -> SyncenR {
        SyncenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization input polarity"]
    #[inline(always)]
    pub fn syncp(&self) -> SyncpR {
        SyncpR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward"]
    #[inline(always)]
    pub fn nbr(&self) -> NbrR {
        NbrR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization input identification"]
    #[inline(always)]
    pub fn syncid(&self) -> SyncidR {
        SyncidR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Multiplexer input identification"]
    #[inline(always)]
    pub fn muxinid(&mut self) -> MuxinidW<'_, RmCh2cfgSpec> {
        MuxinidW::new(self, 0)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn soie(&mut self) -> SoieW<'_, RmCh2cfgSpec> {
        SoieW::new(self, 8)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    pub fn evgen(&mut self) -> EvgenW<'_, RmCh2cfgSpec> {
        EvgenW::new(self, 9)
    }
    #[doc = "Bit 16 - Synchronization enable"]
    #[inline(always)]
    pub fn syncen(&mut self) -> SyncenW<'_, RmCh2cfgSpec> {
        SyncenW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Synchronization input polarity"]
    #[inline(always)]
    pub fn syncp(&mut self) -> SyncpW<'_, RmCh2cfgSpec> {
        SyncpW::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward"]
    #[inline(always)]
    pub fn nbr(&mut self) -> NbrW<'_, RmCh2cfgSpec> {
        NbrW::new(self, 19)
    }
    #[doc = "Bits 24:28 - Synchronization input identification"]
    #[inline(always)]
    pub fn syncid(&mut self) -> SyncidW<'_, RmCh2cfgSpec> {
        SyncidW::new(self, 24)
    }
}
#[doc = "Request multiplexer channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rm_ch2cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rm_ch2cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmCh2cfgSpec;
impl crate::RegisterSpec for RmCh2cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rm_ch2cfg::R`](R) reader structure"]
impl crate::Readable for RmCh2cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`rm_ch2cfg::W`](W) writer structure"]
impl crate::Writable for RmCh2cfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RM_CH2CFG to value 0"]
impl crate::Resettable for RmCh2cfgSpec {}
