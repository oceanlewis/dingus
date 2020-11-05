let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix { inherit sources; };
  nixpkgs = import sources.nixpkgs {};
  lib = nixpkgs.lib;

  cleanSourceFilter = name: type:
    baseNameOf (toString name) != "target";

  filteredSource =
    src: lib.cleanSourceWith {
      filter = cleanSourceFilter; inherit src;
    };

in with nixpkgs;

rustPlatform.buildRustPackage rec {
  name = "dingus";
  src = filteredSource ./.;
  buildInputs = [];

  checkPhase = "";
  cargoSha256 = "sha256:0rk3v2vv6l1kmn7agqlvax15qyp5zrrg2grs3j0yrzgpy5jfj9c9";

  meta = with stdenv.lib; {
    description = "Easily apply environment variables loaded from a config file to a shell session.";
    homepage = https://github.com/nuxeh/url-bot-rs;
    license = licenses.mit;
    platforms = platforms.all;
  };
}
