
#[derive(PartialEq, Clone)]
pub struct Infra {
    pub arms : Vec<Arm>,
}


#[derive(PartialEq, Clone)]
pub struct Arm {
    pub length: f32,
}
