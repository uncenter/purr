{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      naersk,
      nixpkgs,
    }:
    let
      forAllSystems =
        function:
        nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (
          system: function nixpkgs.legacyPackages.${system}
        );
    in
    {
      packages = forAllSystems (pkgs: {
        purr =
          let
            naersk' = pkgs.callPackage naersk { };
          in
          naersk'.buildPackage {
            pname = "purr";
            src = ./.;
            buildInputs = with pkgs; [
              openssl
            ];
          };
        default = self.packages.${pkgs.stdenv.hostPlatform.system}.purr;
      });

      devShell = forAllSystems (
        pkgs:
        pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            clippy
            rustfmt
            rust-analyzer
          ];
          inputsFrom = [ self.packages.${pkgs.stdenv.hostPlatform.system}.purr ];
          env = {
            OPENSSL_NO_VENDOR = 1;
            RUST_SRC_PATH = toString pkgs.rustPlatform.rustLibSrc;
          };
        }
      );
    };
}
