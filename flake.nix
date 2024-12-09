{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs;
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-compat, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        aoclib-rs = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          src = builtins.path { path = ./.; name = "aoclib-rs"; };

          cargoLock = { lockFile = ./Cargo.lock; };
        };
        aoclib-rs-shell = pkgs.mkShell {
          inputsFrom = [ aoclib-rs ];
          packages = with pkgs; [
            clippy
            rustfmt
          ];
        };
      in
      {
        packages = {
          inherit aoclib-rs;
          default = aoclib-rs;
        };
        devShells = {
          inherit aoclib-rs-shell;
          default = aoclib-rs-shell;
        };
      }
    );
}
