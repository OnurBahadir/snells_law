const PI:f32=3.141592653;

pub fn  ref_angle(n:f32,angle1:f32,n2:f32)->f32{

    rad2deg((n*deg2rad(angle1).sin()/n2).asin())
}

fn rad2deg(x:f32)->f32{
    x*180.0/PI
}

fn deg2rad(x:f32)->f32{
    x*PI/180.0
}
