let
  sources = import ./nix/sources.nix;
  nixpkgs = import sources.nixpkgs {};
  rs = import ./nix/rust.nix { inherit sources; };

in with nixpkgs;

mkShell {
  RUST_SRC_PATH = "${rs.rust-src.outPath}/lib/rustlib/src/rust/library";

  buildInputs = with rs; [
    cargo
    rust
    rust-src
  ];
}
