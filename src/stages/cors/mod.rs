use actix_cors::Cors;

/// Returns cors setup
pub fn stage(methods: &Vec<String>) -> Cors {
    // Set bindings
    let bindings = methods.to_owned();
    let methods:Vec<&str> = bindings.iter().map(|s| s.as_str()).collect();

    // Return cors
    Cors::default()
        .allow_any_origin()
        .allowed_methods(methods)
        .allow_any_header()
        .max_age(3600)
}