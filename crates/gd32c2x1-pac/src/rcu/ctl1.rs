#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "Field `LXTALEN` reader - LXTAL enable"]
pub type LxtalenR = crate::BitReader;
#[doc = "Field `LXTALEN` writer - LXTAL enable"]
pub type LxtalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LXTALSTB` reader - External low-speed oscillator stabilization"]
pub type LxtalstbR = crate::BitReader;
#[doc = "Field `LXTALBPS` reader - LXTAL bypass mode enable"]
pub type LxtalbpsR = crate::BitReader;
#[doc = "Field `LXTALBPS` writer - LXTAL bypass mode enable"]
pub type LxtalbpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LXTALDRI` reader - LXTAL drive capability"]
pub type LxtaldriR = crate::BitReader;
#[doc = "Field `LXTALDRI` writer - LXTAL drive capability"]
pub type LxtaldriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCKMEN` reader - LXTAL clock monitor enable"]
pub type LckmenR = crate::BitReader;
#[doc = "Field `LCKMEN` writer - LXTAL clock monitor enable"]
pub type LckmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCKMD` reader - LXTAL clock failure detection"]
pub type LckmdR = crate::BitReader;
#[doc = "Field `LCKMD` writer - LXTAL clock failure detection"]
pub type LckmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LXTALSTBRST` reader - LXTAL stabilization reset."]
pub type LxtalstbrstR = crate::BitReader;
#[doc = "Field `LXTALSTBRST` writer - LXTAL stabilization reset."]
pub type LxtalstbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSRC` reader - RTC clock entry selection"]
pub type RtcsrcR = crate::FieldReader;
#[doc = "Field `RTCSRC` writer - RTC clock entry selection"]
pub type RtcsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RtcenR = crate::BitReader;
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPRST` reader - Backup register reset"]
pub type BkprstR = crate::BitReader;
#[doc = "Field `BKPRST` writer - Backup register reset"]
pub type BkprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSCKOUTEN` reader - Low speed clock output enable"]
pub type LsckoutenR = crate::BitReader;
#[doc = "Field `LSCKOUTEN` writer - Low speed clock output enable"]
pub type LsckoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSCKOUTSEL` reader - Low speed clock output selection"]
pub type LsckoutselR = crate::BitReader;
#[doc = "Field `LSCKOUTSEL` writer - Low speed clock output selection"]
pub type LsckoutselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LXTAL enable"]
    #[inline(always)]
    pub fn lxtalen(&self) -> LxtalenR {
        LxtalenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External low-speed oscillator stabilization"]
    #[inline(always)]
    pub fn lxtalstb(&self) -> LxtalstbR {
        LxtalstbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LXTAL bypass mode enable"]
    #[inline(always)]
    pub fn lxtalbps(&self) -> LxtalbpsR {
        LxtalbpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LXTAL drive capability"]
    #[inline(always)]
    pub fn lxtaldri(&self) -> LxtaldriR {
        LxtaldriR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - LXTAL clock monitor enable"]
    #[inline(always)]
    pub fn lckmen(&self) -> LckmenR {
        LckmenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LXTAL clock failure detection"]
    #[inline(always)]
    pub fn lckmd(&self) -> LckmdR {
        LckmdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LXTAL stabilization reset."]
    #[inline(always)]
    pub fn lxtalstbrst(&self) -> LxtalstbrstR {
        LxtalstbrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock entry selection"]
    #[inline(always)]
    pub fn rtcsrc(&self) -> RtcsrcR {
        RtcsrcR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup register reset"]
    #[inline(always)]
    pub fn bkprst(&self) -> BkprstR {
        BkprstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Low speed clock output enable"]
    #[inline(always)]
    pub fn lsckouten(&self) -> LsckoutenR {
        LsckoutenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low speed clock output selection"]
    #[inline(always)]
    pub fn lsckoutsel(&self) -> LsckoutselR {
        LsckoutselR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LXTAL enable"]
    #[inline(always)]
    pub fn lxtalen(&mut self) -> LxtalenW<'_, Ctl1Spec> {
        LxtalenW::new(self, 0)
    }
    #[doc = "Bit 2 - LXTAL bypass mode enable"]
    #[inline(always)]
    pub fn lxtalbps(&mut self) -> LxtalbpsW<'_, Ctl1Spec> {
        LxtalbpsW::new(self, 2)
    }
    #[doc = "Bit 3 - LXTAL drive capability"]
    #[inline(always)]
    pub fn lxtaldri(&mut self) -> LxtaldriW<'_, Ctl1Spec> {
        LxtaldriW::new(self, 3)
    }
    #[doc = "Bit 5 - LXTAL clock monitor enable"]
    #[inline(always)]
    pub fn lckmen(&mut self) -> LckmenW<'_, Ctl1Spec> {
        LckmenW::new(self, 5)
    }
    #[doc = "Bit 6 - LXTAL clock failure detection"]
    #[inline(always)]
    pub fn lckmd(&mut self) -> LckmdW<'_, Ctl1Spec> {
        LckmdW::new(self, 6)
    }
    #[doc = "Bit 7 - LXTAL stabilization reset."]
    #[inline(always)]
    pub fn lxtalstbrst(&mut self) -> LxtalstbrstW<'_, Ctl1Spec> {
        LxtalstbrstW::new(self, 7)
    }
    #[doc = "Bits 8:9 - RTC clock entry selection"]
    #[inline(always)]
    pub fn rtcsrc(&mut self) -> RtcsrcW<'_, Ctl1Spec> {
        RtcsrcW::new(self, 8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RtcenW<'_, Ctl1Spec> {
        RtcenW::new(self, 15)
    }
    #[doc = "Bit 16 - Backup register reset"]
    #[inline(always)]
    pub fn bkprst(&mut self) -> BkprstW<'_, Ctl1Spec> {
        BkprstW::new(self, 16)
    }
    #[doc = "Bit 24 - Low speed clock output enable"]
    #[inline(always)]
    pub fn lsckouten(&mut self) -> LsckoutenW<'_, Ctl1Spec> {
        LsckoutenW::new(self, 24)
    }
    #[doc = "Bit 25 - Low speed clock output selection"]
    #[inline(always)]
    pub fn lsckoutsel(&mut self) -> LsckoutselW<'_, Ctl1Spec> {
        LsckoutselW::new(self, 25)
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
#[doc = "`reset()` method sets CTL1 to value 0x08"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0x08;
}
