{
  description = "Development flake for howtovulkan-rs";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let 
        pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in {
        devShells."x86_64-linux".default = pkgs.mkShellNoCC rec {
            buildInputs = with pkgs; [
                ktx-tools

                libxkbcommon
                wayland

                vulkan-headers
                vulkan-loader
                vulkan-volk
                vulkan-tools
                vulkan-validation-layers
                vulkan-tools-lunarg
                vulkan-memory-allocator
                shaderc

                stdenv.cc.cc.lib
            ];

            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
            
            SHADERC_LIB_DIR = "${pkgs.lib.makeLibraryPath [pkgs.shaderc]}";

            VULKAN_SDK = "${pkgs.vulkan-validation-layers}/share/vulkan/implicit_layer.d";
            VK_LAYER_PATH = "${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d";
            VK_LOADER_DEBUG= "all vulkaninfo";
			VK_LAYER_PRINTF_TO_STDOUT = 1;
			VK_VALIDATION_FEATURES = "+DEBUG_PRINTF";
            VK_LAYER_PRINTF_BUFFER_SIZE = 65536;
        };
  };
}
