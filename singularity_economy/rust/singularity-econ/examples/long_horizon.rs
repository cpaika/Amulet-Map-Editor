//! Long-horizon run to 2050. Calibration is 2026-2036; beyond that this
//! is STRUCTURE, not forecast (loop dynamics extrapolated, not fitted) —
//! but the model is stable and the long-dated mechanisms self-activate:
//! power rents normalize ~2040, IP tolls finally decay ~2046 as orbital
//! compute crosses its cost gate, the robot bootstrap (R2) goes vertical
//! 2038-2050, the Land autonomization trend plateaus ~0.40, and the
//! sovereign debt snowball becomes the dominant question (debt/GDP 4x).
use singularity_econ::{simulate, Params};
fn main() {
    let s = simulate(&Params { end_year: 2050, ..Params::default() });
    println!("year disp power ip silic auton long_r debt/gdp orbGWe fleet_M");
    for st in &s {
        println!("{} {:.2} {:.2} {:.2} {:.2} {:.2} {:.3} {:.2} {:.1} {:.0}",
            st.year, st.cog_displacement, st.power_margin, st.ip_toll_margin,
            st.silicon_margin, st.autonomization_index, st.long_rate,
            st.gov_debt_gdp, st.orbital_gw_equiv, st.robot_fleet_m);
    }
}
