fn main() {
	println!("cargo:rerun-if-changed=./windows/winmd/Microsoft.Win32.Graphics.Direct3D.Legacy.winmd");
	println!("cargo:rerun-if-changed=build.rs");

	windows_bindgen::bindgen([
		"--in",
		"default",
		".windows/winmd/Microsoft.Win32.Graphics.Direct3D.Legacy.winmd",
		"--out",
		"src/bindings.rs",
		"--filter",
		"Microsoft.Win32.Graphics.Direct3D.Legacy",
		"--reference",
		"windows,skip-root,Windows"
	]).unwrap();
}