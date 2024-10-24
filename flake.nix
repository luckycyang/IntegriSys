{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , nixpkgs
    , rust-overlay
    , flake-utils
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustpkg = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = [ "rust-src" "rustfmt" "clippy" "rust-analyzer" ]; # rust-src for rust-analyzer
            targets = [ "x86_64-unknown-linux-gnu" ];
          });
      in
      with pkgs; {
        devShells = {
          default = mkShell {
            buildInputs = [
              openssl
              pkg-config
              vulkan-tools
              eza
              fd
              rustpkg
              (python313.withPackages (ps: with ps;[
                pillow
              ]))
            ];

            shellHook = ''
              alias ls=eza
              alias find=fd
            '';
            LD_LIBRARY_PATH = "\$\{LD_LIBRARY_PATH\}:${ pkgs.lib.makeLibraryPath [ pkgs.vulkan-loader pkgs.libGL ] }";
          };
        };
      }
    );
}
