{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let 
        pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in {
        # dependencies per raylib-rs
        devShells."x86_64-linux".default = pkgs.mkShell rec {
            buildInputs = with pkgs; [
                libxkbcommon
                wayland

                vulkan-headers
                vulkan-loader
                vulkan-volk
                vulkan-tools
                vulkan-memory-allocator
                udev

                # raylib
                # glfw
                # cmake
                # clang
                # wayland

                # libGL
                # xorg.libXrandr
                # xorg.libXinerama
                # xorg.libXcursor
                # xorg.libXi
            ];

            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";

            # LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

            # env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
  };
}
