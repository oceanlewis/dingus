let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix { inherit sources; };
  nixpkgs = import sources.nixpkgs {};

in with nixpkgs;

rustPlatform.buildRustPackage rec {
  name = "dingus";
  src = ./.;

  checkPhase = "cargo test";
  cargoSha256 = "sha256:0lym8r16jf12dnqpyybvw8k7jym17kqqm0bs55qqalbcjcdldn4z";
  buildInputs = [];

  meta = with stdenv.lib; {
    description = "Easily apply environment variables loaded from a config file to a shell session.";
    homepage = https://github.com/nuxeh/url-bot-rs;
    license = licenses.mit;
    platforms = platforms.all;
  };
}
