// Example custom build script.
fn main() {
	// Tell Cargo that if the given file changes, to rerun this build script.
	println!("cargo::rerun-if-changed=java_src/com/me/Test.java");
	// Call `mkdir -p target/classes` to create the output directory
	std::fs::create_dir_all("target/classes").unwrap();
	// Compile the Java class `Test.java` to a `.class` file in the output directory
	std::process::Command::new("javac")
		.args(&["-d", "target/classes", "java_src/com/me/Test.java"])
		.output()
		.unwrap();
}