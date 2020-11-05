let
  sources = import ./nix/sources.nix;
  nixpkgs = import sources.nixpkgs {};

  rustPkgs = with (import ./nix/rust.nix { inherit sources; }); [
    rust cargo
  ];

in with nixpkgs;

mkShell {
  buildInputs = rustPkgs;
}
