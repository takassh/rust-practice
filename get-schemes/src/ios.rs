#[cfg(test)]
mod test;

pub struct XcodeProjectInfo<'a> {
    pub targets: Vec<&'a str>,
    pub build_configurations: Vec<&'a str>,
    pub schemes: Vec<&'a str>,
}

impl<'a> XcodeProjectInfo<'_> {
    pub fn new_from_xcode_build_output(output: &'a str) -> XcodeProjectInfo {
        let mut targets = vec![];
        let mut build_configurations = vec![];
        let mut schemes = vec![];
        let mut collector = None;
        for line in output.split("\n") {
            if line.is_empty() {
                collector = None;
                continue;
            } else if line.ends_with("Targets:") {
                collector = Some(&mut targets);
                continue;
            } else if line.ends_with("Build Configurations:") {
                collector = Some(&mut build_configurations);
                continue;
            } else if line.ends_with("Schemes:") {
                collector = Some(&mut schemes);
                continue;
            }
            if let Some(ref mut c) = collector {
                c.push(line.trim());
            }
        }
        if schemes.is_empty() {
            schemes.push("Runner")
        }

        XcodeProjectInfo {
            targets: targets,
            build_configurations: build_configurations,
            schemes: schemes,
        }
    }

    pub fn scheme_for(&self, expected_scheme: &str) -> Option<String> {
        if self.schemes.contains(&expected_scheme) {
            return Some(expected_scheme.to_string());
        }

        let mut iter = self
            .schemes
            .iter()
            .filter(|x| x.to_lowercase() == expected_scheme.to_lowercase());
        let first = iter.next();
        if let None = first {
            return None;
        }
        if let Some(_) = iter.next() {
            return None;
        }

        Some(first.unwrap().to_string())
    }
}
