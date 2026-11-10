fn main() {
    let args: Vec<_> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--online-test-node") {
        road_dash::online::verification::run(
            args.get(i + 1).is_some_and(|s| s == "host"),
            args.get(i + 2).expect("endpoint").clone(),
            args.get(i + 3).expect("report path").clone(),
        );
        return;
    }
    if let Some(i) = args.iter().position(|a| a == "--online-native-test") {
        road_dash::online::verification::run_native(
            args.get(i + 1).is_some_and(|s| s == "host"),
            args.get(i + 2).expect("endpoint").clone(),
            args.get(i + 3).expect("report path").clone(),
        );
        return;
    }
    let default_inspection = args.get(1).is_some_and(|a| a == "--inspect-world")
        && args.get(2).is_none_or(|a| a.starts_with("--"))
        && std::env::var("ROADDASH_TRACK").map_or(true, |v| v == "metro");
    if args.iter().any(|a| a == "--inspect-metropolis") || default_inspection {
        let t = road_dash::track::Track::metropolis().expect("validated metropolis");
        let m = t.metropolis.as_ref().unwrap();
        let report = serde_json::to_string_pretty(&serde_json::json!({"buildings":m.buildings.len(),"surfaces":m.surfaces.len(),"routes":2,"logical_length":t.length,"source_layout_sha256":m.source_layout_sha256})).unwrap();
        if let Some(i) = args.iter().position(|a| a == "--output") {
            std::fs::write(args.get(i + 1).expect("output path"), report).expect("write report");
        } else {
            println!("{report}");
        }
        return;
    }
    road_dash::diagnostics::install_panic_report();
    if std::env::args().any(|a| a == "--verify-panic-report") {
        panic!("intentional panic-report verification");
    }
    if args.iter().any(|a| a == "--inspect-city")
        || (args.get(1).is_some_and(|a| a == "--inspect-world")
            && args.get(2).is_none_or(|a| a.starts_with("--")))
    {
        let report = road_dash::world_gen::city::document().expect("validated city authoring");
        let json = serde_json::to_string_pretty(&report).expect("city report");
        if let Some(index) = args.iter().position(|a| a == "--output") {
            std::fs::write(args.get(index + 1).expect("report output path"), json)
                .expect("write city report");
        } else {
            println!("{json}");
        }
        return;
    }
    if args.iter().any(|a| a == "--original-crash-report") {
        println!(
            "{}",
            serde_json::to_string_pretty(&road_dash::original_crash::report())
                .expect("crash report")
        );
        return;
    }

    if let Err(error) = road_dash::biomechanics::Settings::selected() {
        eprintln!("Ragdoll configuration: {error}");
        std::process::exit(2);
    }
    if args.iter().any(|a| a == "--animation-lab-view") {
        road_dash::animation::preview::build(args.iter().any(|a| a == "--animation-lab-smoke"))
            .run();
        return;
    }
    if args.iter().any(|a| a == "--ragdoll-lab-view") {
        road_dash::biomechanics::view::build(args.iter().any(|a| a == "--ragdoll-view-smoke"))
            .run();
        return;
    }
    if args.iter().any(|a| a == "--ragdoll-lab") {
        let output = args
            .iter()
            .position(|a| a == "--output")
            .and_then(|i| args.get(i + 1))
            .map_or("ragdoll-lab.json", String::as_str);
        if let Err(error) = road_dash::biomechanics::lab::run(std::path::Path::new(output)) {
            eprintln!("Ragdoll lab: {error}");
            std::process::exit(2);
        }
        return;
    }
    if let Some(index) = args
        .iter()
        .position(|a| a == "--validate-world" || a == "--inspect-world")
    {
        let result = (|| -> Result<String, String> {
            let read = |offset| {
                args.get(index + offset).ok_or_else(|| "usage: --validate-world/--inspect-world WORLD_JSON ARCHETYPES_JSON TRACK_JSON [--output REPORT_JSON]".to_string()).and_then(|p| std::fs::read_to_string(p).map_err(|e| format!("{p}: {e}")))
            };
            let track = road_dash::track::Track::from_json(&read(3)?)?;
            let plan = road_dash::world_gen::WorldPlan::from_json(&read(1)?, &read(2)?, &track)?;
            let baseline = if args[index] == "--inspect-world" {
                Some(road_dash::world_gen::report::headless_for_plan(
                    road_dash::track::Track::from_definition(track.definition.clone())?,
                    plan.clone(),
                ))
            } else {
                None
            };
            serde_json::to_string_pretty(&road_dash::world_gen::report::document(
                &plan,
                &track,
                baseline.as_ref(),
            )?)
            .map_err(|e| e.to_string())
        })();
        let result = result.and_then(|json| {
            if let Some(output) = args.iter().position(|a| a == "--output") {
                let path = args
                    .get(output + 1)
                    .ok_or_else(|| "--output requires a report path".to_string())?;
                std::fs::write(path, json).map_err(|e| format!("{path}: {e}"))?;
                println!("Validated world; wrote {path}");
            } else {
                println!("{json}");
            }
            Ok(())
        });
        if let Err(error) = result {
            eprintln!("World validation failed: {error}");
            std::process::exit(2);
        }
        return;
    }
    if let Some(index) = args.iter().position(|a| a == "--validate-animation") {
        let result = args
            .get(index + 1)
            .ok_or_else(|| "usage: --validate-animation MANIFEST_JSON".to_string())
            .and_then(|path| {
                rr_content::animation::Catalog::validate_directory(std::path::Path::new(path))
            });
        match result {
            Ok(catalog) => println!(
                "Valid static animation authoring: {} poses, {} use cases; not connected to runtime",
                catalog.poses.len(),
                catalog.use_cases.len()
            ),
            Err(error) => {
                eprintln!("Animation validation failed: {error}");
                std::process::exit(2);
            }
        }
        return;
    }
    if let Some(index) = args.iter().position(|a| a == "--validate-handling") {
        let result = args
            .get(index + 1)
            .ok_or_else(|| "handling command needs JSON path".to_string())
            .and_then(|p| std::fs::read_to_string(p).map_err(|e| e.to_string()))
            .and_then(|json| road_dash::vehicle::Tuning::from_json(&json));
        match result {
            Ok(tuning) => println!("Valid handling: {} presets", tuning.presets.len()),
            Err(error) => {
                eprintln!("Handling validation failed: {error}");
                std::process::exit(2);
            }
        }
        return;
    }
    if let Some(index) = args
        .iter()
        .position(|a| a == "--inspect-track" || a == "--validate-track")
    {
        let result = args
            .get(index + 1)
            .ok_or_else(|| "track command needs JSON path".to_string())
            .and_then(|path| std::fs::read_to_string(path).map_err(|e| e.to_string()))
            .and_then(|json| road_dash::track::Track::from_json(&json));
        match result {
            Ok(track) => println!(
                "{}",
                serde_json::json!({"id":track.definition.id.as_str(),"closed":track.definition.closed,"length_m":track.length,"samples":track.points.len(),"elevation_min_m":track.bounds.0.y,"elevation_max_m":track.bounds.1.y,"gates":track.definition.checkpoints.len(),"spawns":track.definition.spawns.len(),"collision_step_m":2.5})
            ),
            Err(error) => {
                eprintln!("Track validation failed: {error}");
                std::process::exit(2);
            }
        }
        return;
    }
    if let Some(index) = args.iter().position(|a| a == "--validate-content") {
        let result = args
            .get(index + 1)
            .ok_or_else(|| "--validate-content needs a JSON path".to_string())
            .and_then(|path| std::fs::read_to_string(path).map_err(|e| e.to_string()))
            .and_then(|json| rr_content::Catalog::from_json(&json));
        match result {
            Ok(catalog) => {
                println!(
                    "Valid schema {}: {} bikes, {} riders",
                    catalog.schema_version,
                    catalog.bikes.len(),
                    catalog.riders.len()
                );
                return;
            }
            Err(error) => {
                eprintln!("Content validation failed: {error}");
                std::process::exit(2);
            }
        }
    }
    let status = road_dash::build_app(std::env::args().any(|a| {
        a == "--police-smoke"
            || a == "--frontend-smoke"
            || a == "--destruction-survey"
            || a == "--metro-survey"
            || a == "--smoke-test"
            || a == "--actor-smoke-test"
            || a == "--actor-skin-survey"
            || a == "--world-survey"
            || a == "--lod-survey"
    }))
    .run();
    // Native test fixtures report failed acceptance via AppExit. Preserve that
    // status for scripts rather than returning a misleading successful process.
    if let bevy::app::AppExit::Error(code) = status {
        std::process::exit(i32::from(code.get()));
    }
}
