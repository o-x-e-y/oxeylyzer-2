{
  rustPlatform,
  name,
  version,
}: (rustPlatform.buildRustPackage {
  pname = name;
  inherit version;
  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
    allowBuiltinFetchGit = true;
  };
  meta = {
    homepage = "https://github.com/O-X-E-Y/oxeylyzer-2";
    mainProgram = "oxeylyzer";
  };
})
