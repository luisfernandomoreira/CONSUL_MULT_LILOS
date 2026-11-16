#[doc = "Register `RG_CH0CFG` reader"]
pub type R = crate::R<RgCh0cfgSpec>;
#[doc = "Register `RG_CH0CFG` writer"]
pub type W = crate::W<RgCh0cfgSpec>;
#[doc = "Field `TID` reader - Trigger input identification"]
pub type TidR = crate::FieldReader;
#[doc = "Field `TID` writer - Trigger input identification"]
pub type TidW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TOIE` reader - Trigger overrun interrupt enable"]
pub type ToieR = crate::BitReader;
#[doc = "Field `TOIE` writer - Trigger overrun interrupt enable"]
pub type ToieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGEN` reader - DMAMUX request generator channel x enable"]
pub type RgenR = crate::BitReader;
#[doc = "Field `RGEN` writer - DMAMUX request generator channel x enable"]
pub type RgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGTP` reader - DMA request generator trigger polarity"]
pub type RgtpR = crate::FieldReader;
#[doc = "Field `RGTP` writer - DMA request generator trigger polarity"]
pub type RgtpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NBRG` reader - Number of DMA requests to be generated"]
pub type NbrgR = crate::FieldReader;
#[doc = "Field `NBRG` writer - Number of DMA requests to be generated"]
pub type NbrgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Trigger input identification"]
    #[inline(always)]
    pub fn tid(&self) -> TidR {
        TidR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn toie(&self) -> ToieR {
        ToieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMAMUX request generator channel x enable"]
    #[inline(always)]
    pub fn rgen(&self) -> RgenR {
        RgenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    pub fn rgtp(&self) -> RgtpR {
        RgtpR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated"]
    #[inline(always)]
    pub fn nbrg(&self) -> NbrgR {
        NbrgR::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trigger input identification"]
    #[inline(always)]
    pub fn tid(&mut self) -> TidW<'_, RgCh0cfgSpec> {
        TidW::new(self, 0)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn toie(&mut self) -> ToieW<'_, RgCh0cfgSpec> {
        ToieW::new(self, 8)
    }
    #[doc = "Bit 16 - DMAMUX request generator channel x enable"]
    #[inline(always)]
    pub fn rgen(&mut self) -> RgenW<'_, RgCh0cfgSpec> {
        RgenW::new(self, 16)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    pub fn rgtp(&mut self) -> RgtpW<'_, RgCh0cfgSpec> {
        RgtpW::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated"]
    #[inline(always)]
    pub fn nbrg(&mut self) -> NbrgW<'_, RgCh0cfgSpec> {
        NbrgW::new(self, 19)
    }
}
#[doc = "Request generator channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_ch0cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_ch0cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgCh0cfgSpec;
impl crate::RegisterSpec for RgCh0cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rg_ch0cfg::R`](R) reader structure"]
impl crate::Readable for RgCh0cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`rg_ch0cfg::W`](W) writer structure"]
impl crate::Writable for RgCh0cfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RG_CH0CFG to value 0"]
impl crate::Resettable for RgCh0cfgSpec {}
