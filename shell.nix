{ pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
	nativeBuildInputs = with pkgs; [
		rustc
		cargo

		pkg-config
		gcc
		cmake
	];

	RUST_SRC_PATH = with pkgs; "${rustPlatform.rustLibSrc}";
}
