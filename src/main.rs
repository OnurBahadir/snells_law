mod snell;

fn main() {
    // Refractive index of Air 
    const N1:f32=1.000273; 

    // Refractive index Fused silica
    const N2:f32=1.458;     

    // angle of incidence
    let theta1=45.0 ; 


    let ref_angle = snell::ref_angle(N1, theta1, N2);

    println!("Angle of refraction : {}",ref_angle);


}
