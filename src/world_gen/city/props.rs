//! Sidewalk furnishing sockets outside roadways, intersections and pedestrian patrol lanes.
use bevy::prelude::*;
use rr_content::city::CitySettings;
pub struct Socket {
    pub builder: &'static str,
    pub position: Vec3,
    pub rotation: Quat,
    pub size: Vec3,
    pub seed: usize,
}
pub fn sockets(c: &CitySettings) -> Vec<Socket> {
    let mut out = Vec::new();
    for block in 0..(c.length_m / c.block_pitch_m).floor() as usize {
        for side in [-1.0, 1.0] {
            // Keep the established x=±8.9 pedestrian lane clear. Fixtures hug the building side.
            let x = side * (c.road_width_m / 2.0 + c.sidewalk_width_m - 0.45);
            let rotation = Quat::from_rotation_y(-side * std::f32::consts::FRAC_PI_2);
            let extra = [
                "hydrant",
                "parking_meter",
                "mailbox",
                "news_boxes",
                "payphone_pair",
                "information_board",
                "traffic_sign",
                "recycling_pair",
            ][block % 8];
            for (builder, offset, size) in [
                ("bench", 48.0, Vec3::new(1.85, 1.05, 0.45)),
                ("street_bin", 76.0, Vec3::new(0.55, 1.0, 0.55)),
                (
                    extra,
                    24.0,
                    if matches!(
                        extra,
                        "traffic_sign" | "information_board" | "payphone_pair"
                    ) {
                        Vec3::new(0.8, 2.2, 0.45)
                    } else {
                        Vec3::new(0.6, 1.0, 0.45)
                    },
                ),
            ] {
                let z = block as f32 * c.block_pitch_m + offset;
                let cross = z
                    .rem_euclid(c.block_pitch_m)
                    .min(c.block_pitch_m - z.rem_euclid(c.block_pitch_m));
                if cross > c.cross_street_width_m / 2.0 + c.sidewalk_width_m + size.x / 2.0 {
                    out.push(Socket {
                        builder,
                        position: Vec3::new(x, 0.16, z),
                        rotation,
                        size,
                        seed: block,
                    });
                }
            }
            if block % 4 == 0 {
                out.push(Socket {
                    builder: ["sedan", "van", "pickup", "car_hulk"][block / 4 % 4],
                    position: Vec3::new(
                        side * (c.parallel_road_offset_m + c.parallel_road_width_m / 2.0 - 1.3),
                        0.025,
                        block as f32 * c.block_pitch_m + 50.0,
                    ),
                    rotation: Quat::from_rotation_y(if side > 0.0 {
                        0.0
                    } else {
                        std::f32::consts::PI
                    }),
                    size: Vec3::new(1.85, 1.65, 3.8),
                    seed: block,
                });
            }
        }
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_furnishing_preserves_the_road_crossings_and_pedestrian_lane() {
        let city = rr_content::city::CityPlan::from_json(super::super::CITY_JSON).unwrap();
        let c = &city.config;
        let sockets = sockets(c);
        assert_eq!(sockets.len(), 346);
        for s in sockets {
            if matches!(s.builder, "sedan" | "van" | "pickup" | "car_hulk") {
                assert!(s.position.x.abs() - s.size.x / 2.0 > c.parallel_road_offset_m);
                assert!(
                    s.position.x.abs() + s.size.x / 2.0
                        < c.parallel_road_offset_m + c.parallel_road_width_m / 2.0
                );
                continue;
            }
            let half_x = s.size.z / 2.0;
            assert!(s.position.x.abs() - half_x > c.road_width_m / 2.0);
            assert!(s.position.x.abs() + half_x < c.road_width_m / 2.0 + c.sidewalk_width_m);
            assert!(s.position.x.abs() - half_x > 8.9 + 0.27);
            let cross = s
                .position
                .z
                .rem_euclid(c.block_pitch_m)
                .min(c.block_pitch_m - s.position.z.rem_euclid(c.block_pitch_m));
            assert!(cross - s.size.x / 2.0 > c.cross_street_width_m / 2.0 + c.sidewalk_width_m);
        }
    }
}
