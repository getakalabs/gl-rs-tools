use handlebars::Handlebars as HBS;

// Struct container for handlebars options
struct Options {
    pub asset_path: String,
    pub extension: String,
}

// Default implementation for options
impl Default for Options {
    fn default() -> Self {
        Self {
            asset_path: String::from("./assets/templates"),
            extension: String::from(".hbs"),
        }
    }
}

// Create handlebar implementations
impl Options {
    // Create new with options
    #[allow(dead_code)]
    pub fn new_options<AP, E>(asset_path: AP, extension: E) -> Self
        where AP: ToString,
              E: ToString
    {
        // Create bindings
        let asset_path_bindings = asset_path.to_string();
        let extension_bindings = extension.to_string();

        // Return with new values
        Self {
            asset_path: asset_path_bindings,
            extension: extension_bindings,
        }
    }
}

// Stage handlebar instance
pub fn stage() -> HBS<'static> {
    // Initialize handlebars
    let mut handlebars = HBS::new();
    let options = Options::default();

    // Register directories
    handlebars
        .register_templates_directory(&options.extension, &options.asset_path)
        .expect("Invalid template directory");

    // Return handlebars
    handlebars
}