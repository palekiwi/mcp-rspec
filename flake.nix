{
  description = "A flake for mcp-rspec";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, fenix, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      rustToolchain = fenix.packages.${system}.stable.toolchain;
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "mcp-rspec";
        version = "0.1.0";
        src = ./.;

        cargoHash = "sha256-c6iM90V7/t38qY9ZKJajwVn0/48gqeGatEIkwpkbUgw";

        meta = with pkgs.lib; {
          description = "MCP for running RSpec";
          license = licenses.mit;
          maintainers = [ ];
        };
      };

      devShells.${system}.default = pkgs.mkShell
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
    };
}
