{
  description = "A flake for developing cashu-rs-mint";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";

    flakebox = {
      url = "github:rustshop/flakebox";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flakebox, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        lib = pkgs.lib;
        flakeboxLib = flakebox.lib.${system} { };

        rustSrc = flakeboxLib.filterSubPaths {
          root = builtins.path {
            name = "cashu-rs-mint";
            path = ./.;
          };
          paths = [ "Cargo.toml" "Cargo.lock" ".cargo" "src" ];
        };

        targetsStd = flakeboxLib.mkStdTargets { };
        toolchainsStd = flakeboxLib.mkStdToolchains { };
        toolchainNative = flakeboxLib.mkFenixToolchain {
          targets = (pkgs.lib.getAttrs [ "default" ] targetsStd);
        };

        commonArgs = {
          buildInputs = [ pkgs.openssl ] ++ lib.optionals pkgs.stdenv.isDarwin
            [ pkgs.darwin.apple_sdk.frameworks.SystemConfiguration ];
          nativeBuildInputs = [ pkgs.pkg-config ];
        };
        outputs = (flakeboxLib.craneMultiBuild { toolchains = toolchainsStd; })
          (craneLib':
            let
              craneLib = (craneLib'.overrideArgs {
                pname = "flexbox-multibuild";
                src = rustSrc;
              }).overrideArgs commonArgs;
            in rec {
              workspaceDeps = craneLib.buildWorkspaceDepsOnly { };
              workspaceBuild =
                craneLib.buildWorkspace { cargoArtifacts = workspaceDeps; };
              cashu-rs-mint = craneLib.buildPackageGroup {
                pname = "cashu-rs-mint";
                packages = [ "cashu-rs-mint" ];
                mainProgram = "cashu-rs-mint";
              };
            });
      in {
        legacyPackages = outputs;
        packages = { default = outputs.cashu-rs-mint; };
        devShells = flakeboxLib.mkShells {
          toolchain = toolchainNative;
          packages = [ pkgs.clightning pkgs.bitcoind pkgs.just pkgs.mprocs ];
          nativeBuildInputs = [ ];
          shellHook = ''
            export CASHU_RS_MINT_DIR=/tmp/cashu-rs-mint
            export BITCOIN_DIR=$CASHU_RS_MINT_DIR/bitcoin
            export LIGHTNING_DIR=$CASHU_RS_MINT_DIR/lighting
            mkdir -p $CASHU_RS_MINT_DIR
            mkdir -p $BITCOIN_DIR
            mkdir -p $LIGHTNING_DIR
            mkdir -p $LIGHTNING_DIR/ln_1
            mkdir -p $LIGHTNING_DIR/ln_2
            alias btc="bitcoin-cli -regtest -datadir=$BITCOIN_DIR"
            alias ln1="lightning-cli --lightning-dir=$LIGHTNING_DIR/ln_1 --regtest"
            alias ln2="lightning-cli --lightning-dir=$LIGHTNING_DIR/ln_2 --regtest"
          '';
        };
      });
}
