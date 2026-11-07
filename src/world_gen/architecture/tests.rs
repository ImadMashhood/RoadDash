use super::*;
use crate::track::Track;
fn plan() -> Architecture {
    let track = Track::from_json(include_str!("../../../content/tracks/valley.json")).unwrap();
    let world = WorldPlan::from_json(
        super::super::WORLD_JSON,
        super::super::ARCHETYPES_JSON,
        &track,
    )
    .unwrap();
    let settings = super::super::layout::Construction::from_json(
        super::super::layout::CONSTRUCTION_JSON,
        &track,
    )
    .unwrap();
    let layout = WorldLayout::generate(&world, &track, settings).unwrap();
    Architecture::new(&world, &layout).unwrap()
}
#[test]
fn architecture_preserves_every_lot_identity_and_geometry_stays_in_its_envelope() {
    let p = plan();
    let again = plan();
    assert_eq!(p.buildings.len(), again.buildings.len());
    assert!(p.buildings.len() > 247);
    let mut repeats = 0;
    for (b, other) in p.buildings.iter().zip(&again.buildings) {
        assert_eq!(b.id, other.id);
        assert_eq!(b.design, other.design);
        let d = &p.catalog.records[b.design];
        let panels = b.panels(d, 15);
        assert!(!panels.is_empty());
        if panels.iter().filter(|p| p.band == 1).count() > 4 {
            repeats += 1;
        }
        for panel in panels {
            assert!(panel.transform.scale.is_finite() && panel.transform.scale.min_element() > 0.0);
            for lod in &d.lods {
                let bounds = &lod.bounds[panel.band];
                for x in [bounds.min[0], bounds.max[0]] {
                    for y in [bounds.min[1], bounds.max[1]] {
                        for z in [bounds.min[2], bounds.max[2]] {
                            let v = panel.transform.transform_point(Vec3::new(x, y, z));
                            assert!(
                                v.x.abs() <= b.size.x * 0.5 + 0.05
                                    && v.z.abs() <= b.size.z * 0.5 + 0.05,
                                "outside lot {} {} {v:?} size={:?}",
                                b.id,
                                d.id,
                                b.size
                            );
                            assert!(
                                v.y >= -0.01 && v.y <= b.size.y + 0.05,
                                "outside height {} {} {v:?}",
                                b.id,
                                d.id
                            );
                        }
                    }
                }
            }
        }
    }
    assert!(repeats > 20, "tall buildings need actual repeated storeys");
}
#[test]
fn grammar_rejects_unknown_models_and_unbounded_streaming() {
    let c = Catalog::from_json(CATALOG).unwrap();
    let mut r: serde_json::Value = serde_json::from_str(RULES).unwrap();
    r["rules"]["F01"] = serde_json::json!(["missing"]);
    assert!(Rules::from_json(&r.to_string(), &c).is_err());
    r = serde_json::from_str(RULES).unwrap();
    r["max_active_panels"] = 100000.into();
    assert!(Rules::from_json(&r.to_string(), &c).is_err());
    assert_eq!(c.records.len(), 128);
}

#[test]
fn catalog_rejects_unsafe_paths_bad_hashes_and_inverted_bounds() {
    let value: serde_json::Value = serde_json::from_str(CATALOG).unwrap();
    for (path, replacement) in [
        ("/records/0/id", serde_json::json!("../escape")),
        ("/records/0/masonry", serde_json::json!("")),
        ("/records/0/width", serde_json::json!(-1.0)),
        ("/records/0/lods/0/sha256", serde_json::json!("unverified")),
        (
            "/records/0/lods/0/bounds/0/min/0",
            serde_json::json!(10000.0),
        ),
    ] {
        let mut bad = value.clone();
        *bad.pointer_mut(path).unwrap() = replacement;
        assert!(
            Catalog::from_json(&bad.to_string()).is_err(),
            "accepted {path}"
        );
    }
}
