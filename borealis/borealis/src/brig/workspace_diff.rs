use {
    crate::brig::{tokens_to_string, Crate, Workspace},
    cargo_util_schemas::manifest::{
        InheritableDependency, InheritableField, PackageName, TomlDependency,
        TomlDetailedDependency, TomlManifest, TomlPackage, TomlWorkspace,
    },
    common::HashMap,
    semver::{BuildMetadata, Prerelease, Version},
    std::{
        collections::BTreeMap,
        fs::{self, read_to_string},
        path::{Path, PathBuf},
    },
    walkdir::WalkDir,
};

pub fn write_workspace<P: AsRef<Path>>(workspace: &Workspace, path: P) {
    let target_fs = build_fs(workspace);
    let current_fs = read_fs(path.as_ref());

    write_difference(target_fs, current_fs, path.as_ref());
}

// todo: check for extra files that need deleting
// todo: make folders?
fn write_difference(
    target: HashMap<PathBuf, String>,
    current: HashMap<PathBuf, String>,
    path: &Path,
) {
    for (k, v) in target {
        if let Some(contents) = current.get(&k) {
            if *contents != v {
                println!("diff @ {k:?}");
                fs::write(path.join(k), v).unwrap();
            }
        } else {
            todo!()
        }
    }
}

fn read_fs(path: &Path) -> HashMap<PathBuf, String> {
    WalkDir::new(path)
        .into_iter()
        .filter(|entry| entry.as_ref().unwrap().file_type().is_file())
        .map(|entry| {
            let entry = entry.unwrap();
            let adjusted_path = entry.path().strip_prefix(path).unwrap().to_owned();
            (adjusted_path, read_to_string(entry.path()).unwrap())
        })
        .collect()
}

fn build_fs(workspace: &Workspace) -> HashMap<PathBuf, String> {
    let workspace_manifest = (
        PathBuf::from("Cargo.toml"),
        toml::to_string_pretty(&TomlManifest {
            cargo_features: None,
            package: None,
            project: None,
            profile: None,
            lib: None,
            bin: None,
            example: None,
            test: None,
            bench: None,
            dependencies: None,
            dev_dependencies: None,
            dev_dependencies2: None,
            build_dependencies: None,
            build_dependencies2: None,
            features: None,
            target: None,
            replace: None,
            patch: None,
            workspace: Some(TomlWorkspace {
                members: Some(workspace.crates.keys().map(ToString::to_string).collect()),
                resolver: Some("2".to_owned()),
                exclude: None,
                default_members: None,
                metadata: None,
                package: None,
                dependencies: None,
                lints: None,
            }),
            badges: None,
            lints: None,
        })
        .unwrap(),
    );

    workspace
        .crates
        .iter()
        .flat_map(
            |(
                name,
                Crate {
                    dependencies,
                    contents,
                },
            )| {
                let manifest = (
                    PathBuf::from(name.as_ref()).join("Cargo.toml"),
                    toml::to_string_pretty(&TomlManifest {
                        cargo_features: None,
                        package: Some(Box::new(TomlPackage {
                            edition: Some(InheritableField::Value("2021".to_owned())),
                            rust_version: None,
                            name: PackageName::new(name.as_ref().to_owned()).unwrap(),
                            version: Some(InheritableField::Value(Version {
                                major: 0,
                                minor: 0,
                                patch: 0,
                                pre: Prerelease::EMPTY,
                                build: BuildMetadata::EMPTY,
                            })),
                            authors: None,
                            build: None,
                            metabuild: None,
                            default_target: None,
                            forced_target: None,
                            links: None,
                            exclude: None,
                            include: None,
                            publish: None,
                            workspace: None,
                            im_a_teapot: None,
                            autobins: None,
                            autoexamples: None,
                            autotests: None,
                            autobenches: None,
                            default_run: None,
                            description: None,
                            homepage: None,
                            documentation: None,
                            readme: None,
                            keywords: None,
                            categories: None,
                            license: None,
                            license_file: None,
                            repository: None,
                            resolver: None,
                            metadata: None,
                            _invalid_cargo_features: None,
                        })),
                        project: None,
                        profile: None,
                        lib: None,
                        bin: None,
                        example: None,
                        test: None,
                        bench: None,
                        dependencies: Some(
                            dependencies
                                .iter()
                                .map(|n| (n, PackageName::new(n.to_string()).unwrap()))
                                .map(|(n, package_name)| {
                                    (
                                        package_name,
                                        InheritableDependency::Value(TomlDependency::Detailed(
                                            TomlDetailedDependency {
                                                version: None,
                                                registry: None,
                                                registry_index: None,
                                                path: Some(format!("../{n}")),
                                                git: None,
                                                branch: None,
                                                tag: None,
                                                rev: None,
                                                features: None,
                                                optional: None,
                                                default_features: None,
                                                default_features2: None,
                                                package: None,
                                                public: None,
                                                artifact: None,
                                                lib: None,
                                                target: None,
                                                _unused_keys: BTreeMap::new(),
                                            },
                                        )),
                                    )
                                })
                                .collect(),
                        ),
                        dev_dependencies: None,
                        dev_dependencies2: None,
                        build_dependencies: None,
                        build_dependencies2: None,
                        features: None,
                        target: None,
                        replace: None,
                        patch: None,
                        workspace: None,
                        badges: None,
                        lints: None,
                    })
                    .unwrap(),
                );

                let source = (
                    PathBuf::from(name.as_ref()).join("src").join("lib.rs"),
                    tokens_to_string(contents),
                );

                [manifest, source].into_iter()
            },
        )
        .chain([workspace_manifest])
        .collect()
}
