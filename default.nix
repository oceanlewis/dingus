let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  inherit (pkgs) stdenv lib;

  cleanSourceFilter = name: type:
    baseNameOf (toString name) != "target";

  filteredSource =
    src: lib.cleanSourceWith {
      filter = cleanSourceFilter; inherit src;
    };

in

pkgs.rustPlatform.buildRustPackage rec {
  name = "dingus";
  src = filteredSource ./.;
  buildInputs = [ ];

  checkPhase = "";
  cargoSha256 = "sha256-+SGokTxfsQ0mipKkIzl6merurI0MnRzPisxAdaKSJAw=";

  meta = {
    description = "Easily apply environment variables loaded from a config file to a shell session.";
    homepage = https://github.com/davidarmstronglewis/dingus;
    license = lib.licenses.mit;
    platforms = lib.platforms.all;
  };
}
