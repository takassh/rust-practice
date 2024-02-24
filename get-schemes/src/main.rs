mod cli;

use std::process::Command;

use clap::Parser;
use cli::Args;

use get_schemes::{
    android,
    ios::XcodeProjectInfo,
};

fn main() {
    let args = Args::parse();
    let flutter_project_dir = args.path;
    {
        let args = vec!["-list"];
        let output = Command::new("xcodebuild")
            .current_dir(format!("{}/ios", flutter_project_dir))
            .args(args)
            .output()
            .expect("failed to execute process");

        let output = String::from_utf8(output.stdout).expect("failed to convert bytes to String");

        let info = XcodeProjectInfo::new_from_xcode_build_output(&output);

        println!("targets:{:#?}", info.targets);
        println!("build_configurations:{:#?}", info.build_configurations);
        println!("schemes:{:#?}", info.schemes);

        let selected_scheme = info.scheme_for("develop");

        println!(
            "selected scheme:{:#?}",
            selected_scheme.expect("failed to select scheme")
        );
    }

    {
        let args = vec!["app:tasks", "--all", "--console=auto"];
        let output = Command::new("./gradlew")
            .env(
                "JAVA_HOME",
                "/Applications/Android Studio.app/Contents/jre/Contents/Home",
            )
            .current_dir(format!("{}/android", flutter_project_dir))
            .args(args)
            .output()
            .expect("failed to execute process");
        let output = String::from_utf8(output.stdout).expect("failed to convert bytes to String");

        let schemes = android::get_schemes(&output);

        println!("{:#?}", schemes);
    }
}
