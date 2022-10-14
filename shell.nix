let
    pkgs = import <nixpkgs> {};
in
    pkgs.mkShell {
        buildInputs = with pkgs; [
            # Rust related dependencies
            rustup
	    rust-analyzer

	    pkg-config
	    openssl
        ];
    }
