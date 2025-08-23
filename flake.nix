{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux"];
      perSystem = {
        config,
        self',
        pkgs,
        lib,
        system,
        ...
      }: let
        devDeps = with pkgs; [gdb cargo-tarpaulin cargo-audit sccache cargo-watch gnuplot cargo-show-asm lldb];
        runtimeDeps = with pkgs; [];
        buildDeps = with pkgs; [pkg-config rustPlatform.bindgenHook openssl];
        cargoToml = builtins.fromTOML (builtins.readFile ./chess/Cargo.toml);

        chess = features: rustc:
          (pkgs.makeRustPlatform {
            cargo = rustc;
            rustc = rustc;
          }).buildRustPackage {
            inherit (cargoToml.package) name version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            buildFeatures = features;
            buildInputs = runtimeDeps;
            nativeBuildInputs = buildDeps;
            # Uncomment if your cargo tests require networking or otherwise
            # don't play nicely with the Nix build sandbox:
            doCheck = true;
          };

        mkDevShell = rustc:
          pkgs.mkShell {
            shellHook = ''
              export RUSTC_WRAPPER="sccache"
              export SCCACHE_CACHE_SIZE="10G"

              export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
            '';
            buildInputs = runtimeDeps;
            nativeBuildInputs = buildDeps ++ devDeps ++ [rustc];
          };
      in {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust-overlay)];
        };

        packages.default = self'.packages.nightly;
        packages.nightly = chess "" (pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default));
        packages.hello = pkgs.hello;

        devShells.default = self'.devShells.nightly;
        devShells.nightly =
          mkDevShell (pkgs.rust-bin.selectLatestNightlyWith
            (toolchain: toolchain.default));
        devShells.stable = mkDevShell pkgs.rust-bin.stable.latest.default;

      };
      flake = {
        hydraJobs = {
          inherit (inputs.self) packages;
        };
      };
    };
}
