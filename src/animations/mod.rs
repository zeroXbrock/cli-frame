use self::{camera::CAMERA, globe::GLOBE, loading::LOADING};

mod camera;
mod globe;
mod loading;

pub enum Animation {
    Globe,
    Camera,
    Loading,
}

impl Animation {
    pub fn frames(&self) -> Vec<&'static str> {
        match self {
            Animation::Camera => CAMERA.to_vec(),
            Animation::Globe => GLOBE.to_vec(),
            Animation::Loading => LOADING.to_vec(),
        }
    }
}
