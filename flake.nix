{
  description = "oxeylyzer-2";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      imports = [inputs.devshell.flakeModule inputs.treefmt-nix.flakeModule];
      perSystem = {
        pkgs,
        system,
        config,
        ...
      }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [inputs.rust-overlay.overlays.default];
        };
        packages.default = pkgs.callPackage ./. {
          inherit ((builtins.fromTOML (builtins.readFile ./Cargo.toml)).package) name version;
        };
        treefmt = {
          programs = {
            alejandra.enable = true;
            statix.enable = true;
            rustfmt.enable = true;
          };
          flakeFormatter = true;
          projectRootFile = "flake.nix";
        };
        devshells.default = {
          packages = with pkgs; [
            config.treefmt.build.wrapper
            # nix formatters
            alejandra
            statix
            # rust
            gcc
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
          ];
        };
      };
    };
}
