//! "Musk case": every robotics parameter at its aggressive edge — Tesla's
//! own stated timeline (volume 2027, ~1M/yr capacity by YE2027), Chinese
//! component scaling, 30% learning rate, cheap builds.

use singularity_econ::{simulate, Params};

fn main() {
    let p = Params {
        robotics_year: 2027,               // volume starts a year early
        robot_prod_2028_m: 0.9,            // ~Musk's 1M/yr capacity claim, hit
        component_capacity_2028: 1.0,      // components somehow ready day one
        component_base_growth: 1.0,        // doubling organically
        component_supply_gain: 3.5,        // ferocious China supply response
        component_growth_ceiling: 2.5,     // 3.5x/yr max growth (unprecedented)
        bootstrap_gain: 0.3,               // robots build robot factories fast
        robot_learning_rate: 0.30,         // top-of-range Wright's law
        robot_cost_2028_k: 35.0,           // China BOM from day one
        robot_cost_floor_k: 6.0,
        ..Params::default()
    };
    println!("{:<6} {:>10} {:>10} {:>10} {:>9}",
             "year", "prod M/yr", "fleet M", "phys disp", "cost $k");
    for s in simulate(&p) {
        println!("{:<6} {:>10.2} {:>10.1} {:>9.1}% {:>9.1}",
                 s.year, s.robot_prod_m, s.robot_fleet_m,
                 s.phys_displacement * 100.0, s.robot_cost_k);
    }
}
