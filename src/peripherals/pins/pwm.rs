//! PWM Pin abstractions.


use crate::prelude::*;

use super::*;


/// PWM Function selector.
const FUNCSEL : u32 = 4;


pub trait PWMPin<const N: usize>: PinTrait {}

impl PWMPin<0> for Gpio<00> {}
impl PWMPin<0> for Gpio<01> {}

impl PWMPin<1> for Gpio<02> {}
impl PWMPin<1> for Gpio<03> {}

impl PWMPin<2> for Gpio<04> {}
impl PWMPin<2> for Gpio<05> {}

impl PWMPin<3> for Gpio<06> {}
impl PWMPin<3> for Gpio<07> {}

impl PWMPin<4> for Gpio<08> {}
impl PWMPin<4> for Gpio<09> {}

impl PWMPin<5> for Gpio<10> {}
impl PWMPin<5> for Gpio<11> {}

impl PWMPin<6> for Gpio<12> {}
impl PWMPin<6> for Gpio<13> {}

impl PWMPin<7> for Gpio<14> {}
impl PWMPin<7> for Gpio<15> {}

impl PWMPin<0> for Gpio<16> {}
impl PWMPin<0> for Gpio<17> {}

impl PWMPin<1> for Gpio<18> {}
impl PWMPin<1> for Gpio<19> {}

impl PWMPin<2> for Gpio<20> {}
impl PWMPin<2> for Gpio<21> {}

impl PWMPin<3> for Gpio<22> {}
impl PWMPin<3> for Gpio<23> {}

impl PWMPin<4> for Gpio<24> {}
impl PWMPin<4> for Gpio<25> {}

impl PWMPin<5> for Gpio<26> {}
impl PWMPin<5> for Gpio<27> {}

impl PWMPin<6> for Gpio<28> {}
impl PWMPin<6> for Gpio<29> {}



pub trait PWMChannelA<const N: usize> {}

impl PWMChannelA<0> for Gpio<00> {}
impl PWMChannelA<1> for Gpio<02> {}
impl PWMChannelA<2> for Gpio<04> {}
impl PWMChannelA<3> for Gpio<06> {}
impl PWMChannelA<4> for Gpio<08> {}
impl PWMChannelA<5> for Gpio<10> {}
impl PWMChannelA<6> for Gpio<12> {}
impl PWMChannelA<7> for Gpio<14> {}
impl PWMChannelA<0> for Gpio<16> {}
impl PWMChannelA<1> for Gpio<18> {}
impl PWMChannelA<2> for Gpio<20> {}
impl PWMChannelA<3> for Gpio<22> {}
impl PWMChannelA<4> for Gpio<24> {}
impl PWMChannelA<5> for Gpio<26> {}
impl PWMChannelA<6> for Gpio<28> {}



pub trait PWMChannelB<const N: usize> {}

impl PWMChannelB<0> for Gpio<01> {}
impl PWMChannelB<1> for Gpio<03> {}
impl PWMChannelB<2> for Gpio<05> {}
impl PWMChannelB<3> for Gpio<07> {}
impl PWMChannelB<4> for Gpio<09> {}
impl PWMChannelB<5> for Gpio<11> {}
impl PWMChannelB<6> for Gpio<13> {}
impl PWMChannelB<7> for Gpio<15> {}
impl PWMChannelB<0> for Gpio<17> {}
impl PWMChannelB<1> for Gpio<19> {}
impl PWMChannelB<2> for Gpio<21> {}
impl PWMChannelB<3> for Gpio<23> {}
impl PWMChannelB<4> for Gpio<25> {}
impl PWMChannelB<5> for Gpio<27> {}
impl PWMChannelB<6> for Gpio<29> {}




impl PWMPin<0> for NULLPIN {}
impl PWMPin<1> for NULLPIN {}
impl PWMPin<2> for NULLPIN {}
impl PWMPin<3> for NULLPIN {}
impl PWMPin<4> for NULLPIN {}
impl PWMPin<5> for NULLPIN {}
impl PWMPin<6> for NULLPIN {}
impl PWMPin<7> for NULLPIN {}

impl PWMChannelA<0> for NULLPIN {}
impl PWMChannelA<1> for NULLPIN {}
impl PWMChannelA<2> for NULLPIN {}
impl PWMChannelA<3> for NULLPIN {}
impl PWMChannelA<4> for NULLPIN {}
impl PWMChannelA<5> for NULLPIN {}
impl PWMChannelA<6> for NULLPIN {}
impl PWMChannelA<7> for NULLPIN {}

impl PWMChannelB<0> for NULLPIN {}
impl PWMChannelB<1> for NULLPIN {}
impl PWMChannelB<2> for NULLPIN {}
impl PWMChannelB<3> for NULLPIN {}
impl PWMChannelB<4> for NULLPIN {}
impl PWMChannelB<5> for NULLPIN {}
impl PWMChannelB<6> for NULLPIN {}
impl PWMChannelB<7> for NULLPIN {}
