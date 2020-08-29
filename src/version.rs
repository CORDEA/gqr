struct Version<'a> {
    key: &'a str,
    module: u32,
}

const VERSIONS: [Version; 1] = [Version {
    key: "m1",
    module: 11,
}];
