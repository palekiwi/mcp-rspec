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

      # Common build dependencies
      commonNativeBuildInputs = with pkgs; [ pkg-config ];
      commonBuildInputs = with pkgs; [ openssl ];

      # Common OpenSSL environment
      opensslEnv = {
        PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        OPENSSL_DIR = "${pkgs.openssl.out}";
        OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
        OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
      };
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage ({
        pname = "mcp-rspec";
        version = "0.1.0";
        src = ./.;

        cargoHash = "sha256-c6iM90V7/t38qY9ZKJajwVn0/48gqeGatEIkwpkbUgw";

        nativeBuildInputs = commonNativeBuildInputs;
        buildInputs = commonBuildInputs;

        meta = with pkgs.lib; {
          description = "MCP for running RSpec";
          license = licenses.mit;
          maintainers = [ ];
        };
      } // opensslEnv);

      devShells.${system}.default = pkgs.mkShell
        ({
          buildInputs = [
            rustToolchain
            pkgs.rust-analyzer
            pkgs.cargo-expand
            pkgs.cargo-watch
            pkgs.cargo-edit
          ] ++ commonNativeBuildInputs ++ commonBuildInputs;

          shellHook = ''
            echo "Rust development environment ready!"
            echo "Rust version: $(rustc --version)"
          '';
        } // opensslEnv);
    };
}
