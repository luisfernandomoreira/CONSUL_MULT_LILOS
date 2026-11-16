#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `PG` reader - Main flash page program command bit"]
pub type PgR = crate::BitReader;
#[doc = "Field `PG` writer - Main flash page program command bit"]
pub type PgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER` reader - Main flash page erase command bit"]
pub type PerR = crate::BitReader;
#[doc = "Field `PER` writer - Main flash page erase command bit"]
pub type PerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER` reader - Main flash mass erase command bit"]
pub type MerR = crate::BitReader;
#[doc = "Field `MER` writer - Main flash mass erase command bit"]
pub type MerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PN` reader - Page number selection"]
pub type PnR = crate::FieldReader;
#[doc = "Field `PN` writer - Page number selection"]
pub type PnW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `START` reader - Send erase command to FMC bit"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Send erase command to FMC bit"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBSTART` reader - Send option byte change command to FMC bit."]
pub type ObstartR = crate::BitReader;
#[doc = "Field `OBSTART` writer - Send option byte change command to FMC bit."]
pub type ObstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTPG` reader - Main flash fast program command bit"]
pub type FstpgR = crate::BitReader;
#[doc = "Field `FSTPG` writer - Main flash fast program command bit"]
pub type FstpgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDIE` reader - End of operation interrupt enable bit"]
pub type EndieR = crate::BitReader;
#[doc = "Field `ENDIE` writer - End of operation interrupt enable bit"]
pub type EndieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - OPRERR error interrupt enable bit"]
pub type ErrieR = crate::BitReader;
#[doc = "Field `ERRIE` writer - OPRERR error interrupt enable bit"]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPERRIE` reader - Read protection error interrupt enable bit"]
pub type RperrieR = crate::BitReader;
#[doc = "Field `RPERRIE` writer - Read protection error interrupt enable bit"]
pub type RperrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBRLD` reader - Option byte reload bit"]
pub type ObrldR = crate::BitReader;
#[doc = "Field `OBRLD` writer - Option byte reload bit"]
pub type ObrldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCR` reader - Secure-access area enable bit."]
pub type ScrR = crate::BitReader;
#[doc = "Field `SCR` writer - Secure-access area enable bit."]
pub type ScrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBLK` reader - FMC_OBCTL lock bit"]
pub type OblkR = crate::BitReader;
#[doc = "Field `OBLK` writer - FMC_OBCTL lock bit"]
pub type OblkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LK` reader - FMC_CTL lock bit"]
pub type LkR = crate::BitReader;
#[doc = "Field `LK` writer - FMC_CTL lock bit"]
pub type LkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Main flash page program command bit"]
    #[inline(always)]
    pub fn pg(&self) -> PgR {
        PgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Main flash page erase command bit"]
    #[inline(always)]
    pub fn per(&self) -> PerR {
        PerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Main flash mass erase command bit"]
    #[inline(always)]
    pub fn mer(&self) -> MerR {
        MerR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - Page number selection"]
    #[inline(always)]
    pub fn pn(&self) -> PnR {
        PnR::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Send erase command to FMC bit"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Send option byte change command to FMC bit."]
    #[inline(always)]
    pub fn obstart(&self) -> ObstartR {
        ObstartR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Main flash fast program command bit"]
    #[inline(always)]
    pub fn fstpg(&self) -> FstpgR {
        FstpgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - End of operation interrupt enable bit"]
    #[inline(always)]
    pub fn endie(&self) -> EndieR {
        EndieR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - OPRERR error interrupt enable bit"]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Read protection error interrupt enable bit"]
    #[inline(always)]
    pub fn rperrie(&self) -> RperrieR {
        RperrieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Option byte reload bit"]
    #[inline(always)]
    pub fn obrld(&self) -> ObrldR {
        ObrldR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Secure-access area enable bit."]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - FMC_OBCTL lock bit"]
    #[inline(always)]
    pub fn oblk(&self) -> OblkR {
        OblkR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FMC_CTL lock bit"]
    #[inline(always)]
    pub fn lk(&self) -> LkR {
        LkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main flash page program command bit"]
    #[inline(always)]
    pub fn pg(&mut self) -> PgW<'_, CtlSpec> {
        PgW::new(self, 0)
    }
    #[doc = "Bit 1 - Main flash page erase command bit"]
    #[inline(always)]
    pub fn per(&mut self) -> PerW<'_, CtlSpec> {
        PerW::new(self, 1)
    }
    #[doc = "Bit 2 - Main flash mass erase command bit"]
    #[inline(always)]
    pub fn mer(&mut self) -> MerW<'_, CtlSpec> {
        MerW::new(self, 2)
    }
    #[doc = "Bits 3:8 - Page number selection"]
    #[inline(always)]
    pub fn pn(&mut self) -> PnW<'_, CtlSpec> {
        PnW::new(self, 3)
    }
    #[doc = "Bit 16 - Send erase command to FMC bit"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CtlSpec> {
        StartW::new(self, 16)
    }
    #[doc = "Bit 17 - Send option byte change command to FMC bit."]
    #[inline(always)]
    pub fn obstart(&mut self) -> ObstartW<'_, CtlSpec> {
        ObstartW::new(self, 17)
    }
    #[doc = "Bit 18 - Main flash fast program command bit"]
    #[inline(always)]
    pub fn fstpg(&mut self) -> FstpgW<'_, CtlSpec> {
        FstpgW::new(self, 18)
    }
    #[doc = "Bit 24 - End of operation interrupt enable bit"]
    #[inline(always)]
    pub fn endie(&mut self) -> EndieW<'_, CtlSpec> {
        EndieW::new(self, 24)
    }
    #[doc = "Bit 25 - OPRERR error interrupt enable bit"]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<'_, CtlSpec> {
        ErrieW::new(self, 25)
    }
    #[doc = "Bit 26 - Read protection error interrupt enable bit"]
    #[inline(always)]
    pub fn rperrie(&mut self) -> RperrieW<'_, CtlSpec> {
        RperrieW::new(self, 26)
    }
    #[doc = "Bit 27 - Option byte reload bit"]
    #[inline(always)]
    pub fn obrld(&mut self) -> ObrldW<'_, CtlSpec> {
        ObrldW::new(self, 27)
    }
    #[doc = "Bit 28 - Secure-access area enable bit."]
    #[inline(always)]
    pub fn scr(&mut self) -> ScrW<'_, CtlSpec> {
        ScrW::new(self, 28)
    }
    #[doc = "Bit 30 - FMC_OBCTL lock bit"]
    #[inline(always)]
    pub fn oblk(&mut self) -> OblkW<'_, CtlSpec> {
        OblkW::new(self, 30)
    }
    #[doc = "Bit 31 - FMC_CTL lock bit"]
    #[inline(always)]
    pub fn lk(&mut self) -> LkW<'_, CtlSpec> {
        LkW::new(self, 31)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL to value 0xc000_0000"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0xc000_0000;
}
