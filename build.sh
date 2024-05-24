#!/bin/bash
###
 # @Descripttion: 
 # @version: 
 # @Author: Wynters
 # @Date: 2024-05-05 23:03:31
 # @LastEditTime: 2024-05-24 13:03:11
 # @FilePath: \RustPanel\build.sh
 #
 #
 #docker run -d -v F:/Users/Wynters/rust/RustPanel:/build rust:alpine3.19 tail -f NUL
 #apk update
 #apk add libc-dev
### 
echo ">>> Building RustPanel in release mode..."
OS=$(uname)
case "$OS" in
    Linux)
        #LINK="x86_64-unknown-linux-gnu"
        LINK="x86_64-unknown-linux-musl"
        VERSION=$(grep 'version =' ./Cargo.toml | grep -o '[0-9]*\.[0-9]*\.[0-9]*')
        TARGET_DIR="./target/$LINK/release/RustPanel"

        rm -rf "$TARGET_DIR"
        rm -rf "./target/$LINK/release/panel"
        rm -rf "./target/$LINK/release/rp"

        cargo build --target "$LINK" --release
        if [ $? -ne 0 ]; then
            echo ">>> Error: Cargo build failed from Linux"
        else
            #mv "$TARGET_DIR" "./target/$LINK/release/panel"
            echo "Cargo build successful from $LINK"

            mkdir -p "$TARGET_DIR"
            mkdir -p "$TARGET_DIR/bin"
            mkdir -p "$TARGET_DIR/runtime"

            dirs="addons config public locales install"
            for dir in $dirs; do
                cp -r "./$dir" "$TARGET_DIR" > /dev/null
                if [ $? -ne 0 ]; then
                     echo "error: copy $dir to $TARGET_DIR fial"
                    exit 1
                fi
            done


            cp "./target/$LINK/release/rp"     "$TARGET_DIR/bin/rp"
            cp "./target/$LINK/release/panel"  "$TARGET_DIR/panel"

            ./build/upx -9 -qvf "$TARGET_DIR/panel" "$TARGET_DIR/bin/rp"
             

            echo $(grep 'version =' ./Cargo.toml | grep -o '[0-9]*\.[0-9]*\.[0-9]*' | tail -n1) > "$TARGET_DIR/VERSION"

            rm -rf "./target/RustPanel_$VERSION-$LINK.tar.gz" 
            tar -czvf "./target/RustPanel_$VERSION-$LINK.tar.gz" -C "./target/$LINK/release" RustPanel > NUL

            echo ">>> :) build successful, directory in: $TARGET_DIR"
        fi
    ;;
    Win*)
            
        LINK="x86_64-pc-windows-msvc"
        VERSION=$(grep 'version =' ./Cargo.toml | grep -o '[0-9]*\.[0-9]*\.[0-9]*')
        TARGET_DIR="./target/$LINK/release/RustPanel"

        rm -rf "./target/$LINK/release/*.*"
        rm -rf "$TARGET_DIR"

        cargo build --target "$LINK" --release
        if [ $? -ne 0 ]; then
            echo ">>> Error: Cargo build failed from Windows"
        else
            #mv "$TARGET_DIR.exe" "./target/$LINK/release/panel.exe"
            echo "Cargo build successful from $LINK"

            mkdir -p "$TARGET_DIR"
            mkdir -p "$TARGET_DIR/bin"
            mkdir -p "$TARGET_DIR/runtime"

            dirs="addons config public locales install"
            for dir in $dirs; do
                cp -r "./$dir" "$TARGET_DIR" > /dev/null
                if [ $? -ne 0 ]; then
                    echo "error: copy $dir to $TARGET_DIR fial"
                    exit 1
                fi
            done

            cp "./target/$LINK/release/rp.exe"     "$TARGET_DIR/bin/rp.exe"
            cp "./target/$LINK/release/panel.exe"  "$TARGET_DIR/panel.exe"
            echo $(grep 'version =' ./Cargo.toml | grep -o '[0-9]*\.[0-9]*\.[0-9]*' | tail -n1) > "$TARGET_DIR/VERSION"

            rm -rf "./target/RustPanel_$VERSION-$LINK.tar.gz" 
            tar -czvf "./target/RustPanel_$VERSION-$LINK.tar.gz" -C "./target/$LINK/release" RustPanel > NUL

            
            echo ">>> :) build successful, directory in: $TARGET_DIR"
        fi

    ;;
    *)
        VERSION=$(grep 'version =' ./Cargo.toml | grep -o '[0-9]*\.[0-9]*\.[0-9]*')
        TARGET_DIR="./target/release/RustPanel"
        
        rm -rf "$TARGET_DIR"
        rm -rf "./target/$LINK/release/panel"
        rm -rf "./target/$LINK/release/rp"

        cargo build --release
        if [ $? -ne 0 ]; then
            echo ">>> Error: Cargo build failed from $OS"
        else
            #mv "$TARGET_DIR" "./target/release/panel"
            echo "Cargo build successful from $LINK"

            mkdir -p "$TARGET_DIR"
            mkdir -p "$TARGET_DIR/bin"
            mkdir -p "$TARGET_DIR/runtime"

            dirs="addons config public locales install"
            for dir in $dirs; do
                cp -r "./$dir" "$TARGET_DIR" > /dev/null
                if [ $? -ne 0 ]; then
                     echo "error: copy $dir to $TARGET_DIR fial"
                    exit 1
                fi
            done

            cp "./target/release/rp"     "$TARGET_DIR/bin/rp"
            cp "./target/release/panel"  "$TARGET_DIR/panel"

             ./build/upx -9 -qvf "$TARGET_DIR/panel" "$TARGET_DIR/bin/rp"

            echo $(grep 'version =' ./Cargo.toml | grep -o '[0-9]*\.[0-9]*\.[0-9]*' | tail -n1) > "$TARGET_DIR/VERSION"

            rm -rf "./target/RustPanel_$VERSION-$LINK.tar.gz" 
            tar -czvf "./target/RustPanel_$VERSION-$LINK.tar.gz" -C "./target/release" RustPanel > NUL
            
            echo ">>> :) build successful, directory in: $TARGET_DIR"
        fi
    ;;
esac