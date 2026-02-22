use crate::actor::MainMarker;

pub struct MainTask {
    pub(crate) f: Box<dyn Fn(MainMarker) -> ()>,
}
