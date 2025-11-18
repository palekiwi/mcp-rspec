{
  description = "A flake for mcp-rspec";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rustToolchain = fenix.packages.${system}.stable.toolchain;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "mcp-rspec";
          version = "0.1.0-dev";
          src = pkgs.lib.cleanSource ./.;

          cargoHash = "sha256-c6iM90V7/t38qY9ZKJajwVn0/48gqeGatEIkwpkbUgw";

          meta = with pkgs.lib; {
            description = "MCP for running RSpec";
            license = licenses.mit;
            maintainers = [ ];
          };
        };

        devShells.default = pkgs.mkShell
          {
            buildInputs = [
              rustToolchain
              pkgs.rust-analyzer
              pkgs.cargo-expand
              pkgs.cargo-watch
              pkgs.cargo-edit
            ];

            shellHook = ''
              echo "Rust development environment ready!"
              echo "Rust version: $(rustc --version)"
            '';
          };
      });
}
