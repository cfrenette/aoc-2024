{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            # Rust Deps
            (rust-bin.stable.latest.default.override
              {
                extensions = [ "rust-src" ];
              })
            rust-analyzer
          ];

          # Environment Variables
          RUST_SRC_PATH = "${rust-bin.stable.latest.default}/lib/rustlib/src/rust/library";
          # RUST_BACKTRACE = 1;
        };
      }
    );
}
