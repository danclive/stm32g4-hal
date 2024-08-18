/// pins
use super::*;

use crate::pac;

use crate::gpio::*;

macro_rules! pins_tuples {
    // Tuple of two pins
    ($(($CHA:ty, $CHB:ty)),*) => {
        $(
            impl<TIM, CHA, CHB, TA, TB> Pins<TIM, ($CHA, $CHB), (TA, TB)> for (CHA, CHB)
            where
                CHA: Pins<TIM, $CHA, TA>,
                CHB: Pins<TIM, $CHB, TB>,
            {
                type Channel = (Pwm<TIM, $CHA, TA, ActiveHigh, ActiveHigh>, Pwm<TIM, $CHB, TB, ActiveHigh, ActiveHigh>);
            }
        )*
    };
    // Tuple of three pins
    ($(($CHA:ty, $CHB:ty, $CHC:ty)),*) => {
        $(
            pins_tuples! {
                PERM3: ($CHA, $CHB, $CHC),
                PERM3: ($CHA, $CHC, $CHB),
                PERM3: ($CHB, $CHA, $CHC),
                PERM3: ($CHB, $CHC, $CHA),
                PERM3: ($CHC, $CHA, $CHB),
                PERM3: ($CHC, $CHB, $CHA)
            }
        )*
    };
    // Permutate tuple of three pins
    ($(PERM3: ($CHA:ty, $CHB:ty, $CHC:ty)),*) => {
        $(
            impl<TIM, CHA, CHB, CHC, TA, TB, TC> Pins<TIM, ($CHA, $CHB, $CHC), (TA, TB, TC)> for (CHA, CHB, CHC)
            where
                CHA: Pins<TIM, $CHA, TA>,
                CHB: Pins<TIM, $CHB, TB>,
                CHC: Pins<TIM, $CHC, TC>,
            {
                type Channel = (Pwm<TIM, $CHA, TA, ActiveHigh, ActiveHigh>, Pwm<TIM, $CHB, TB, ActiveHigh, ActiveHigh>, Pwm<TIM, $CHC, TC, ActiveHigh, ActiveHigh>);
            }
        )*
    };
    // Tuple of four pins (permutates the last 3, leaves 4th in place)
    ($(($CHD:ty, $CHA:ty, $CHB:ty, $CHC:ty)),*) => {
        $(
            pins_tuples! {
                PERM4: ($CHD, $CHA, $CHB, $CHC),
                PERM4: ($CHD, $CHA, $CHC, $CHB),
                PERM4: ($CHD, $CHB, $CHA, $CHC),
                PERM4: ($CHD, $CHB, $CHC, $CHA),
                PERM4: ($CHD, $CHC, $CHA, $CHB),
                PERM4: ($CHD, $CHC, $CHB, $CHA)
            }
        )*
    };
    // Tuple of four pins (permutates the last 3, leaves 1st in place)
    ($(PERM4: ($CHA:ty, $CHB:ty, $CHC:ty, $CHD:ty)),*) => {
        $(
            impl<TIM, CHA, CHB, CHC, CHD, TA, TB, TC, TD> Pins<TIM, ($CHA, $CHB, $CHC, $CHD), (TA, TB, TC, TD)> for (CHA, CHB, CHC, CHD)
            where
                CHA: Pins<TIM, $CHA, TA>,
                CHB: Pins<TIM, $CHB, TB>,
                CHC: Pins<TIM, $CHC, TC>,
                CHD: Pins<TIM, $CHD, TD>,
            {
                type Channel = (Pwm<TIM, $CHA, TA, ActiveHigh, ActiveHigh>, Pwm<TIM, $CHB, TB, ActiveHigh, ActiveHigh>, Pwm<TIM, $CHC, TC, ActiveHigh, ActiveHigh>, Pwm<TIM, $CHD, TD, ActiveHigh, ActiveHigh>);
            }
        )*
    }
}

pins_tuples! {
    (C1, C2),
    (C2, C1),
    (C1, C3),
    (C3, C1),
    (C1, C4),
    (C4, C1),
    (C2, C3),
    (C3, C2),
    (C2, C4),
    (C4, C2),
    (C3, C4),
    (C4, C3)
}

pins_tuples! {
    (C1, C2, C3),
    (C1, C2, C4),
    (C1, C3, C4),
    (C2, C3, C4)
}

pins_tuples! {
    (C1, C2, C3, C4),
    (C2, C1, C3, C4),
    (C3, C1, C2, C4),
    (C4, C1, C2, C3)
}

// Pin definitions, mark which pins can be used with which timers and channels
macro_rules! pins {
    // Single channel timer
    ($TIMX:ident { OUT: [$($OUT:ty),*] }) => {
        $(
            impl Pins<pac::$TIMX, C1, ComplementaryImpossible> for $OUT {
                type Channel = Pwm<pac::$TIMX, C1, ComplementaryImpossible, ActiveHigh, ActiveHigh>;
            }
        )*
    };
    // Dual channel timer $pm
    ($TIMX:ident {
        CH1($COMP1:ty): [$($( #[ $pmeta1:meta ] )* $CH1:ty),*]
        CH2($COMP2:ty): [$($( #[ $pmeta2:meta ] )* $CH2:ty),*]
        CH1N: [$($( #[ $pmeta3:meta ] )* $CH1N:ty),*]
        CH2N: [$($( #[ $pmeta4:meta ] )* $CH2N:ty),*]
        BRK: [$($( #[ $pmeta5:meta ] )* $BRK:ty),*]
        BRK2: [$($( #[ $pmeta6:meta ] )* $BRK2:ty),*] }) => {

        $(
            $( #[ $pmeta1 ] )*
            impl Pins<pac::$TIMX, C1, $COMP1> for $CH1 {
                type Channel = Pwm<pac::$TIMX, C1, $COMP1, ActiveHigh, ActiveHigh>;
            }
        )*
        $(
            $( #[ $pmeta2 ] )*
            impl Pins<pac::$TIMX, C2, $COMP2> for $CH2 {
                type Channel = Pwm<pac::$TIMX, C2, $COMP2, ActiveHigh, ActiveHigh>;
            }
        )*
        $(
            $( #[ $pmeta3 ] )*
            impl NPins<pac::$TIMX, C1> for $CH1N {}
        )*
        $(
            $( #[ $pmeta4 ] )*
            impl NPins<pac::$TIMX, C2> for $CH2N {}
        )*
        $(
            $( #[ $pmeta5 ] )*
            impl FaultPins<pac::$TIMX,> for $BRK {
                const INPUT: BreakInput = BreakInput::BreakIn;
            }
        )*
        $(
            $( #[ $pmeta6 ] )*
            impl FaultPins<pac::$TIMX> for $BRK2 {
                const INPUT: BreakInput = BreakInput::BreakIn2;
            }
        )*
    };
    // Quad channel timers
    ($TIMX:ident {
        CH1($COMP1:ty): [$($( #[ $pmeta1:meta ] )* $CH1:ty),*]
        CH2($COMP2:ty): [$($( #[ $pmeta2:meta ] )* $CH2:ty),*]
        CH3($COMP3:ty): [$($( #[ $pmeta3:meta ] )* $CH3:ty),*]
        CH4($COMP4:ty): [$($( #[ $pmeta4:meta ] )* $CH4:ty),*]
        CH1N: [$($( #[ $pmeta5:meta ] )* $CH1N:ty),*]
        CH2N: [$($( #[ $pmeta6:meta ] )* $CH2N:ty),*]
        CH3N: [$($( #[ $pmeta7:meta ] )* $CH3N:ty),*]
        CH4N: [$($( #[ $pmeta8:meta ] )* $CH4N:ty),*]
        BRK: [$($( #[ $pmeta9:meta ] )* $BRK:ty),*]
        BRK2: [$($( #[ $pmeta10:meta ] )* $BRK2:ty),*] }) => {

        $(
            $( #[ $pmeta1 ] )*
            impl Pins<pac::$TIMX, C1, $COMP1> for $CH1 {
                type Channel = Pwm<pac::$TIMX, C1, $COMP1, ActiveHigh, ActiveHigh>;
            }
        )*
        $(
            $( #[ $pmeta2 ] )*
            impl Pins<pac::$TIMX, C2, $COMP2> for $CH2 {
                type Channel = Pwm<pac::$TIMX, C2, $COMP2, ActiveHigh, ActiveHigh>;
            }
        )*
        $(
            $( #[ $pmeta3 ] )*
            impl Pins<pac::$TIMX, C3, $COMP3> for $CH3 {
                type Channel = Pwm<pac::$TIMX, C3, $COMP3, ActiveHigh, ActiveHigh>;
            }
        )*
        $(
            $( #[ $pmeta4 ] )*
            impl Pins<pac::$TIMX, C4, $COMP4> for $CH4 {
                type Channel = Pwm<pac::$TIMX, C4, $COMP4, ActiveHigh, ActiveHigh>;
            }
        )*
        $(
            $( #[ $pmeta5 ] )*
            impl NPins<pac::$TIMX, C1> for $CH1N {}
        )*
        $(
            $( #[ $pmeta6 ] )*
            impl NPins<pac::$TIMX, C2> for $CH2N {}
        )*
        $(
            $( #[ $pmeta7 ] )*
            impl NPins<pac::$TIMX, C3> for $CH3N {}
        )*
        $(
            $( #[ $pmeta8 ] )*
            impl NPins<pac::$TIMX, C4> for $CH4N {}
        )*
        $(
            $( #[ $pmeta9 ] )*
            impl FaultPins<pac::$TIMX> for $BRK {
                const INPUT: BreakInput = BreakInput::BreakIn;
            }
        )*
        $(
            $( #[ $pmeta10 ] )*
            impl FaultPins<pac::$TIMX> for $BRK2 {
                const INPUT: BreakInput = BreakInput::BreakIn2;
            }
        )*
    }
}

// Single channel timers
pins! {
    Lptimer1 {
        OUT: [
            PA14<Alt<1>>,
            PB2<Alt<1>>,
            PC1<Alt<1>>
        ]
    }
}

pins! {
    Tim1 {
        CH1(ComplementaryDisabled): [
            PA8<Alt<6>>,
            PC0<Alt<2>>,
            PE9<Alt<2>>
        ]
        CH2(ComplementaryDisabled): [
            PA9<Alt<6>>,
            PC1<Alt<2>>,
            PE11<Alt<2>>
        ]
        CH3(ComplementaryDisabled): [
            PA10<Alt<6>>,
            PC2<Alt<2>>,
            PE13<Alt<2>>
        ]
        CH4(ComplementaryDisabled): [
            PA11<Alt<11>>,
            PC3<Alt<2>>,
            PE14<Alt<2>>
        ]
        CH1N: [
            PA7<Alt<6>>,
            PA11<Alt<6>>,
            PB13<Alt<6>>,
            PC13<Alt<4>>,
            PE8<Alt<2>>
        ]
        CH2N: [
            PA12<Alt<6>>,
            PB0<Alt<6>>,
            PB14<Alt<6>>,
            PE10<Alt<2>>
        ]
        CH3N: [
            PB1<Alt<6>>,
            PB9<Alt<12>>,
            PB15<Alt<4>>,
            PE12<Alt<2>>,
            PF0<Alt<6>>
        ]
        CH4N: [
            PC5<Alt<6>>,
            PE15<Alt<6>>
        ]
        BRK: [
            PA6<Alt<6>>,
            PA14<Alt<6>>,
            PA15<Alt<9>>,
            PB8<Alt<12>>,
            PB10<Alt<12>>,
            PB12<Alt<6>>,
            PC13<Alt<2>>,
            PE15<Alt<2>>
        ]
        BRK2: [
            PA11<Alt<12>>,
            PC3<Alt<6>>,
            PE14<Alt<6>>
        ]
    }
}

pins! {
    Tim2 {
        CH1(ComplementaryImpossible): [
            PA0<Alt<1>>,
            PA5<Alt<1>>,
            PA15<Alt<1>>,
            PD3<Alt<2>>
        ]
        CH2(ComplementaryImpossible): [
            PA1<Alt<1>>,
            PB3<Alt<1>>,
            PD4<Alt<2>>
        ]
        CH3(ComplementaryImpossible): [
            PA2<Alt<1>>,
            PA9<Alt<10>>,
            PB10<Alt<1>>,
            PD7<Alt<2>>
        ]
        CH4(ComplementaryImpossible): [
            PA3<Alt<1>>,
            PA10<Alt<10>>,
            PB11<Alt<1>>,
            PD6<Alt<2>>
        ]
        CH1N: []
        CH2N: []
        CH3N: []
        CH4N: []
        BRK: []
        BRK2: []
    }
}

pins! {
    Tim3 {
        CH1(ComplementaryImpossible): [
            PA6<Alt<2>>,
            PB4<Alt<2>>,
            PC6<Alt<2>>,
            PE2<Alt<2>>
        ]
        CH2(ComplementaryImpossible): [
            PA4<Alt<2>>,
            PA7<Alt<2>>,
            PB5<Alt<2>>,
            PC7<Alt<2>>,
            PE3<Alt<2>>
        ]
        CH3(ComplementaryImpossible): [
            PB0<Alt<2>>,
            PC8<Alt<2>>,
            PE4<Alt<2>>
        ]
        CH4(ComplementaryImpossible): [
            PB1<Alt<2>>,
            PB7<Alt<10>>,
            PC9<Alt<2>>,
            PE5<Alt<2>>
        ]
        CH1N: []
        CH2N: []
        CH3N: []
        CH4N: []
        BRK: []
        BRK2: []
    }
}

pins! {
    Tim4 {
        CH1(ComplementaryImpossible): [
            PA11<Alt<10>>,
            PB6<Alt<2>>,
            PD12<Alt<2>>
        ]
        CH2(ComplementaryImpossible): [
            PA12<Alt<10>>,
            PB7<Alt<2>>,
            PD13<Alt<2>>
        ]
        CH3(ComplementaryImpossible): [
            PA13<Alt<10>>,
            PB8<Alt<2>>,
            PD14<Alt<2>>
        ]
        CH4(ComplementaryImpossible): [
            PB9<Alt<2>>,
            PD15<Alt<2>>,
            #[cfg(any(
                feature = "stm32g471",
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484"
            ))]
            PF6<Alt<2>>
        ]
        CH1N: []
        CH2N: []
        CH3N: []
        CH4N: []
        BRK: []
        BRK2: []
    }
}

#[cfg(feature = "tim5")]
pins! {
    Tim5 {
        CH1(ComplementaryImpossible): [
            PA0<Alt<2>>,
            PB2<Alt<2>>,
            PF6<Alt<6>>
        ]
        CH2(ComplementaryImpossible): [
            PA1<Alt<2>>,
            PC12<Alt<1>>,
            PF7<Alt<6>>
        ]
        CH3(ComplementaryImpossible): [
            PA2<Alt<2>>,
            PE8<Alt<1>>,
            PF8<Alt<6>>
        ]
        CH4(ComplementaryImpossible): [
            PA3<Alt<2>>,
            PE9<Alt<1>>,
            PF9<Alt<6>>
        ]
        CH1N: []
        CH2N: []
        CH3N: []
        CH4N: []
        BRK: []
        BRK2: []
    }
}

pins! {
    Tim8 {
        CH1(ComplementaryDisabled): [
            PA15<Alt<2>>,
            PB6<Alt<5>>,
            PC6<Alt<4>>
        ]
        CH2(ComplementaryDisabled): [
            PA14<Alt<5>>,
            PB8<Alt<10>>,
            PC7<Alt<4>>
        ]
        CH3(ComplementaryDisabled): [
            PB9<Alt<10>>,
            PC8<Alt<4>>
        ]
        CH4(ComplementaryDisabled): [
            PC9<Alt<4>>,
            PD1<Alt<4>>
        ]
        CH1N: [
            PA7<Alt<4>>,
            PB3<Alt<4>>,
            PC10<Alt<4>>
        ]
        CH2N: [
            PB0<Alt<4>>,
            PB4<Alt<4>>,
            PC11<Alt<4>>
        ]
        CH3N: [
            PB1<Alt<4>>,
            PB5<Alt<3>>,
            PC12<Alt<4>>
        ]
        CH4N: [
            PC13<Alt<6>>,
            PD0<Alt<6>>
        ]
        BRK: [
            PA0<Alt<9>>,
            PA6<Alt<4>>,
            PA10<Alt<11>>,
            PB7<Alt<5>>,
            PD2<Alt<4>>
        ]
        BRK2: [
            PB6<Alt<10>>,
            PC9<Alt<6>>,
            PD1<Alt<6>>
        ]
    }
}

pins! {
    Tim15 {
        CH1(ComplementaryDisabled): [
            PA2<Alt<9>>,
            PB14<Alt<1>>,
            PF9<Alt<3>>
        ]
        CH2(ComplementaryImpossible): [
            PA3<Alt<9>>,
            PB15<Alt<1>>,
            PF10<Alt<3>>
        ]
        CH1N: [
            PA1<Alt<9>>,
            PB15<Alt<2>>,
            #[cfg(any(
                feature = "stm32g471",
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484"
            ))]
            PG9<Alt<14>>
        ]
        CH2N: []
        BRK: [
            PA9<Alt<9>>,
            PC5<Alt<2>>
        ]
        BRK2: []
    }
}

pins! {
    Tim16 {
        CH1(ComplementaryDisabled): [
            PA6<Alt<1>>,
            PA12<Alt<1>>,
            PB4<Alt<1>>,
            PB8<Alt<1>>,
            PE0<Alt<4>>
        ]
        CH2(ComplementaryImpossible): []
        CH1N: [
            PA13<Alt<1>>,
            PB6<Alt<1>>
        ]
        CH2N: []
        BRK: [
            PB5<Alt<1>>
        ]
        BRK2: []
    }
}

pins! {
    Tim17 {
        CH1(ComplementaryDisabled): [
            PA7<Alt<1>>,
            PB5<Alt<10>>,
            PB9<Alt<1>>,
            PE1<Alt<4>>
        ]
        CH2(ComplementaryImpossible): []
        CH1N: [
            PB7<Alt<1>>
        ]
        CH2N: []
        BRK: [
            PA10<Alt<1>>,
            PB4<Alt<10>>
        ]
        BRK2: []
    }
}

#[cfg(feature = "tim20")]
pins! {
    Tim20 {
        CH1(ComplementaryDisabled): [
            PB2<Alt<3>>,
            PE2<Alt<6>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PF12<Alt<2>>
        ]
        CH2(ComplementaryDisabled): [
            PC2<Alt<6>>,
            PE3<Alt<6>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PF13<Alt<2>>
        ]
        CH3(ComplementaryDisabled): [
            PC8<Alt<6>>,
            PF2<Alt<2>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PF14<Alt<2>>
        ]
        CH4(ComplementaryDisabled): [
            PE1<Alt<4>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PF3<Alt<2>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PF15<Alt<2>>
        ]
        CH1N: [
            PE4<Alt<6>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PF4<Alt<3>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PG0<Alt<2>>
        ]
        CH2N: [
            PE5<Alt<6>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PF5<Alt<2>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PG1<Alt<2>>
        ]
        CH3N: [
            PE6<Alt<6>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PG2<Alt<2>>
        ]
        CH4N: [
            PE0<Alt<3>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PG3<Alt<6>>
        ]
        BRK: [
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PF7<Alt<2>>,
            PF9<Alt<2>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PG3<Alt<2>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PG6<Alt<2>>
        ]
        BRK2: [
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PF8<Alt<2>>,
            PF10<Alt<2>>,
            #[cfg(any(
                feature = "stm32g473",
                feature = "stm32g474",
                feature = "stm32g483",
                feature = "stm32g484",
            ))]
            PG4<Alt<2>>
        ]
    }
}
