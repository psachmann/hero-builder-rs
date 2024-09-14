{
  description = "Rust dependencies for the project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustup
          ];
          shellHook = ''
            # setting path for cargo binaries
            export PATH=$HOME/.cargo/bin:$PATH
            # setting backtrace for rust
            export RUST_BACKTRACE=1
          '';
        };
      }
    );
}
