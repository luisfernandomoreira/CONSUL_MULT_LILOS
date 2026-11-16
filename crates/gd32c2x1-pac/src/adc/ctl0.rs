#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `WD0CHSEL` reader - Analog watchdog 0 channel select"]
pub type Wd0chselR = crate::FieldReader;
#[doc = "Field `WD0CHSEL` writer - Analog watchdog 0 channel select"]
pub type Wd0chselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EOCIE` reader - Interrupt enable for EOC"]
pub type EocieR = crate::BitReader;
#[doc = "Field `EOCIE` writer - Interrupt enable for EOC"]
pub type EocieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD0EIE` reader - Interrupt enable for WD0E"]
pub type Wd0eieR = crate::BitReader;
#[doc = "Field `WD0EIE` writer - Interrupt enable for WD0E"]
pub type Wd0eieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOICIE` reader - Interrupt enable for EOIC"]
pub type EoicieR = crate::BitReader;
#[doc = "Field `EOICIE` writer - Interrupt enable for EOIC"]
pub type EoicieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM` reader - Scan mode"]
pub type SmR = crate::BitReader;
#[doc = "Field `SM` writer - Scan mode"]
pub type SmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD0SC` reader - When in scan mode, analog watchdog 0 is effective on a single channel."]
pub type Wd0scR = crate::BitReader;
#[doc = "Field `WD0SC` writer - When in scan mode, analog watchdog 0 is effective on a single channel."]
pub type Wd0scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICA` reader - Inserted sequence convert automatically"]
pub type IcaR = crate::BitReader;
#[doc = "Field `ICA` writer - Inserted sequence convert automatically"]
pub type IcaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISRC` reader - Discontinuous mode on routine sequence"]
pub type DisrcR = crate::BitReader;
#[doc = "Field `DISRC` writer - Discontinuous mode on routine sequence"]
pub type DisrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISIC` reader - Discontinuous mode on inserted sequence"]
pub type DisicR = crate::BitReader;
#[doc = "Field `DISIC` writer - Discontinuous mode on inserted sequence"]
pub type DisicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISNUM` reader - Number of conversions in discontinuous mode"]
pub type DisnumR = crate::FieldReader;
#[doc = "Field `DISNUM` writer - Number of conversions in discontinuous mode"]
pub type DisnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IWD0EN` reader - Inserted channel analog watchdog 0 enable"]
pub type Iwd0enR = crate::BitReader;
#[doc = "Field `IWD0EN` writer - Inserted channel analog watchdog 0 enable"]
pub type Iwd0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWD0EN` reader - Routine channel analog watchdog 0 enable"]
pub type Rwd0enR = crate::BitReader;
#[doc = "Field `RWD0EN` writer - Routine channel analog watchdog 0 enable"]
pub type Rwd0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRES` reader - ADC resolution"]
pub type DresR = crate::FieldReader;
#[doc = "Field `DRES` writer - ADC resolution"]
pub type DresW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WD1EIE` reader - Interrupt enable for WD1E"]
pub type Wd1eieR = crate::BitReader;
#[doc = "Field `WD1EIE` writer - Interrupt enable for WD1E"]
pub type Wd1eieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD2EIE` reader - Interrupt enable for WD2E"]
pub type Wd2eieR = crate::BitReader;
#[doc = "Field `WD2EIE` writer - Interrupt enable for WD2E"]
pub type Wd2eieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Analog watchdog 0 channel select"]
    #[inline(always)]
    pub fn wd0chsel(&self) -> Wd0chselR {
        Wd0chselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&self) -> EocieR {
        EocieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable for WD0E"]
    #[inline(always)]
    pub fn wd0eie(&self) -> Wd0eieR {
        Wd0eieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable for EOIC"]
    #[inline(always)]
    pub fn eoicie(&self) -> EoicieR {
        EoicieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    pub fn sm(&self) -> SmR {
        SmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When in scan mode, analog watchdog 0 is effective on a single channel."]
    #[inline(always)]
    pub fn wd0sc(&self) -> Wd0scR {
        Wd0scR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Inserted sequence convert automatically"]
    #[inline(always)]
    pub fn ica(&self) -> IcaR {
        IcaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Discontinuous mode on routine sequence"]
    #[inline(always)]
    pub fn disrc(&self) -> DisrcR {
        DisrcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Discontinuous mode on inserted sequence"]
    #[inline(always)]
    pub fn disic(&self) -> DisicR {
        DisicR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Number of conversions in discontinuous mode"]
    #[inline(always)]
    pub fn disnum(&self) -> DisnumR {
        DisnumR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 22 - Inserted channel analog watchdog 0 enable"]
    #[inline(always)]
    pub fn iwd0en(&self) -> Iwd0enR {
        Iwd0enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Routine channel analog watchdog 0 enable"]
    #[inline(always)]
    pub fn rwd0en(&self) -> Rwd0enR {
        Rwd0enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - ADC resolution"]
    #[inline(always)]
    pub fn dres(&self) -> DresR {
        DresR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - Interrupt enable for WD1E"]
    #[inline(always)]
    pub fn wd1eie(&self) -> Wd1eieR {
        Wd1eieR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt enable for WD2E"]
    #[inline(always)]
    pub fn wd2eie(&self) -> Wd2eieR {
        Wd2eieR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Analog watchdog 0 channel select"]
    #[inline(always)]
    pub fn wd0chsel(&mut self) -> Wd0chselW<'_, Ctl0Spec> {
        Wd0chselW::new(self, 0)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&mut self) -> EocieW<'_, Ctl0Spec> {
        EocieW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt enable for WD0E"]
    #[inline(always)]
    pub fn wd0eie(&mut self) -> Wd0eieW<'_, Ctl0Spec> {
        Wd0eieW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt enable for EOIC"]
    #[inline(always)]
    pub fn eoicie(&mut self) -> EoicieW<'_, Ctl0Spec> {
        EoicieW::new(self, 7)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    pub fn sm(&mut self) -> SmW<'_, Ctl0Spec> {
        SmW::new(self, 8)
    }
    #[doc = "Bit 9 - When in scan mode, analog watchdog 0 is effective on a single channel."]
    #[inline(always)]
    pub fn wd0sc(&mut self) -> Wd0scW<'_, Ctl0Spec> {
        Wd0scW::new(self, 9)
    }
    #[doc = "Bit 10 - Inserted sequence convert automatically"]
    #[inline(always)]
    pub fn ica(&mut self) -> IcaW<'_, Ctl0Spec> {
        IcaW::new(self, 10)
    }
    #[doc = "Bit 11 - Discontinuous mode on routine sequence"]
    #[inline(always)]
    pub fn disrc(&mut self) -> DisrcW<'_, Ctl0Spec> {
        DisrcW::new(self, 11)
    }
    #[doc = "Bit 12 - Discontinuous mode on inserted sequence"]
    #[inline(always)]
    pub fn disic(&mut self) -> DisicW<'_, Ctl0Spec> {
        DisicW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Number of conversions in discontinuous mode"]
    #[inline(always)]
    pub fn disnum(&mut self) -> DisnumW<'_, Ctl0Spec> {
        DisnumW::new(self, 13)
    }
    #[doc = "Bit 22 - Inserted channel analog watchdog 0 enable"]
    #[inline(always)]
    pub fn iwd0en(&mut self) -> Iwd0enW<'_, Ctl0Spec> {
        Iwd0enW::new(self, 22)
    }
    #[doc = "Bit 23 - Routine channel analog watchdog 0 enable"]
    #[inline(always)]
    pub fn rwd0en(&mut self) -> Rwd0enW<'_, Ctl0Spec> {
        Rwd0enW::new(self, 23)
    }
    #[doc = "Bits 24:25 - ADC resolution"]
    #[inline(always)]
    pub fn dres(&mut self) -> DresW<'_, Ctl0Spec> {
        DresW::new(self, 24)
    }
    #[doc = "Bit 30 - Interrupt enable for WD1E"]
    #[inline(always)]
    pub fn wd1eie(&mut self) -> Wd1eieW<'_, Ctl0Spec> {
        Wd1eieW::new(self, 30)
    }
    #[doc = "Bit 31 - Interrupt enable for WD2E"]
    #[inline(always)]
    pub fn wd2eie(&mut self) -> Wd2eieW<'_, Ctl0Spec> {
        Wd2eieW::new(self, 31)
    }
}
#[doc = "Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {}
