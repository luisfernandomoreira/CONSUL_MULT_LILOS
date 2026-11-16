#[doc = "Register `OBCTL` reader"]
pub type R = crate::R<ObctlSpec>;
#[doc = "Register `OBCTL` writer"]
pub type W = crate::W<ObctlSpec>;
#[doc = "Field `SPC` reader - Security protection level option byte status bits"]
pub type SpcR = crate::FieldReader;
#[doc = "Field `SPC` writer - Security protection level option byte status bits"]
pub type SpcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BORST_EN` reader - Brown out reset enable"]
pub type BorstEnR = crate::BitReader;
#[doc = "Field `BORST_EN` writer - Brown out reset enable"]
pub type BorstEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BORR_TH` reader - BOR threshold at rising VDD supply"]
pub type BorrThR = crate::FieldReader;
#[doc = "Field `BORR_TH` writer - BOR threshold at rising VDD supply"]
pub type BorrThW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BORF_TH` reader - BOR threshold at falling VDD supply"]
pub type BorfThR = crate::FieldReader;
#[doc = "Field `BORF_TH` writer - BOR threshold at falling VDD supply"]
pub type BorfThW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `nRST_DPSLP` reader - Option byte deepsleep reset value"]
pub type NRstDpslpR = crate::BitReader;
#[doc = "Field `nRST_DPSLP` writer - Option byte deepsleep reset value"]
pub type NRstDpslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_STDBY` reader - Option byte standby reset value"]
pub type NRstStdbyR = crate::BitReader;
#[doc = "Field `nRST_STDBY` writer - Option byte standby reset value"]
pub type NRstStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nFWDG_HW` reader - Free watchdog configuration bit"]
pub type NFwdgHwR = crate::BitReader;
#[doc = "Field `nFWDG_HW` writer - Free watchdog configuration bit"]
pub type NFwdgHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nWWDG_HW` reader - Window watchdog configuration bit"]
pub type NWwdgHwR = crate::BitReader;
#[doc = "Field `nWWDG_HW` writer - Window watchdog configuration bit"]
pub type NWwdgHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HXTAL_REMAP` reader - HXTAL remapping"]
pub type HxtalRemapR = crate::BitReader;
#[doc = "Field `HXTAL_REMAP` writer - HXTAL remapping"]
pub type HxtalRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_ECCEN` reader - SRAM ECC disable"]
pub type SramEccenR = crate::BitReader;
#[doc = "Field `SRAM_ECCEN` writer - SRAM ECC disable"]
pub type SramEccenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWBT0` reader - Software BOOT0"]
pub type Swbt0R = crate::BitReader;
#[doc = "Field `SWBT0` writer - Software BOOT0"]
pub type Swbt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT1` reader - BOOT1 option bit"]
pub type NBoot1R = crate::BitReader;
#[doc = "Field `nBOOT1` writer - BOOT1 option bit"]
pub type NBoot1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT0` reader - BOOT0 option bit"]
pub type NBoot0R = crate::BitReader;
#[doc = "Field `nBOOT0` writer - BOOT0 option bit"]
pub type NBoot0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRST_MDSEL` reader - NRST pin mode selection."]
pub type NrstMdselR = crate::FieldReader;
#[doc = "Field `NRST_MDSEL` writer - NRST pin mode selection."]
pub type NrstMdselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Security protection level option byte status bits"]
    #[inline(always)]
    pub fn spc(&self) -> SpcR {
        SpcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Brown out reset enable"]
    #[inline(always)]
    pub fn borst_en(&self) -> BorstEnR {
        BorstEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - BOR threshold at rising VDD supply"]
    #[inline(always)]
    pub fn borr_th(&self) -> BorrThR {
        BorrThR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - BOR threshold at falling VDD supply"]
    #[inline(always)]
    pub fn borf_th(&self) -> BorfThR {
        BorfThR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Option byte deepsleep reset value"]
    #[inline(always)]
    pub fn n_rst_dpslp(&self) -> NRstDpslpR {
        NRstDpslpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Option byte standby reset value"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRstStdbyR {
        NRstStdbyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Free watchdog configuration bit"]
    #[inline(always)]
    pub fn n_fwdg_hw(&self) -> NFwdgHwR {
        NFwdgHwR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - Window watchdog configuration bit"]
    #[inline(always)]
    pub fn n_wwdg_hw(&self) -> NWwdgHwR {
        NWwdgHwR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - HXTAL remapping"]
    #[inline(always)]
    pub fn hxtal_remap(&self) -> HxtalRemapR {
        HxtalRemapR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SRAM ECC disable"]
    #[inline(always)]
    pub fn sram_eccen(&self) -> SramEccenR {
        SramEccenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Software BOOT0"]
    #[inline(always)]
    pub fn swbt0(&self) -> Swbt0R {
        Swbt0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - BOOT1 option bit"]
    #[inline(always)]
    pub fn n_boot1(&self) -> NBoot1R {
        NBoot1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BOOT0 option bit"]
    #[inline(always)]
    pub fn n_boot0(&self) -> NBoot0R {
        NBoot0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - NRST pin mode selection."]
    #[inline(always)]
    pub fn nrst_mdsel(&self) -> NrstMdselR {
        NrstMdselR::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Security protection level option byte status bits"]
    #[inline(always)]
    pub fn spc(&mut self) -> SpcW<'_, ObctlSpec> {
        SpcW::new(self, 0)
    }
    #[doc = "Bit 8 - Brown out reset enable"]
    #[inline(always)]
    pub fn borst_en(&mut self) -> BorstEnW<'_, ObctlSpec> {
        BorstEnW::new(self, 8)
    }
    #[doc = "Bits 9:10 - BOR threshold at rising VDD supply"]
    #[inline(always)]
    pub fn borr_th(&mut self) -> BorrThW<'_, ObctlSpec> {
        BorrThW::new(self, 9)
    }
    #[doc = "Bits 11:12 - BOR threshold at falling VDD supply"]
    #[inline(always)]
    pub fn borf_th(&mut self) -> BorfThW<'_, ObctlSpec> {
        BorfThW::new(self, 11)
    }
    #[doc = "Bit 13 - Option byte deepsleep reset value"]
    #[inline(always)]
    pub fn n_rst_dpslp(&mut self) -> NRstDpslpW<'_, ObctlSpec> {
        NRstDpslpW::new(self, 13)
    }
    #[doc = "Bit 14 - Option byte standby reset value"]
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> NRstStdbyW<'_, ObctlSpec> {
        NRstStdbyW::new(self, 14)
    }
    #[doc = "Bit 16 - Free watchdog configuration bit"]
    #[inline(always)]
    pub fn n_fwdg_hw(&mut self) -> NFwdgHwW<'_, ObctlSpec> {
        NFwdgHwW::new(self, 16)
    }
    #[doc = "Bit 19 - Window watchdog configuration bit"]
    #[inline(always)]
    pub fn n_wwdg_hw(&mut self) -> NWwdgHwW<'_, ObctlSpec> {
        NWwdgHwW::new(self, 19)
    }
    #[doc = "Bit 21 - HXTAL remapping"]
    #[inline(always)]
    pub fn hxtal_remap(&mut self) -> HxtalRemapW<'_, ObctlSpec> {
        HxtalRemapW::new(self, 21)
    }
    #[doc = "Bit 22 - SRAM ECC disable"]
    #[inline(always)]
    pub fn sram_eccen(&mut self) -> SramEccenW<'_, ObctlSpec> {
        SramEccenW::new(self, 22)
    }
    #[doc = "Bit 24 - Software BOOT0"]
    #[inline(always)]
    pub fn swbt0(&mut self) -> Swbt0W<'_, ObctlSpec> {
        Swbt0W::new(self, 24)
    }
    #[doc = "Bit 25 - BOOT1 option bit"]
    #[inline(always)]
    pub fn n_boot1(&mut self) -> NBoot1W<'_, ObctlSpec> {
        NBoot1W::new(self, 25)
    }
    #[doc = "Bit 26 - BOOT0 option bit"]
    #[inline(always)]
    pub fn n_boot0(&mut self) -> NBoot0W<'_, ObctlSpec> {
        NBoot0W::new(self, 26)
    }
    #[doc = "Bits 27:28 - NRST pin mode selection."]
    #[inline(always)]
    pub fn nrst_mdsel(&mut self) -> NrstMdselW<'_, ObctlSpec> {
        NrstMdselW::new(self, 27)
    }
}
#[doc = "Option byte control register\n\nYou can [`read`](crate::Reg::read) this register and get [`obctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ObctlSpec;
impl crate::RegisterSpec for ObctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obctl::R`](R) reader structure"]
impl crate::Readable for ObctlSpec {}
#[doc = "`write(|w| ..)` method takes [`obctl::W`](W) writer structure"]
impl crate::Writable for ObctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OBCTL to value 0"]
impl crate::Resettable for ObctlSpec {}
