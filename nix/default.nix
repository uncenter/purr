{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
  stdenv,
  darwin,
}:
rustPlatform.buildRustPackage {
  pname = "purr";
  inherit ((lib.importTOML ../Cargo.toml).package) version;

  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [ pkg-config ];

  buildInputs =
    [ openssl ]
    ++ lib.optionals stdenv.isDarwin (
      with darwin.apple_sdk.frameworks;
      [
        Security
        SystemConfiguration
      ]
    );

  meta = with lib; {
    description = "Utility commands for managing userstyles";
    homepage = "https://github.com/uncenter/purr";
    license = licenses.mit;
    mainProgram = "purr";
  };
}
