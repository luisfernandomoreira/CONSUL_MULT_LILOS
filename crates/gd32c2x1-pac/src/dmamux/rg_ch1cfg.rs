#[doc = "Register `RG_CH1CFG` reader"]
pub type R = crate::R<RgCh1cfgSpec>;
#[doc = "Register `RG_CH1CFG` writer"]
pub type W = crate::W<RgCh1cfgSpec>;
#[doc = "Field `TID` reader - Trigger input identification"]
pub type TidR = crate::FieldReader;
#[doc = "Field `TID` writer - Trigger input identification"]
pub type TidW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TOIE` reader - Trigger overrun interrupt enable"]
pub type ToieR = crate::BitReader;
#[doc = "Field `TOIE` writer - Trigger overrun interrupt enable"]
pub type ToieW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bits 16:17 - DMA request generator trigger polarity"]
    #[inline(always)]
    pub fn rgtp(&self) -> RgtpR {
        RgtpR::new(((self.bits >> 16) & 3) as u8)
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
    pub fn tid(&mut self) -> TidW<'_, RgCh1cfgSpec> {
        TidW::new(self, 0)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn toie(&mut self) -> ToieW<'_, RgCh1cfgSpec> {
        ToieW::new(self, 8)
    }
    #[doc = "Bits 16:17 - DMA request generator trigger polarity"]
    #[inline(always)]
    pub fn rgtp(&mut self) -> RgtpW<'_, RgCh1cfgSpec> {
        RgtpW::new(self, 16)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated"]
    #[inline(always)]
    pub fn nbrg(&mut self) -> NbrgW<'_, RgCh1cfgSpec> {
        NbrgW::new(self, 19)
    }
}
#[doc = "Request generator channel 1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_ch1cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_ch1cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgCh1cfgSpec;
impl crate::RegisterSpec for RgCh1cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rg_ch1cfg::R`](R) reader structure"]
impl crate::Readable for RgCh1cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`rg_ch1cfg::W`](W) writer structure"]
impl crate::Writable for RgCh1cfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RG_CH1CFG to value 0"]
impl crate::Resettable for RgCh1cfgSpec {}
