mod display;
mod service;

use {VRService, VRServiceCreator};

pub struct MockServiceCreator;

impl MockServiceCreator {
    pub fn new() -> Box<VRServiceCreator> {
        Box::new(MockServiceCreator)
    }
}

impl VRServiceCreator for MockServiceCreator {
     fn new_service(&self) -> Box<VRService> {
         Box::new(service::MockVRService::new())
     }
}