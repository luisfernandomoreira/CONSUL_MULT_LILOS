#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `ADCON` reader - ADC ON"]
pub type AdconR = crate::BitReader;
#[doc = "Field `ADCON` writer - ADC ON"]
pub type AdconW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTN` reader - Continuous mode"]
pub type CtnR = crate::BitReader;
#[doc = "Field `CTN` writer - Continuous mode"]
pub type CtnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - DMA request enable"]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - DMA request enable"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAL` reader - Data alignment"]
pub type DalR = crate::BitReader;
#[doc = "Field `DAL` writer - Data alignment"]
pub type DalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETSIC` reader - External trigger selection for inserted sequence"]
pub type EtsicR = crate::FieldReader;
#[doc = "Field `ETSIC` writer - External trigger selection for inserted sequence"]
pub type EtsicW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETEIC` reader - External trigger enable for inserted sequence"]
pub type EteicR = crate::BitReader;
#[doc = "Field `ETEIC` writer - External trigger enable for inserted sequence"]
pub type EteicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETSRC` reader - External trigger selection for routine sequence"]
pub type EtsrcR = crate::FieldReader;
#[doc = "Field `ETSRC` writer - External trigger selection for routine sequence"]
pub type EtsrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ETERC` reader - External trigger enable for routine sequence"]
pub type EtercR = crate::BitReader;
#[doc = "Field `ETERC` writer - External trigger enable for routine sequence"]
pub type EtercW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWICST` reader - Software start on inserted sequence."]
pub type SwicstR = crate::BitReader;
#[doc = "Field `SWICST` writer - Software start on inserted sequence."]
pub type SwicstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRCST` reader - Software start of routine sequence"]
pub type SwrcstR = crate::BitReader;
#[doc = "Field `SWRCST` writer - Software start of routine sequence"]
pub type SwrcstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSVEN` reader - Temperature sensor output voltage channel (VSENSE) enable"]
pub type TsvenR = crate::BitReader;
#[doc = "Field `TSVEN` writer - Temperature sensor output voltage channel (VSENSE) enable"]
pub type TsvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INREFEN` reader - Internal reference voltage channel (VREFINT) enable"]
pub type InrefenR = crate::BitReader;
#[doc = "Field `INREFEN` writer - Internal reference voltage channel (VREFINT) enable"]
pub type InrefenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC ON"]
    #[inline(always)]
    pub fn adcon(&self) -> AdconR {
        AdconR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    pub fn ctn(&self) -> CtnR {
        CtnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dal(&self) -> DalR {
        DalR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - External trigger selection for inserted sequence"]
    #[inline(always)]
    pub fn etsic(&self) -> EtsicR {
        EtsicR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - External trigger enable for inserted sequence"]
    #[inline(always)]
    pub fn eteic(&self) -> EteicR {
        EteicR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - External trigger selection for routine sequence"]
    #[inline(always)]
    pub fn etsrc(&self) -> EtsrcR {
        EtsrcR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - External trigger enable for routine sequence"]
    #[inline(always)]
    pub fn eterc(&self) -> EtercR {
        EtercR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Software start on inserted sequence."]
    #[inline(always)]
    pub fn swicst(&self) -> SwicstR {
        SwicstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Software start of routine sequence"]
    #[inline(always)]
    pub fn swrcst(&self) -> SwrcstR {
        SwrcstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor output voltage channel (VSENSE) enable"]
    #[inline(always)]
    pub fn tsven(&self) -> TsvenR {
        TsvenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Internal reference voltage channel (VREFINT) enable"]
    #[inline(always)]
    pub fn inrefen(&self) -> InrefenR {
        InrefenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ON"]
    #[inline(always)]
    pub fn adcon(&mut self) -> AdconW<'_, Ctl1Spec> {
        AdconW::new(self, 0)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    pub fn ctn(&mut self) -> CtnW<'_, Ctl1Spec> {
        CtnW::new(self, 1)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, Ctl1Spec> {
        DmaW::new(self, 8)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dal(&mut self) -> DalW<'_, Ctl1Spec> {
        DalW::new(self, 11)
    }
    #[doc = "Bits 12:14 - External trigger selection for inserted sequence"]
    #[inline(always)]
    pub fn etsic(&mut self) -> EtsicW<'_, Ctl1Spec> {
        EtsicW::new(self, 12)
    }
    #[doc = "Bit 15 - External trigger enable for inserted sequence"]
    #[inline(always)]
    pub fn eteic(&mut self) -> EteicW<'_, Ctl1Spec> {
        EteicW::new(self, 15)
    }
    #[doc = "Bits 17:19 - External trigger selection for routine sequence"]
    #[inline(always)]
    pub fn etsrc(&mut self) -> EtsrcW<'_, Ctl1Spec> {
        EtsrcW::new(self, 17)
    }
    #[doc = "Bit 20 - External trigger enable for routine sequence"]
    #[inline(always)]
    pub fn eterc(&mut self) -> EtercW<'_, Ctl1Spec> {
        EtercW::new(self, 20)
    }
    #[doc = "Bit 21 - Software start on inserted sequence."]
    #[inline(always)]
    pub fn swicst(&mut self) -> SwicstW<'_, Ctl1Spec> {
        SwicstW::new(self, 21)
    }
    #[doc = "Bit 22 - Software start of routine sequence"]
    #[inline(always)]
    pub fn swrcst(&mut self) -> SwrcstW<'_, Ctl1Spec> {
        SwrcstW::new(self, 22)
    }
    #[doc = "Bit 23 - Temperature sensor output voltage channel (VSENSE) enable"]
    #[inline(always)]
    pub fn tsven(&mut self) -> TsvenW<'_, Ctl1Spec> {
        TsvenW::new(self, 23)
    }
    #[doc = "Bit 24 - Internal reference voltage channel (VREFINT) enable"]
    #[inline(always)]
    pub fn inrefen(&mut self) -> InrefenW<'_, Ctl1Spec> {
        InrefenW::new(self, 24)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {}
