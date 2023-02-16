{
  description = "example of using r2db to simplify pooling and some borrowing.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          (import rust-overlay)
          (self: super: {
            rustToolchain =
              super.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          })
        ];

        rustVersion = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };
        pkgs = import nixpkgs { inherit system overlays; };
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustToolchain sccache ];

          shellHook = ''
            export RUSTC_WRAPPER=sccache
            export RUST_LOG=info
          '';
        };
      });
}
