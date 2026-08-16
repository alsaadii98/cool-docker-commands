{
  description = "dok — docker output, made readable";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "dok";
          version = manifest.version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          # No tests touch the daemon, but the sandbox has no socket anyway.
          doCheck = true;

          meta = with pkgs.lib; {
            description = manifest.description;
            homepage = manifest.repository;
            license = licenses.mit;
            mainProgram = "dok";
            platforms = platforms.unix ++ platforms.windows;
          };
        };

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
          name = "dok";
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [ cargo rustc rustfmt clippy rust-analyzer docker-client python3 ];
        };
      });
}
