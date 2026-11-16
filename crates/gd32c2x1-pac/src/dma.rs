#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intf: Intf,
    intc: Intc,
    ch0ctl: Ch0ctl,
    ch0cnt: Ch0cnt,
    ch0paddr: Ch0paddr,
    ch0maddr: Ch0maddr,
    _reserved6: [u8; 0x04],
    ch1ctl: Ch1ctl,
    ch1cnt: Ch1cnt,
    ch1paddr: Ch1paddr,
    ch1maddr: Ch1maddr,
    _reserved10: [u8; 0x04],
    ch2ctl: Ch2ctl,
    ch2cnt: Ch2cnt,
    ch2paddr: Ch2paddr,
    ch2maddr: Ch2maddr,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA interrupt flag register (DMA_INTF)"]
    #[inline(always)]
    pub const fn intf(&self) -> &Intf {
        &self.intf
    }
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_INTC)"]
    #[inline(always)]
    pub const fn intc(&self) -> &Intc {
        &self.intc
    }
    #[doc = "0x08 - DMA channel configuration register (DMA_CH0CTL)"]
    #[inline(always)]
    pub const fn ch0ctl(&self) -> &Ch0ctl {
        &self.ch0ctl
    }
    #[doc = "0x0c - DMA channel 0 counter register"]
    #[inline(always)]
    pub const fn ch0cnt(&self) -> &Ch0cnt {
        &self.ch0cnt
    }
    #[doc = "0x10 - DMA channel 0 peripheral base address register"]
    #[inline(always)]
    pub const fn ch0paddr(&self) -> &Ch0paddr {
        &self.ch0paddr
    }
    #[doc = "0x14 - DMA channel 0 memory base address register"]
    #[inline(always)]
    pub const fn ch0maddr(&self) -> &Ch0maddr {
        &self.ch0maddr
    }
    #[doc = "0x1c - DMA channel configuration register (DMA_CH1CTL)"]
    #[inline(always)]
    pub const fn ch1ctl(&self) -> &Ch1ctl {
        &self.ch1ctl
    }
    #[doc = "0x20 - DMA channel 1 counter register"]
    #[inline(always)]
    pub const fn ch1cnt(&self) -> &Ch1cnt {
        &self.ch1cnt
    }
    #[doc = "0x24 - DMA channel 1 peripheral base address register"]
    #[inline(always)]
    pub const fn ch1paddr(&self) -> &Ch1paddr {
        &self.ch1paddr
    }
    #[doc = "0x28 - DMA channel 1 memory base address register"]
    #[inline(always)]
    pub const fn ch1maddr(&self) -> &Ch1maddr {
        &self.ch1maddr
    }
    #[doc = "0x30 - DMA channel configuration register (DMA_CH2CTL)"]
    #[inline(always)]
    pub const fn ch2ctl(&self) -> &Ch2ctl {
        &self.ch2ctl
    }
    #[doc = "0x34 - DMA channel 2 counter register"]
    #[inline(always)]
    pub const fn ch2cnt(&self) -> &Ch2cnt {
        &self.ch2cnt
    }
    #[doc = "0x38 - DMA channel 2 peripheral base address register"]
    #[inline(always)]
    pub const fn ch2paddr(&self) -> &Ch2paddr {
        &self.ch2paddr
    }
    #[doc = "0x3c - DMA channel 2 memory base address register"]
    #[inline(always)]
    pub const fn ch2maddr(&self) -> &Ch2maddr {
        &self.ch2maddr
    }
}
#[doc = "INTF (r) register accessor: DMA interrupt flag register (DMA_INTF)\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intf`] module"]
#[doc(alias = "INTF")]
pub type Intf = crate::Reg<intf::IntfSpec>;
#[doc = "DMA interrupt flag register (DMA_INTF)"]
pub mod intf;
#[doc = "INTC (w) register accessor: DMA interrupt flag clear register (DMA_INTC)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc`] module"]
#[doc(alias = "INTC")]
pub type Intc = crate::Reg<intc::IntcSpec>;
#[doc = "DMA interrupt flag clear register (DMA_INTC)"]
pub mod intc;
#[doc = "CH0CTL (rw) register accessor: DMA channel configuration register (DMA_CH0CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0ctl`] module"]
#[doc(alias = "CH0CTL")]
pub type Ch0ctl = crate::Reg<ch0ctl::Ch0ctlSpec>;
#[doc = "DMA channel configuration register (DMA_CH0CTL)"]
pub mod ch0ctl;
#[doc = "CH0CNT (rw) register accessor: DMA channel 0 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cnt`] module"]
#[doc(alias = "CH0CNT")]
pub type Ch0cnt = crate::Reg<ch0cnt::Ch0cntSpec>;
#[doc = "DMA channel 0 counter register"]
pub mod ch0cnt;
#[doc = "CH0PADDR (rw) register accessor: DMA channel 0 peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0paddr`] module"]
#[doc(alias = "CH0PADDR")]
pub type Ch0paddr = crate::Reg<ch0paddr::Ch0paddrSpec>;
#[doc = "DMA channel 0 peripheral base address register"]
pub mod ch0paddr;
#[doc = "CH0MADDR (rw) register accessor: DMA channel 0 memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0maddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0maddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0maddr`] module"]
#[doc(alias = "CH0MADDR")]
pub type Ch0maddr = crate::Reg<ch0maddr::Ch0maddrSpec>;
#[doc = "DMA channel 0 memory base address register"]
pub mod ch0maddr;
#[doc = "CH1CTL (rw) register accessor: DMA channel configuration register (DMA_CH1CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ctl`] module"]
#[doc(alias = "CH1CTL")]
pub type Ch1ctl = crate::Reg<ch1ctl::Ch1ctlSpec>;
#[doc = "DMA channel configuration register (DMA_CH1CTL)"]
pub mod ch1ctl;
#[doc = "CH1CNT (rw) register accessor: DMA channel 1 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cnt`] module"]
#[doc(alias = "CH1CNT")]
pub type Ch1cnt = crate::Reg<ch1cnt::Ch1cntSpec>;
#[doc = "DMA channel 1 counter register"]
pub mod ch1cnt;
#[doc = "CH1PADDR (rw) register accessor: DMA channel 1 peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1paddr`] module"]
#[doc(alias = "CH1PADDR")]
pub type Ch1paddr = crate::Reg<ch1paddr::Ch1paddrSpec>;
#[doc = "DMA channel 1 peripheral base address register"]
pub mod ch1paddr;
#[doc = "CH1MADDR (rw) register accessor: DMA channel 1 memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1maddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1maddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1maddr`] module"]
#[doc(alias = "CH1MADDR")]
pub type Ch1maddr = crate::Reg<ch1maddr::Ch1maddrSpec>;
#[doc = "DMA channel 1 memory base address register"]
pub mod ch1maddr;
#[doc = "CH2CTL (rw) register accessor: DMA channel configuration register (DMA_CH2CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2ctl`] module"]
#[doc(alias = "CH2CTL")]
pub type Ch2ctl = crate::Reg<ch2ctl::Ch2ctlSpec>;
#[doc = "DMA channel configuration register (DMA_CH2CTL)"]
pub mod ch2ctl;
#[doc = "CH2CNT (rw) register accessor: DMA channel 2 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cnt`] module"]
#[doc(alias = "CH2CNT")]
pub type Ch2cnt = crate::Reg<ch2cnt::Ch2cntSpec>;
#[doc = "DMA channel 2 counter register"]
pub mod ch2cnt;
#[doc = "CH2PADDR (rw) register accessor: DMA channel 2 peripheral base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2paddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2paddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2paddr`] module"]
#[doc(alias = "CH2PADDR")]
pub type Ch2paddr = crate::Reg<ch2paddr::Ch2paddrSpec>;
#[doc = "DMA channel 2 peripheral base address register"]
pub mod ch2paddr;
#[doc = "CH2MADDR (rw) register accessor: DMA channel 2 memory base address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2maddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2maddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2maddr`] module"]
#[doc(alias = "CH2MADDR")]
pub type Ch2maddr = crate::Reg<ch2maddr::Ch2maddrSpec>;
#[doc = "DMA channel 2 memory base address register"]
pub mod ch2maddr;
