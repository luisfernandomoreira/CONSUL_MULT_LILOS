#[doc = "Register `CTL2` reader"]
pub type R = crate::R<Ctl2Spec>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<Ctl2Spec>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRLP` reader - IrDA low-power"]
pub type IrlpR = crate::BitReader;
#[doc = "Field `IRLP` writer - IrDA low-power"]
pub type IrlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDEN` reader - Half-duplex enable"]
pub type HdenR = crate::BitReader;
#[doc = "Field `HDEN` writer - Half-duplex enable"]
pub type HdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DENR` reader - DMA enable for reception"]
pub type DenrR = crate::BitReader;
#[doc = "Field `DENR` writer - DMA enable for reception"]
pub type DenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DENT` reader - DMA enable for transmission"]
pub type DentR = crate::BitReader;
#[doc = "Field `DENT` writer - DMA enable for transmission"]
pub type DentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` reader - RTS enable"]
pub type RtsenR = crate::BitReader;
#[doc = "Field `RTSEN` writer - RTS enable"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEN` reader - CTS enable"]
pub type CtsenR = crate::BitReader;
#[doc = "Field `CTSEN` writer - CTS enable"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CtsieR = crate::BitReader;
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CtsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSB` reader - One sample bit method"]
pub type OsbR = crate::BitReader;
#[doc = "Field `OSB` writer - One sample bit method"]
pub type OsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRD` reader - Overrun disable"]
pub type OvrdR = crate::BitReader;
#[doc = "Field `OVRD` writer - Overrun disable"]
pub type OvrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRE` reader - Disable DMA on reception error"]
pub type DdreR = crate::BitReader;
#[doc = "Field `DDRE` writer - Disable DMA on reception error"]
pub type DdreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEM` reader - Driver enable mode"]
pub type DemR = crate::BitReader;
#[doc = "Field `DEM` writer - Driver enable mode"]
pub type DemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEP` reader - Driver enable polarity mode"]
pub type DepR = crate::BitReader;
#[doc = "Field `DEP` writer - Driver enable polarity mode"]
pub type DepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&self) -> IrlpR {
        IrlpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex enable"]
    #[inline(always)]
    pub fn hden(&self) -> HdenR {
        HdenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable for reception"]
    #[inline(always)]
    pub fn denr(&self) -> DenrR {
        DenrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable for transmission"]
    #[inline(always)]
    pub fn dent(&self) -> DentR {
        DentR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&self) -> RtsenR {
        RtsenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CtsieR {
        CtsieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - One sample bit method"]
    #[inline(always)]
    pub fn osb(&self) -> OsbR {
        OsbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun disable"]
    #[inline(always)]
    pub fn ovrd(&self) -> OvrdR {
        OvrdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable DMA on reception error"]
    #[inline(always)]
    pub fn ddre(&self) -> DdreR {
        DdreR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&self) -> DemR {
        DemR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Driver enable polarity mode"]
    #[inline(always)]
    pub fn dep(&self) -> DepR {
        DepR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, Ctl2Spec> {
        ErrieW::new(self, 0)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&mut self) -> IrlpW<'_, Ctl2Spec> {
        IrlpW::new(self, 2)
    }
    #[doc = "Bit 3 - Half-duplex enable"]
    #[inline(always)]
    pub fn hden(&mut self) -> HdenW<'_, Ctl2Spec> {
        HdenW::new(self, 3)
    }
    #[doc = "Bit 6 - DMA enable for reception"]
    #[inline(always)]
    pub fn denr(&mut self) -> DenrW<'_, Ctl2Spec> {
        DenrW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA enable for transmission"]
    #[inline(always)]
    pub fn dent(&mut self) -> DentW<'_, Ctl2Spec> {
        DentW::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RtsenW<'_, Ctl2Spec> {
        RtsenW::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CtsenW<'_, Ctl2Spec> {
        CtsenW::new(self, 9)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CtsieW<'_, Ctl2Spec> {
        CtsieW::new(self, 10)
    }
    #[doc = "Bit 11 - One sample bit method"]
    #[inline(always)]
    pub fn osb(&mut self) -> OsbW<'_, Ctl2Spec> {
        OsbW::new(self, 11)
    }
    #[doc = "Bit 12 - Overrun disable"]
    #[inline(always)]
    pub fn ovrd(&mut self) -> OvrdW<'_, Ctl2Spec> {
        OvrdW::new(self, 12)
    }
    #[doc = "Bit 13 - Disable DMA on reception error"]
    #[inline(always)]
    pub fn ddre(&mut self) -> DdreW<'_, Ctl2Spec> {
        DdreW::new(self, 13)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&mut self) -> DemW<'_, Ctl2Spec> {
        DemW::new(self, 14)
    }
    #[doc = "Bit 15 - Driver enable polarity mode"]
    #[inline(always)]
    pub fn dep(&mut self) -> DepW<'_, Ctl2Spec> {
        DepW::new(self, 15)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl2Spec;
impl crate::RegisterSpec for Ctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2::R`](R) reader structure"]
impl crate::Readable for Ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"]
impl crate::Writable for Ctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL2 to value 0"]
impl crate::Resettable for Ctl2Spec {}
