let mozilla_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
    nixpkgs = import <nixpkgs> { overlays = [ mozilla_overlay ]; };
    rustPkgs = nixpkgs.rustChannelOf { channel = "1.36.0"; };
    rustup-faker = import (nixpkgs.fetchFromGitHub {
        owner = "hnefatl";
        repo = "rustup-faker";
        rev = "1bb0a6f357727e43218399ea6624f12ada2c3047";
        sha256 = "0m8kp6hxd6b41p1ydyrc3p0i8xi4grqcns6jjpfjmpykay5d0n6m";
    });
in
    with nixpkgs;
    stdenv.mkDerivation {
        name = "rust-particles";

        buildInputs = [
            x11
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            libGL

            lldb

            # Required by VSCode's RLS extension
            rustup-faker
            rustPkgs.rust-analysis
            rustPkgs.rust-src
            rustPkgs.rust-std
            rustPkgs.rls-preview

            rustPkgs.cargo
            rustPkgs.rust
            rustPkgs.rustfmt-preview
            rustPkgs.clippy-preview
        ];

        LD_LIBRARY_PATH = "${rustPkgs.rust}/lib:${xorg.libX11}/lib:${xorg.libXcursor}/lib:${xorg.libXrandr}/lib:${xorg.libXi}/lib:${libGL}/lib:$LD_LIBRARY_PATH";
    }
