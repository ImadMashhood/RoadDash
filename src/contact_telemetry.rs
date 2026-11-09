//! Bounded adapter observations; never drives simulation or visual transforms.
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct VehicleContact {
    pub tick: u64,
    pub rider: Entity,
    pub object: Entity,
    pub incoming: Vec3,
    pub object_velocity: Vec3,
    pub normal: Vec3,
    pub closing_mps: f32,
    pub approach: rr_core::contact::VehicleApproach,
    /// Solid collider bounds, rather than visual mesh bounds.
    pub object_height_m: Option<f32>,
    pub interrupted_stagger: bool,
    pub crash: bool,
}

#[derive(Resource, Default)]
pub struct VehicleContacts(pub VecDeque<VehicleContact>);
impl VehicleContacts {
    pub fn record(&mut self, sample: VehicleContact) {
        // A retry rewinds the active race, but ticks remain monotonic. Each
        // record identifies its observation tick and both adapter entities.
        if self.0.len() == 128 {
            self.0.pop_front();
        }
        self.0.push_back(sample);
    }
}
