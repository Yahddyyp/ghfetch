{
  description = "Display your GitHub status in a beautiful way";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [
        "aarch64-darwin"
        "x86_64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];

      forAllSystems = nixpkgs.lib.genAttrs systems;

    in {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };

          release =
            if system == "aarch64-darwin" then {
              archive = "ghfetch-v1.2.0-aarch64-apple-darwin.tar.gz";
              hash = "sha256-UREnkxgDaJ4AjIcNpsdl3OY/HYcKlERDmq25qILfb2g=";
            }
            else if system == "x86_64-darwin" then {
              archive = "ghfetch-v1.2.0-x86_64-apple-darwin.tar.gz";
              hash = "sha256-9hz0PocHzBNsafyGY28U5DkmZieuhsPuoNotY9xU9XY=";
            }
            else if system == "aarch64-linux" then {
              archive = "ghfetch-v1.2.0-aarch64-unknown-linux-gnu.tar.gz";
              hash = "sha256-qsJlcwcJcn1jM624x9B5rsCqKtLUVpvW/SIbfxCEICU=";
            }
            else {
              archive = "ghfetch-v1.2.0-x86_64-unknown-linux-gnu.tar.gz";
              hash = "sha256-z5BgFqKyET8IvDJJIGGkDTz+qn9nCSPfk5wQgUYPb3k=";
            };
        in {
          default = pkgs.stdenvNoCC.mkDerivation {
            pname = "ghfetch";
            version = "1.2.0";

            src = pkgs.fetchurl {
              url = "https://github.com/Yahddyyp/ghfetch/releases/download/v1.2.0/${release.archive}";
              inherit (release) hash;
            };

            dontUnpack = true;

            installPhase = ''
              runHook preInstall
              mkdir -p $out/bin
              tar -xzf $src
              install -m755 ghfetch $out/bin/ghfetch
              runHook postInstall
            '';

            meta = {
              description = "Display your GitHub status in a beautiful way";
              homepage = "https://github.com/Yahddyyp/ghfetch";
              mainProgram = "ghfetch";
              platforms = [
                "aarch64-darwin"
                "x86_64-darwin"
                "aarch64-linux"
                "x86_64-linux"
              ];
            };
          };
        });
    };
}
