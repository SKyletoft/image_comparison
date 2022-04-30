{
	inputs = {
		naersk.url = "github:nix-community/naersk/master";
		nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
		utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, utils, naersk }:
		utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs { inherit system; };
				naersk-lib = pkgs.callPackage naersk { };
			in {
				defaultPackage = naersk-lib.buildPackage ./.;

				defaultApp = utils.lib.mkApp {
					drv = self.defaultPackage."${system}";
				};

				devShell = with pkgs; mkShell {
					shellHook = ''
						PS1="\e[32;1mnix-flake: \e[34m\w \[\033[00m\]\n↳ "
					'';
					buildInputs = [
						cargo
						rustc
						rustfmt
						pre-commit
						rustPackages.clippy

						valgrind
					];
					RUST_SRC_PATH = rustPlatform.rustLibSrc;
				};
			});
}
