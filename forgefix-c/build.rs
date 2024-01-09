use cbindgen::ItemType;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let enum_config = cbindgen::EnumConfig {
        rename_variants: cbindgen::RenameRule::QualifiedScreamingSnakeCase,
        ..Default::default()
    };

    let parse_config = cbindgen::ParseConfig {
        parse_deps: true,
        ..Default::default()
    };

    let mut export_config = cbindgen::ExportConfig {
        rename: std::collections::HashMap::from([
            ("Tags".to_string(), "tags".to_string()),
            ("CFixError".to_string(), "c_fix_error".to_string()),
            ("MsgType".to_string(), "msg_type".to_string()),
            ("Side".to_string(), "side".to_string()),
            ("OrdType".to_string(), "ord_type".to_string()),
            ("TimeInForce".to_string(), "time_in_force".to_string()),
            ("OpenClose".to_string(), "open_close".to_string()),
        ]),
        ..Default::default()
    };

    let mut config = cbindgen::Config {
        enumeration: enum_config,
        parse: parse_config,
        export: export_config.clone(),
        ..Default::default()
    };

    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_config(config.clone())
        .exclude_item("Tags")
        .with_include("fix_fields.h")
        .with_parse_include(&["forgefix"])
        .with_language(cbindgen::Language::C)
        .with_include_guard("_FIX_H")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("fix.h");

    export_config.item_types = vec![ItemType::Enums];
    config.export = export_config;

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .exclude_item("CFixError")
        .include_item("MsgType")
        .include_item("Side")
        .include_item("OrdType")
        .include_item("TimeInForce")
        .include_item("OpenClose")
        .with_parse_include(&["forgefix"])
        .with_language(cbindgen::Language::C)
        .with_include_guard("_FIX_FIELDS_H")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("fix_fields.h");
}
