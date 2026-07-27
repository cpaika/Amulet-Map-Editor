use singularity_econ::*;
fn main() {
    let v = simulate(&Params::default());
    let mut us=0f64; let mut cn=0f64; let mut eu=0f64;
    let mut cnf=0f64; let mut euf=0f64; let mut usf=0f64;
    for s in &v {
        us=us.max(s.bloc_stress[0]); cn=cn.max(s.bloc_stress[1]); eu=eu.max(s.bloc_stress[2]);
        usf=usf.max(s.bloc_fracture_risk[0]); cnf=cnf.max(s.bloc_fracture_risk[1]); euf=euf.max(s.bloc_fracture_risk[2]);
    }
    println!("peak stress  US={us:.3} CN={cn:.3} EU={eu:.3}");
    println!("peak fract   US={usf:.3} CN={cnf:.3} EU={euf:.3}");
    // last year
    let l=v.last().unwrap();
    println!("final stress US={:.3} CN={:.3} EU={:.3}", l.bloc_stress[0],l.bloc_stress[1],l.bloc_stress[2]);
    println!("final fract  US={:.3} CN={:.3} EU={:.3}", l.bloc_fracture_risk[0],l.bloc_fracture_risk[1],l.bloc_fracture_risk[2]);
}
