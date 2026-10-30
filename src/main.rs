fn main() {
    road_dash::diagnostics::install_panic_report();
    if std::env::args().any(|a| a == "--verify-panic-report") {
        panic!("intentional panic-report verification");
    }
    let args: Vec<_> = std::env::args().collect();
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
