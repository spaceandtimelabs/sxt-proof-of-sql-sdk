use std::{env, fs, path::Path};

fn main() {
    let pwd = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env variable unset");

    let proto_path = Path::new(&pwd).join("proto/".to_owned());

    let proto_defs = fs::read_dir(&proto_path)
        .unwrap()
        .map(|v| proto_path.join(v.expect("read dir must success").path()))
        .collect::<Vec<_>>();

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .type_attribute(".", "#[allow(clippy::all)]")
        .compile(&proto_defs, &[&proto_path])
        .unwrap();
}
