#[doc = "Register `WS` reader"]
pub type R = crate::R<WsSpec>;
#[doc = "Register `WS` writer"]
pub type W = crate::W<WsSpec>;
#[doc = "Field `WSCNT` reader - Wait state counter register"]
pub type WscntR = crate::FieldReader;
#[doc = "Field `WSCNT` writer - Wait state counter register"]
pub type WscntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PFEN` reader - Pre-fetch enable"]
pub type PfenR = crate::BitReader;
#[doc = "Field `PFEN` writer - Pre-fetch enable"]
pub type PfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEN` reader - IBUS cache enable"]
pub type IcenR = crate::BitReader;
#[doc = "Field `ICEN` writer - IBUS cache enable"]
pub type IcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICRST` reader - IBUS cache reset."]
pub type IcrstR = crate::BitReader;
#[doc = "Field `ICRST` writer - IBUS cache reset."]
pub type IcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFPE` reader - Main flash programmed or empty flag"]
pub type MfpeR = crate::BitReader;
#[doc = "Field `MFPE` writer - Main flash programmed or empty flag"]
pub type MfpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGEN` reader - Debugger enable"]
pub type DbgenR = crate::BitReader;
#[doc = "Field `DBGEN` writer - Debugger enable"]
pub type DbgenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Wait state counter register"]
    #[inline(always)]
    pub fn wscnt(&self) -> WscntR {
        WscntR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Pre-fetch enable"]
    #[inline(always)]
    pub fn pfen(&self) -> PfenR {
        PfenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IBUS cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> IcenR {
        IcenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - IBUS cache reset."]
    #[inline(always)]
    pub fn icrst(&self) -> IcrstR {
        IcrstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Main flash programmed or empty flag"]
    #[inline(always)]
    pub fn mfpe(&self) -> MfpeR {
        MfpeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Debugger enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Wait state counter register"]
    #[inline(always)]
    pub fn wscnt(&mut self) -> WscntW<'_, WsSpec> {
        WscntW::new(self, 0)
    }
    #[doc = "Bit 8 - Pre-fetch enable"]
    #[inline(always)]
    pub fn pfen(&mut self) -> PfenW<'_, WsSpec> {
        PfenW::new(self, 8)
    }
    #[doc = "Bit 9 - IBUS cache enable"]
    #[inline(always)]
    pub fn icen(&mut self) -> IcenW<'_, WsSpec> {
        IcenW::new(self, 9)
    }
    #[doc = "Bit 11 - IBUS cache reset."]
    #[inline(always)]
    pub fn icrst(&mut self) -> IcrstW<'_, WsSpec> {
        IcrstW::new(self, 11)
    }
    #[doc = "Bit 16 - Main flash programmed or empty flag"]
    #[inline(always)]
    pub fn mfpe(&mut self) -> MfpeW<'_, WsSpec> {
        MfpeW::new(self, 16)
    }
    #[doc = "Bit 18 - Debugger enable"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DbgenW<'_, WsSpec> {
        DbgenW::new(self, 18)
    }
}
#[doc = "Wait state register\n\nYou can [`read`](crate::Reg::read) this register and get [`ws::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ws::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WsSpec;
impl crate::RegisterSpec for WsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ws::R`](R) reader structure"]
impl crate::Readable for WsSpec {}
#[doc = "`write(|w| ..)` method takes [`ws::W`](W) writer structure"]
impl crate::Writable for WsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WS to value 0x0004_0200"]
impl crate::Resettable for WsSpec {
    const RESET_VALUE: u32 = 0x0004_0200;
}
