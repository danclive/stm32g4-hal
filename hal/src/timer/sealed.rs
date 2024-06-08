use enumflags2::BitFlags;

use super::*;

pub trait General {
    type Width: Into<u32> + From<u16>;
    fn max_auto_reload() -> u32;
    unsafe fn set_auto_reload_unchecked(&mut self, arr: u32);
    fn set_auto_reload(&mut self, arr: u32) -> Result<(), super::Error>;
    fn read_auto_reload() -> u32;
    fn enable_preload(&mut self, b: bool);
    fn enable_counter(&mut self, b: bool);
    fn is_counter_enabled(&self) -> bool;
    fn set_prescaler(&mut self, psc: u16);
    fn read_prescaler(&self) -> u16;
    fn trigger_update(&mut self);
    fn listen_event(&mut self, disable: Option<BitFlags<Event>>, enable: Option<BitFlags<Event>>);
    fn clear_interrupt_flag(&mut self, event: BitFlags<Flag>);
    fn get_interrupt_flag(&self) -> BitFlags<Flag>;
    fn read_count(&self) -> Self::Width;
    fn write_count(&mut self, value: Self::Width);
    fn start_one_pulse(&mut self);
    fn start_free(&mut self, update: bool);
    fn reset_control_register1(&mut self);
    fn reset_counter(&mut self);
}
