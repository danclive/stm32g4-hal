fn main() {
    println!("Hello, world!");
}

// Calculate I2C timing for Analog Filter ON, Digital Filter OFF
macro_rules! i2c_timing {
    ($i2cclk:ident, $freq:ident) => {{
        // Refer to RM0433 Rev 7 Figure 539 for setup and hold timing:
        //
        // t_I2CCLK = 1 / PCLK1
        // t_PRESC  = (PRESC + 1) * t_I2CCLK
        // t_SCLL   = (SCLL + 1) * t_PRESC
        // t_SCLH   = (SCLH + 1) * t_PRESC
        //
        // t_SYNC1 + t_SYNC2 > 4 * t_I2CCLK
        // t_SCL ~= t_SYNC1 + t_SYNC2 + t_SCLL + t_SCLH
        let ratio = $i2cclk / $freq;

        // For the standard-mode configuration method, we must have a ratio of 4
        // or higher
        assert!(
            ratio >= 4,
            "The I2C PCLK must be at least 4 times the bus frequency!"
        );

        let (presc_reg, scll, sclh, sdadel, scldel) = if $freq > 100_000 {
            // Fast-mode (Fm) or Fast-mode Plus (Fm+)
            // here we pick SCLL + 1 = 2 * (SCLH + 1)

            // Prescaler, 96 ticks for sclh/scll. Round up then subtract 1
            let presc_reg = ((ratio - 1) / 96) as u8;
            // ratio < 1200 by pclk 120MHz max., therefore presc < 16

            // Actual precale value selected
            let presc = (presc_reg + 1) as u32;

            let sclh = ((ratio / presc) - 3) / 3;
            let scll = (2 * (sclh + 1)) - 1;

            let (sdadel, scldel) = if $freq > 400_000 {
                // Fast-mode Plus (Fm+)
                assert!($i2cclk >= 17_000_000); // See table in datsheet

                let sdadel = $i2cclk / 8_000_000 / presc;
                let scldel = $i2cclk / 4_000_000 / presc - 1;

                (sdadel, scldel)
            } else {
                // Fast-mode (Fm)
                assert!($i2cclk >= 8_000_000); // See table in datsheet

                let sdadel = $i2cclk / 3_000_000 / presc;
                let scldel = $i2cclk / 1_000_000 / presc - 1;

                (sdadel, scldel)
            };

            (
                presc_reg,
                scll as u8,
                sclh as u8,
                sdadel as u8,
                scldel as u8,
            )
        } else {
            // Standard-mode (Sm)
            // here we pick SCLL = SCLH
            assert!($i2cclk >= 2_000_000); // See table in datsheet

            // Prescaler, 128 or 256 ticks for sclh/scll. Round up then
            // subtract 1
            let presc_reg = (ratio - 1)
                / if $freq < 8000 {
                    256
                } else if $freq < 80_000 {
                    128
                } else {
                    64
                };
            let presc_reg = cmp::min(presc_reg, 15) as u8;

            // Actual prescale value selected
            let presc = (presc_reg + 1) as u32;

            let sclh = ((ratio / presc) - 2) / 2;
            let scll = sclh;

            // Speed check
            assert!(
                sclh < 256,
                "The I2C PCLK is too fast for this bus frequency!"
            );

            let sdadel = $i2cclk / 2_000_000 / presc;
            let scldel = $i2cclk / 500_000 / presc - 1;

            (
                presc_reg,
                scll as u8,
                sclh as u8,
                sdadel as u8,
                scldel as u8,
            )
        };

        // Sanity check
        assert!(presc_reg < 16);

        // Keep values within reasonable limits for fast per_ck
        let sdadel = cmp::max(sdadel, 1);
        let scldel = cmp::max(scldel, 4);

        let sdadel = cmp::min(sdadel, 15);
        let scldel = cmp::min(scldel, 15);

        (presc_reg, scll, sclh, sdadel, scldel)
    }};
}

#[cfg(test)]
mod tests {
    use core::cmp;

    /// Runs a timing testcase over PCLK and I2C clock ranges
    fn i2c_timing_testcase<F>(f: F)
    where
        F: Fn(u32, u32),
    {
        let i2c_timing_tests = [
            // (i2c_clk, range of bus frequencies to test)
            (2_000_000, (4_000..=100_000)),  // Min PCLK
            (8_000_000, (4_000..=400_000)),  // Slowest PCLK for fast mode
            (16_000_000, (4_000..=400_000)), // Default H7 PCLK = 16MHz
            (32_000_000, (4_000..=1_000_000)),
            (79_876_135, (10_000..=1_000_000)),
            (100_000_000, (13_000..=1_000_000)),
            (120_000_000, (15_000..=1_000_000)), // Max PCLK
        ];

        for (clock, freq_range) in i2c_timing_tests.iter() {
            for freq in freq_range.clone().step_by(1_000) {
                f(*clock, freq)
            }
        }
    }

    #[test]
    /// Test the SCL frequency is within the expected range
    fn i2c_frequency() {
        i2c_timing_testcase(|i2c_clk: u32, freq: u32| {
            let (presc_reg, scll, sclh, _, _) = i2c_timing!(i2c_clk, freq);

            // Timing parameters
            let presc = (presc_reg + 1) as f32;
            let t_i2c_clk = 1. / (i2c_clk as f32);
            let freq = freq as f32;

            // Estimate minimum sync times. Analog filter on, 2 i2c_clk cycles
            let t_af_min = 50e-9_f32; // Analog filter 50ns. From H7 Datasheet
            let t_sync1 = t_af_min + 2. * t_i2c_clk;
            let t_sync2 = t_af_min + 2. * t_i2c_clk;

            // See RM0433 Rev 7 Section 47.4.9
            let t_high_low = sclh as f32 + 1. + scll as f32 + 1.;
            let t_scl = t_sync1 + t_sync2 + (t_high_low * presc * t_i2c_clk);
            let f_scl = 1. / t_scl;

            let error = (freq - f_scl) / freq;
            println!(
                "Set SCL = {} Actual = {} Error {:.1}%",
                freq,
                f_scl,
                100. * error
            );

            // We must generate a bus frequency less than or equal to that
            // specified. Tolerate a 2% error
            assert!(f_scl <= 1.02 * freq);

            // But it should not be too much less than specified
            assert!(f_scl > 0.8 * freq);
        });
    }

    #[test]
    /// Test that the low period of SCL is greater than the minimum specification
    fn i2c_scl_low() {
        i2c_timing_testcase(|i2c_clk: u32, freq: u32| {
            let (presc_reg, scll, _, _, _) = i2c_timing!(i2c_clk, freq);

            // Timing parameters
            let presc = (presc_reg + 1) as f32;
            let t_i2c_clk = 1. / (i2c_clk as f32);
            let freq = freq as f32;
            let t_scll = (scll as f32 + 1.) * presc * t_i2c_clk;

            // From I2C Specification Table 10
            //
            // UM10204 rev 6.: https://www.nxp.com/docs/en/user-guide/UM10204.pdf
            let t_scll_minimum = match freq {
                x if x <= 100_000. => 4.7e-6, // Standard mode (Sm)
                x if x <= 400_000. => 1.3e-6, // Fast mode (Fm)
                _ => 0.5e-6,                  // Fast mode Plus (Fm+)
            };

            println!("Target {} Hz; SCLL {}", freq, scll);
            println!("T SCL LOW {:.2e}; MINIMUM {:.2e}", t_scll, t_scll_minimum);
            assert!(t_scll >= t_scll_minimum);
        });
    }

    #[test]
    /// Test the SDADEL value is greater than the minimum specification
    fn i2c_sdadel_minimum() {
        i2c_timing_testcase(|i2c_clk: u32, freq: u32| {
            let (presc_reg, _, _, sdadel, _) = i2c_timing!(i2c_clk, freq);

            // Timing parameters
            let presc = (presc_reg + 1) as f32;
            let t_i2c_clk = 1. / (i2c_clk as f32);
            let freq = freq as f32;
            let t_sdadel = (sdadel as f32) * presc * t_i2c_clk;

            // From I2C Specification Table 10
            //
            // UM10204 rev 6.: https://www.nxp.com/docs/en/user-guide/UM10204.pdf
            let t_fall_max = match freq {
                x if x <= 100_000. => 300e-9, // Standard mode (Sm)
                x if x <= 400_000. => 300e-9, // Fast mode (Fm)
                _ => 120e-9,                  // Fast mode Plus (Fm+)
            };

            let t_af_min = 50e-9_f32; // Analog filter min 50ns. From H7 Datasheet
            let hddat_min = 0.;

            // From RM0433 Rev 7 Section 47.4.5
            //
            // tSDADEL >= {tf + tHD;DAT(min) - tAF(min) - [(DNF + 3) x tI2CCLK]}
            let t_sdadel_minimim = t_fall_max + hddat_min - t_af_min - (3. * t_i2c_clk);

            println!("Target {} Hz; SDADEL {}", freq, sdadel);
            println!(
                "T SDA DELAY {:.2e} MINIMUM {:.2e}",
                t_sdadel, t_sdadel_minimim
            );
            assert!(sdadel <= 15);
            assert!(t_sdadel >= t_sdadel_minimim);
        });
    }

    #[test]
    /// Test the SDADEL value is less than the maximum specification
    fn i2c_sdadel_maximum() {
        i2c_timing_testcase(|i2c_clk: u32, freq: u32| {
            let (presc_reg, _, _, sdadel, _) = i2c_timing!(i2c_clk, freq);

            // Timing parameters
            let presc = (presc_reg + 1) as f32;
            let t_i2c_clk = 1. / (i2c_clk as f32);
            let freq = freq as f32;
            let t_sdadel = (sdadel as f32) * presc * t_i2c_clk;

            let t_hddat_max = match freq {
                x if x <= 100_000. => 3.45e-6, // Standard mode (Sm)
                x if x <= 400_000. => 0.9e-6,  // Fast mode (Fm)
                _ => 0.45e-6,                  // Fast mode Plus (Fm+)
            };
            let t_af_max = 80e-9_f32; // Analog filter max 80ns. From H7 Datasheet

            // From RM0433 Rev 7 Section 47.4.5
            //
            // tSDADEL <= {tHD;DAT(max) - tAF(max) - [(DNF + 4) x tI2CCLK]}
            let t_sdadel_maximum = t_hddat_max - t_af_max - (4. * t_i2c_clk);

            println!("Target {} Hz; SDADEL {}", freq, sdadel);
            println!(
                "T SDA DELAY {:.2e} MAXIMUM {:.2e}",
                t_sdadel, t_sdadel_maximum
            );
            assert!(sdadel <= 15);
            assert!(t_sdadel <= t_sdadel_maximum);
        });
    }

    #[test]
    /// Test the SCLDEL value is greater than the minimum specification
    fn i2c_scldel_minimum() {
        i2c_timing_testcase(|i2c_clk: u32, freq: u32| {
            let (presc_reg, _, _, _, scldel) = i2c_timing!(i2c_clk, freq);

            // Timing parameters
            let presc = (presc_reg + 1) as f32;
            let t_i2c_clk = 1. / (i2c_clk as f32);
            let freq = freq as f32;
            let t_scldel = (scldel as f32) * presc * t_i2c_clk;

            // From I2C Specification Table 10
            //
            // UM10204 rev 6.: https://www.nxp.com/docs/en/user-guide/UM10204.pdf
            let t_rise_max = match freq {
                x if x <= 100_000. => 1000e-9, // Standard mode (Sm)
                x if x <= 400_000. => 300e-9,  // Fast mode (Fm)
                _ => 120e-9,                   // Fast mode Plus (Fm+)
            };
            let t_sudat_min = match freq {
                x if x <= 100_000. => 250e-9, // Standard mode (Sm)
                x if x <= 400_000. => 100e-9, // Fast mode (Fm)
                _ => 50e-9,                   // Fast mode Plus (Fm+)
            };

            // From RM0433 Rev 7 Section 47.4.5
            //
            // tSCLDEL >= tr + tSU;DAT(min)
            let t_scldel_minimum = t_rise_max + t_sudat_min;

            println!("Target {} Hz; SCLDEL {}", freq, scldel);
            println!(
                "T SCL DELAY {:.2e} MINIMUM {:.2e}",
                t_scldel, t_scldel_minimum
            );
            assert!(scldel <= 15);
            assert!(t_scldel >= t_scldel_minimum);
        });
    }
}
