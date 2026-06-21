#!/bin/bash

rm -rf AppDir
rm howtovulkan-rs-x86_64.AppImage

export ARCH=x86_64

mkdir -p AppDir/usr/bin
mkdir -p AppDir/usr/lib
mkdir -p AppDir/usr/share/applications
mkdir -p AppDir/usr/share/icons/hicolor/256x256/apps
mkdir -p AppDir/usr/share/metainfo

cargo build --features no-layers
cp target/debug/howtovulkan-rs AppDir/usr/bin/

ldd build/howtovulkan-rs | grep "=> /" | awk '{print $3}' | xargs -I{} cp -v {} AppDir/usr/lib/

cp /usr/lib/libvulkan.so.1 AppDir/usr/lib/

cp -r assets AppDir/usr/share/howtovulkan-rs/assets
cp -r shaders AppDir/usr/share/howtovulkan-rs/shaders

cat > AppDir/usr/share/applications/com.github.nonsensicalnickname.howtovulkan-rs.desktop << EOF
[Desktop Entry]
Name=My Vulkan Application
Exec=howtovulkan-rs
Icon=howtovulkan-rs
Type=Application
Categories=Graphics;
EOF

cp assets/icon.png AppDir/usr/share/icons/hicolor/256x256/apps/howtovulkan-rs.png
cp assets/icon.png AppDir/howtovulkan-rs.png

cp AppDir/usr/share/applications/com.github.nonsensicalnickname.howtovulkan-rs.desktop AppDir/com.github.nonsensicalnickname.howtovulkan-rs.desktop

cat > AppDir/usr/share/metainfo/howtovulkan-rs.appdata.xml << EOF
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
  <id>com.github.nonsensicalnickname.howtovulkan-rs</id>
  <metadata_license>MIT</metadata_license>
  <name>howtovulkan-rs</name>
  <summary>howtovulkan but its rust and weird</summary>
  <description>
    <p>
		Basic 3D renderer using Vulkan
    </p>
  </description>
  <launchable type="desktop-id">com.github.nonsensicalnickname.howtovulkan-rs.desktop</launchable>
  <url type="homepage">https://github.com/nonsensicalnickname/howtovulkan-rs</url>
  <releases>
    <release version="1.0.0" date="2026-06-21"/>
  </releases>
</component>
EOF

cat > AppDir/AppRun << EOF
#!/bin/bash
SELF=\$(readlink -f "\$0")
HERE=\$(dirname "\$SELF")
export PATH="\${HERE}/usr/bin:\${PATH}"
export LD_LIBRARY_PATH="\${HERE}/usr/lib:\${LD_LIBRARY_PATH}"
"\${HERE}/usr/bin/howtovulkan-rs" "$@"
EOF

chmod +x AppDir/AppRun

wget -c "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage"
chmod +x appimagetool-x86_64.AppImage

./appimagetool-x86_64.AppImage AppDir howtovulkan-rs-x86_64.AppImage
