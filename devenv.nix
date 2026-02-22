{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/packages/
  packages = [ pkgs.git pkgs.pre-commit ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = "stable";
    version = "1.93.1";
  };

  git-hooks.hooks = {
    rustfmt.enable = true;
    # Override the entry to use the shell's cargo-fmt
    rustfmt.entry = lib.mkForce "cargo fmt --all -- --color=always";
  
    clippy.enable = true;
    # Override the entry to use the shell's cargo-clippy
    clippy.entry = lib.mkForce "cargo clippy --";
  };
}
