fn main() {
    road_dash::diagnostics::install_panic_report();
    if std::env::args().any(|a| a == "--verify-panic-report") {
        panic!("intentional panic-report verification");
    }
    let args: Vec<_> = std::env::args().collect();
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
    road_dash::build_app(std::env::args().any(|a| a == "--smoke-test")).run();
}
