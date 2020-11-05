{ sources ? import ./sources.nix}:

let
  nixpkgs = import sources.nixpkgs {
    overlays = [ (import sources.nixpkgs-mozilla) ];
  };

in nixpkgs.rustChannels.stable
