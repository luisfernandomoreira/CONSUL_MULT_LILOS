#[doc = "Register `CCHP` reader"]
pub type R = crate::R<CchpSpec>;
#[doc = "Register `CCHP` writer"]
pub type W = crate::W<CchpSpec>;
#[doc = "Field `DTCFG` reader - Dead time configure"]
pub type DtcfgR = crate::FieldReader;
#[doc = "Field `DTCFG` writer - Dead time configure"]
pub type DtcfgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PROT` reader - Complementary register protect control"]
pub type ProtR = crate::FieldReader;
#[doc = "Field `PROT` writer - Complementary register protect control"]
pub type ProtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOS` reader - Idle mode 'off-state' enable"]
pub type IosR = crate::BitReader;
#[doc = "Field `IOS` writer - Idle mode 'off-state' enable"]
pub type IosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROS` reader - Run mode 'off-state' enable"]
pub type RosR = crate::BitReader;
#[doc = "Field `ROS` writer - Run mode 'off-state' enable"]
pub type RosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK0EN` reader - BREAK0 input signal enable"]
pub type Brk0enR = crate::BitReader;
#[doc = "Field `BRK0EN` writer - BREAK0 input signal enable"]
pub type Brk0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK0P` reader - BREAK0 input signal polarity"]
pub type Brk0pR = crate::BitReader;
#[doc = "Field `BRK0P` writer - BREAK0 input signal polarity"]
pub type Brk0pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OAEN` reader - Output automatic enable"]
pub type OaenR = crate::BitReader;
#[doc = "Field `OAEN` writer - Output automatic enable"]
pub type OaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POEN` reader - Primary output enable"]
pub type PoenR = crate::BitReader;
#[doc = "Field `POEN` writer - Primary output enable"]
pub type PoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK0F` reader - BREAK0 input signal filter"]
pub type Brk0fR = crate::FieldReader;
#[doc = "Field `BRK0F` writer - BREAK0 input signal filter"]
pub type Brk0fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BRK1F` reader - BREAK1 input signal filter"]
pub type Brk1fR = crate::FieldReader;
#[doc = "Field `BRK1F` writer - BREAK1 input signal filter"]
pub type Brk1fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BRK1EN` reader - BREAK1 input signal enable"]
pub type Brk1enR = crate::BitReader;
#[doc = "Field `BRK1EN` writer - BREAK1 input signal enable"]
pub type Brk1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK1P` reader - BREAK1 input signal polarity"]
pub type Brk1pR = crate::BitReader;
#[doc = "Field `BRK1P` writer - BREAK1 input signal polarity"]
pub type Brk1pW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dead time configure"]
    #[inline(always)]
    pub fn dtcfg(&self) -> DtcfgR {
        DtcfgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Complementary register protect control"]
    #[inline(always)]
    pub fn prot(&self) -> ProtR {
        ProtR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Idle mode 'off-state' enable"]
    #[inline(always)]
    pub fn ios(&self) -> IosR {
        IosR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Run mode 'off-state' enable"]
    #[inline(always)]
    pub fn ros(&self) -> RosR {
        RosR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BREAK0 input signal enable"]
    #[inline(always)]
    pub fn brk0en(&self) -> Brk0enR {
        Brk0enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BREAK0 input signal polarity"]
    #[inline(always)]
    pub fn brk0p(&self) -> Brk0pR {
        Brk0pR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output automatic enable"]
    #[inline(always)]
    pub fn oaen(&self) -> OaenR {
        OaenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Primary output enable"]
    #[inline(always)]
    pub fn poen(&self) -> PoenR {
        PoenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - BREAK0 input signal filter"]
    #[inline(always)]
    pub fn brk0f(&self) -> Brk0fR {
        Brk0fR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - BREAK1 input signal filter"]
    #[inline(always)]
    pub fn brk1f(&self) -> Brk1fR {
        Brk1fR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - BREAK1 input signal enable"]
    #[inline(always)]
    pub fn brk1en(&self) -> Brk1enR {
        Brk1enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - BREAK1 input signal polarity"]
    #[inline(always)]
    pub fn brk1p(&self) -> Brk1pR {
        Brk1pR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead time configure"]
    #[inline(always)]
    pub fn dtcfg(&mut self) -> DtcfgW<'_, CchpSpec> {
        DtcfgW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Complementary register protect control"]
    #[inline(always)]
    pub fn prot(&mut self) -> ProtW<'_, CchpSpec> {
        ProtW::new(self, 8)
    }
    #[doc = "Bit 10 - Idle mode 'off-state' enable"]
    #[inline(always)]
    pub fn ios(&mut self) -> IosW<'_, CchpSpec> {
        IosW::new(self, 10)
    }
    #[doc = "Bit 11 - Run mode 'off-state' enable"]
    #[inline(always)]
    pub fn ros(&mut self) -> RosW<'_, CchpSpec> {
        RosW::new(self, 11)
    }
    #[doc = "Bit 12 - BREAK0 input signal enable"]
    #[inline(always)]
    pub fn brk0en(&mut self) -> Brk0enW<'_, CchpSpec> {
        Brk0enW::new(self, 12)
    }
    #[doc = "Bit 13 - BREAK0 input signal polarity"]
    #[inline(always)]
    pub fn brk0p(&mut self) -> Brk0pW<'_, CchpSpec> {
        Brk0pW::new(self, 13)
    }
    #[doc = "Bit 14 - Output automatic enable"]
    #[inline(always)]
    pub fn oaen(&mut self) -> OaenW<'_, CchpSpec> {
        OaenW::new(self, 14)
    }
    #[doc = "Bit 15 - Primary output enable"]
    #[inline(always)]
    pub fn poen(&mut self) -> PoenW<'_, CchpSpec> {
        PoenW::new(self, 15)
    }
    #[doc = "Bits 16:19 - BREAK0 input signal filter"]
    #[inline(always)]
    pub fn brk0f(&mut self) -> Brk0fW<'_, CchpSpec> {
        Brk0fW::new(self, 16)
    }
    #[doc = "Bits 20:23 - BREAK1 input signal filter"]
    #[inline(always)]
    pub fn brk1f(&mut self) -> Brk1fW<'_, CchpSpec> {
        Brk1fW::new(self, 20)
    }
    #[doc = "Bit 24 - BREAK1 input signal enable"]
    #[inline(always)]
    pub fn brk1en(&mut self) -> Brk1enW<'_, CchpSpec> {
        Brk1enW::new(self, 24)
    }
    #[doc = "Bit 25 - BREAK1 input signal polarity"]
    #[inline(always)]
    pub fn brk1p(&mut self) -> Brk1pW<'_, CchpSpec> {
        Brk1pW::new(self, 25)
    }
}
#[doc = "Complementary channel protection register\n\nYou can [`read`](crate::Reg::read) this register and get [`cchp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cchp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CchpSpec;
impl crate::RegisterSpec for CchpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cchp::R`](R) reader structure"]
impl crate::Readable for CchpSpec {}
#[doc = "`write(|w| ..)` method takes [`cchp::W`](W) writer structure"]
impl crate::Writable for CchpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCHP to value 0"]
impl crate::Resettable for CchpSpec {}
