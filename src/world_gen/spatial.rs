//! Oriented XZ rectangles shared by placement, navigation and diagnostics.
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rect {
    pub center: [f32; 2],
    pub right: [f32; 2],
    pub half: [f32; 2],
}
impl Rect {
    pub fn new(center: Vec3, rotation: Quat, half: Vec2) -> Self {
        let right = (rotation * Vec3::X).xz().normalize();
        Self {
            center: center.xz().to_array(),
            right: right.to_array(),
            half: half.to_array(),
        }
    }
    pub fn axes(&self) -> [Vec2; 2] {
        let x = Vec2::from_array(self.right);
        [x, Vec2::new(-x.y, x.x)]
    }
    pub fn local(&self, point: Vec2) -> Vec2 {
        let d = point - Vec2::from_array(self.center);
        let [x, z] = self.axes();
        Vec2::new(d.dot(x), d.dot(z))
    }
    pub fn contains(&self, point: Vec2, margin: f32) -> bool {
        let p = self.local(point).abs();
        p.x <= self.half[0] + margin && p.y <= self.half[1] + margin
    }
    pub fn expanded(&self, margin: f32) -> Self {
        Self {
            half: [self.half[0] + margin, self.half[1] + margin],
            ..self.clone()
        }
    }
    pub fn corners(&self) -> [Vec2; 4] {
        let c = Vec2::from_array(self.center);
        let [x, z] = self.axes();
        let x = x * self.half[0];
        let z = z * self.half[1];
        [c - x - z, c + x - z, c + x + z, c - x + z]
    }
    pub fn overlaps(&self, other: &Self) -> bool {
        let delta = Vec2::from_array(other.center) - Vec2::from_array(self.center);
        let a = self.axes();
        let b = other.axes();
        a.into_iter().chain(b).all(|axis| {
            let ra = self.half[0] * a[0].dot(axis).abs() + self.half[1] * a[1].dot(axis).abs();
            let rb = other.half[0] * b[0].dot(axis).abs() + other.half[1] * b[1].dot(axis).abs();
            delta.dot(axis).abs() < ra + rb - 0.001
        })
    }
    pub fn blocks_segment(&self, from: Vec2, to: Vec2, margin: f32) -> bool {
        let from = self.local(from);
        let to = self.local(to);
        let delta = to - from;
        let mut lo: f32 = 0.0;
        let mut hi: f32 = 1.0;
        for axis in 0..2 {
            let half = self.half[axis] + margin;
            if delta[axis].abs() < 1e-6 {
                if from[axis].abs() > half {
                    return false;
                }
            } else {
                let a = (-half - from[axis]) / delta[axis];
                let b = (half - from[axis]) / delta[axis];
                lo = lo.max(a.min(b));
                hi = hi.min(a.max(b));
                if hi < lo {
                    return false;
                }
            }
        }
        hi >= 0.0 && lo <= 1.0
    }
}
