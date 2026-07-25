use singularity_econ::{simulate, GeoShock, Params, ShockKind};
fn main() {
    let b = simulate(&Params::default());
    let s8 = simulate(&Params {
        geo_shocks: vec![GeoShock { kind: ShockKind::EnergyChokepoint, start_year: 2028, duration_years: 0.75 }],
        ..Params::default()
    });
    for y in [2028, 2029, 2030] {
        let a = b.iter().find(|s| s.year == y).unwrap();
        let s = s8.iter().find(|s| s.year == y).unwrap();
        println!("{y}: chip_util base={:.6} s8={:.6} diff={:.2e} | sil_margin base={:.6} s8={:.6} diff={:.2e} | binding base={:?} s8={:?}",
            a.chip_utilization, s.chip_utilization, (a.chip_utilization - s.chip_utilization).abs(),
            a.silicon_margin, s.silicon_margin, (a.silicon_margin - s.silicon_margin).abs(),
            a.binding, s.binding);
    }
    // does binding ever become Chips on s8 path?
    for s in &s8 {
        if format!("{:?}", s.binding) == "Chips" { println!("S8 path binds Chips in {}", s.year); }
    }
    for s in &b {
        if format!("{:?}", s.binding) == "Chips" { println!("BASE path binds Chips in {}", s.year); }
    }
}
