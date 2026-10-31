fn main() {
    road_dash::diagnostics::install_panic_report();
    if std::env::args().any(|a| a == "--verify-panic-report") {
        panic!("intentional panic-report verification");
    }
    let args: Vec<_> = std::env::args().collect();
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
                Some(road_dash::world_gen::report::headless_baseline(
                    road_dash::track::Track::from_definition(track.definition.clone())?,
                ))
            } else {
                None
            };
            serde_json::to_string_pretty(&road_dash::world_gen::report::document(
                &plan,
                &track,
                baseline.as_ref(),
            ))
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
    road_dash::build_app(
        std::env::args().any(|a| a == "--smoke-test" || a == "--actor-smoke-test"),
    )
    .run();
}
