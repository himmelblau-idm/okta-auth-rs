mod emit;
mod frontend;
mod profile;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    path::{Path, PathBuf},
};
pub type Result<T> = std::result::Result<T, String>;
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema: u32,
    #[serde(default)]
    pub native_profile: bool,
    pub upstream_version: String,
    pub upstream_commit: String,
    pub source_tree_sha256: String,
    pub entries: Vec<Entry>,
    #[serde(default)]
    pub constants: std::collections::BTreeMap<String, Value>,
}
#[derive(Deserialize, Serialize)]
pub struct Entry {
    pub module: String,
    pub symbol: String,
    #[serde(default)]
    pub export_as: Option<String>,
}
impl Entry {
    pub fn public_name(&self) -> &str {
        self.export_as.as_deref().unwrap_or(&self.symbol)
    }
}
pub fn hash(data: &[u8]) -> String {
    format!("{:x}", Sha256::digest(data))
}
fn source_tree_hash(root: &Path) -> Result<String> {
    fn collect(root: &Path, path: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        let mut entries = fs::read_dir(path)
            .map_err(|e| e.to_string())?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let kind = entry.file_type().map_err(|e| e.to_string())?;
            let path = entry.path();
            if kind.is_symlink() {
                return Err(format!("source tree contains symlink: {}", path.display()));
            }
            if kind.is_dir() {
                collect(root, &path, files)?;
            } else if kind.is_file() {
                files.push(
                    path.strip_prefix(root)
                        .map_err(|e| e.to_string())?
                        .to_path_buf(),
                );
            }
        }
        Ok(())
    }
    let root = root.canonicalize().map_err(|e| e.to_string())?;
    let mut files = vec![];
    collect(&root, &root, &mut files)?;
    files.sort();
    let mut digest = Sha256::new();
    for relative in files {
        let name = relative.to_string_lossy().replace('\\', "/");
        digest.update(name.as_bytes());
        digest.update([0]);
        digest.update(fs::read(root.join(&relative)).map_err(|e| e.to_string())?);
        digest.update([0]);
    }
    Ok(format!("{:x}", digest.finalize()))
}
fn run() -> Result<()> {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or(
        "usage: okta-translator <source-hash|inspect|generate|check> --source DIR --manifest FILE --output DIR",
    )?;
    let mut source = PathBuf::from("vendor/okta-auth-js");
    let mut manifest_path = PathBuf::from("translation.json");
    let mut output = PathBuf::from("crates/okta/src/generated");
    while let Some(flag) = args.next() {
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag.as_str() {
            "--source" => source = value.into(),
            "--manifest" => manifest_path = value.into(),
            "--output" => output = value.into(),
            _ => return Err(format!("unknown argument {flag}")),
        }
    }
    if command == "source-hash" {
        println!("{}", source_tree_hash(&source)?);
        return Ok(());
    }
    if !["inspect", "generate", "check"].contains(&command.as_str()) {
        return Err(format!("unknown command {command}"));
    }
    let manifest_bytes = fs::read(&manifest_path).map_err(|e| e.to_string())?;
    let manifest: Manifest = serde_json::from_slice(&manifest_bytes).map_err(|e| e.to_string())?;
    if manifest.schema != 1 {
        return Err("unsupported manifest schema".into());
    }
    let package: Value =
        serde_json::from_slice(&fs::read(source.join("package.json")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if package["version"] != manifest.upstream_version {
        return Err("vendored package version does not match translation manifest".into());
    }
    let actual_tree = source_tree_hash(&source)?;
    if actual_tree != manifest.source_tree_sha256 {
        return Err(format!(
            "vendored source tree hash mismatch: expected {}, got {actual_tree}",
            manifest.source_tree_sha256
        ));
    }
    let mut graph = frontend::Graph::new(&source)?;
    graph.native_profile = manifest.native_profile;
    for entry in &manifest.entries {
        graph.select(&entry.module, &entry.symbol)?;
        let binding = graph.binding(&entry.module, &entry.symbol)?;
        if binding != (entry.module.clone(), entry.symbol.clone()) {
            graph
                .links
                .insert((entry.module.clone(), entry.symbol.clone()), binding);
        }
    }
    let inventory = graph.inventory(&manifest, &hash(&manifest_bytes));
    if command == "inspect" {
        println!(
            "{}",
            serde_json::to_string_pretty(&inventory).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    graph.validate()?;
    if !manifest.constants.is_empty() {
        return Err(
            "manifest constants are not implemented; use the documented native profile".into(),
        );
    }
    let mut public_names = std::collections::BTreeSet::new();
    for entry in &manifest.entries {
        if !public_names.insert(entry.public_name()) {
            return Err(format!("duplicate exported name {}", entry.public_name()));
        }
    }
    let rust = emit::generate(&graph, &manifest)?;
    let mut report = inventory;
    report["compiler_sha256"] = hash(
        concat!(
            include_str!("main.rs"),
            include_str!("frontend.rs"),
            include_str!("emit.rs"),
            include_str!("profile.rs"),
            include_str!("../Cargo.toml")
        )
        .as_bytes(),
    )
    .into();
    report["native_support_sha256"] = hash(
        concat!(
            include_str!("../../runtime/src/value.rs"),
            include_str!("../../runtime/src/native.rs"),
            include_str!("../../runtime/src/http.rs"),
            include_str!("../../runtime/Cargo.toml")
        )
        .as_bytes(),
    )
    .into();
    report["generated_sha256"] = hash(rust.as_bytes()).into();
    let report = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())? + "\n";
    if command == "check" {
        for (name, expected) in [("mod.rs", &rust), ("provenance.json", &report)] {
            let actual =
                fs::read_to_string(output.join(name)).map_err(|e| format!("{name}: {e}"))?;
            if actual != *expected {
                return Err(format!("generated output differs: {name}"));
            }
        }
        println!("generated output is reproducible");
    } else {
        fs::create_dir_all(&output).map_err(|e| e.to_string())?;
        atomic_write(&output.join("mod.rs"), rust.as_bytes())?;
        atomic_write(&output.join("provenance.json"), report.as_bytes())?;
        println!("generated {} declarations", graph.selected.len());
    }
    Ok(())
}
fn atomic_write(path: &Path, data: &[u8]) -> Result<()> {
    let temp = path.with_extension("tmp");
    fs::write(&temp, data).map_err(|e| e.to_string())?;
    fs::rename(&temp, path).map_err(|e| e.to_string())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
