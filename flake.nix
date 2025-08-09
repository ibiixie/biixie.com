{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    inputs@{ self, nixpkgs, ... }:
    let
      inherit (self) outputs;

      systems = [
        "x86_64-linux"
      ];

      pkgsFor = nixpkgs.lib.genAttrs systems (
        system:
        import nixpkgs {
          inherit system;
        }
      );
    in
    {
      devShells.x86_64-linux.default =
        let
          pkgs = pkgsFor.x86_64-linux;
        in
        pkgs.mkShell {
          buildInputs = with pkgs; [
            # Nix
            nil
            nixfmt-rfc-style

            # Misc
            nodejs_23
            yaml-language-server
            nodePackages.prettier

            # Rust
            pkg-config
            openssl
            rustc
            cargo
            gcc
            rust-analyzer
            rustfmt
            clippy
            lldb

            # Toml
            taplo

            # Dockerfile
            dockerfile-language-server-nodejs

            # Docker compose
            docker-compose-language-service

            # JSON, CSS
            vscode-langservers-extracted

            # HTML
            superhtml

            # JavaScript + TypeScript
            typescript
            typescript-language-server

            zola

            # Docker Bake/HCL
            terraform-ls
          ];
        };

      formatter.x86_64-linux =
        let
          pkgs = pkgsFor.x86_64-linux;
        in
        pkgs.nixfmt-rfc-style;
    };
}
