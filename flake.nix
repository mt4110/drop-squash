{
  description = "DropSquash reproducible development environment";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

  outputs = { nixpkgs, ... }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          linuxPackages = pkgs.lib.optionals pkgs.stdenv.isLinux (with pkgs; [
            glib
            gtk3
            libsoup_3
            webkitgtk_4_1
            gst_all_1.gstreamer
            gst_all_1.gst-plugins-base
            gst_all_1.gst-plugins-good
            gst_all_1.gst-plugins-bad
          ]);
        in
        {
          default = pkgs.mkShell {
            packages = (with pkgs; [
              cargo
              cargo-audit
              cargo-deny
              clippy
              nodejs_24
              pkg-config
              pnpm_10
              rustc
              rustfmt
            ]) ++ linuxPackages;

            RUST_BACKTRACE = "1";
          };
        });
    };
}
