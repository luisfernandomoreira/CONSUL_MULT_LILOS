#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Field `IRC48MEN` reader - Internal high speed oscillator enable"]
pub type Irc48menR = crate::BitReader;
#[doc = "Field `IRC48MEN` writer - Internal high speed oscillator enable"]
pub type Irc48menW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC48MSTB` reader - IRC48M high speed internal oscillator stabilization flag"]
pub type Irc48mstbR = crate::BitReader;
#[doc = "Field `IRC48MADJ` reader - Internal 48M RC oscillator clock trim adjust value"]
pub type Irc48madjR = crate::FieldReader;
#[doc = "Field `IRC48MADJ` writer - Internal 48M RC oscillator clock trim adjust value"]
pub type Irc48madjW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IRC48MCALIB` reader - Internal 48M RC oscillator calibration value register"]
pub type Irc48mcalibR = crate::FieldReader;
#[doc = "Field `HXTALEN` reader - External high speed oscillator enable"]
pub type HxtalenR = crate::BitReader;
#[doc = "Field `HXTALEN` writer - External high speed oscillator enable"]
pub type HxtalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HXTALSTB` reader - External crystal oscillator (HXTAL) clock stabilization flag"]
pub type HxtalstbR = crate::BitReader;
#[doc = "Field `HXTALBPS` reader - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HxtalbpsR = crate::BitReader;
#[doc = "Field `HXTALBPS` writer - External crystal oscillator (HXTAL) clock bypass mode enable"]
pub type HxtalbpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKMEN` reader - HXTAL clock monitor enable"]
pub type CkmenR = crate::BitReader;
#[doc = "Field `CKMEN` writer - HXTAL clock monitor enable"]
pub type CkmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC48MDIV_PER` reader - IRC48M clock divider factor for peripheral clock"]
pub type Irc48mdivPerR = crate::FieldReader;
#[doc = "Field `IRC48MDIV_PER` writer - IRC48M clock divider factor for peripheral clock"]
pub type Irc48mdivPerW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IRC48M_PEREN` reader - IRC48M always-enable for peripheral."]
pub type Irc48mPerenR = crate::BitReader;
#[doc = "Field `IRC48M_PEREN` writer - IRC48M always-enable for peripheral."]
pub type Irc48mPerenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC48MDIV_SYS` reader - IRC48M clock divider factor for system clock"]
pub type Irc48mdivSysR = crate::FieldReader;
#[doc = "Field `IRC48MDIV_SYS` writer - IRC48M clock divider factor for system clock"]
pub type Irc48mdivSysW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Internal high speed oscillator enable"]
    #[inline(always)]
    pub fn irc48men(&self) -> Irc48menR {
        Irc48menR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRC48M high speed internal oscillator stabilization flag"]
    #[inline(always)]
    pub fn irc48mstb(&self) -> Irc48mstbR {
        Irc48mstbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Internal 48M RC oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc48madj(&self) -> Irc48madjR {
        Irc48madjR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - Internal 48M RC oscillator calibration value register"]
    #[inline(always)]
    pub fn irc48mcalib(&self) -> Irc48mcalibR {
        Irc48mcalibR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - External high speed oscillator enable"]
    #[inline(always)]
    pub fn hxtalen(&self) -> HxtalenR {
        HxtalenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External crystal oscillator (HXTAL) clock stabilization flag"]
    #[inline(always)]
    pub fn hxtalstb(&self) -> HxtalstbR {
        HxtalstbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    pub fn hxtalbps(&self) -> HxtalbpsR {
        HxtalbpsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HXTAL clock monitor enable"]
    #[inline(always)]
    pub fn ckmen(&self) -> CkmenR {
        CkmenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 25:27 - IRC48M clock divider factor for peripheral clock"]
    #[inline(always)]
    pub fn irc48mdiv_per(&self) -> Irc48mdivPerR {
        Irc48mdivPerR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - IRC48M always-enable for peripheral."]
    #[inline(always)]
    pub fn irc48m_peren(&self) -> Irc48mPerenR {
        Irc48mPerenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - IRC48M clock divider factor for system clock"]
    #[inline(always)]
    pub fn irc48mdiv_sys(&self) -> Irc48mdivSysR {
        Irc48mdivSysR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal high speed oscillator enable"]
    #[inline(always)]
    pub fn irc48men(&mut self) -> Irc48menW<'_, Ctl0Spec> {
        Irc48menW::new(self, 0)
    }
    #[doc = "Bits 2:7 - Internal 48M RC oscillator clock trim adjust value"]
    #[inline(always)]
    pub fn irc48madj(&mut self) -> Irc48madjW<'_, Ctl0Spec> {
        Irc48madjW::new(self, 2)
    }
    #[doc = "Bit 16 - External high speed oscillator enable"]
    #[inline(always)]
    pub fn hxtalen(&mut self) -> HxtalenW<'_, Ctl0Spec> {
        HxtalenW::new(self, 16)
    }
    #[doc = "Bit 18 - External crystal oscillator (HXTAL) clock bypass mode enable"]
    #[inline(always)]
    pub fn hxtalbps(&mut self) -> HxtalbpsW<'_, Ctl0Spec> {
        HxtalbpsW::new(self, 18)
    }
    #[doc = "Bit 19 - HXTAL clock monitor enable"]
    #[inline(always)]
    pub fn ckmen(&mut self) -> CkmenW<'_, Ctl0Spec> {
        CkmenW::new(self, 19)
    }
    #[doc = "Bits 25:27 - IRC48M clock divider factor for peripheral clock"]
    #[inline(always)]
    pub fn irc48mdiv_per(&mut self) -> Irc48mdivPerW<'_, Ctl0Spec> {
        Irc48mdivPerW::new(self, 25)
    }
    #[doc = "Bit 28 - IRC48M always-enable for peripheral."]
    #[inline(always)]
    pub fn irc48m_peren(&mut self) -> Irc48mPerenW<'_, Ctl0Spec> {
        Irc48mPerenW::new(self, 28)
    }
    #[doc = "Bits 29:31 - IRC48M clock divider factor for system clock"]
    #[inline(always)]
    pub fn irc48mdiv_sys(&mut self) -> Irc48mdivSysW<'_, Ctl0Spec> {
        Irc48mdivSysW::new(self, 29)
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
#[doc = "`reset()` method sets CTL0 to value 0x4400_0043"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0x4400_0043;
}
