#!/bin/bash
###
 # @Descripttion: 
 # @version: 
 # @Author: Wynters
 # @Date: 2024-05-05 23:03:31
 # @LastEditTime: 2024-05-18 20:19:55
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
                

        rm -rf "./target/$LINK/release/RustPanel"
        rm -rf "./target/$LINK/release/panel"
        rm -rf "./target/$LINK/release/rp"

        cargo build --target "$LINK" --release
        if [ $? -ne 0 ]; then
            echo ">>> Error: Cargo build failed from Linux"
        else
            #mv "./target/$LINK/release/RustPanel" "./target/$LINK/release/panel"
            echo "Cargo build successful from $LINK"

            mkdir -p "./target/$LINK/release/RustPanel"
            mkdir -p "./target/$LINK/release/RustPanel/bin"
            mkdir -p "./target/$LINK/release/RustPanel/runtime"
            cp -r "./addons" "./target/$LINK/release/RustPanel" > NUL
            cp -r "./config" "./target/$LINK/release/RustPanel" > NUL
            cp -r "./public" "./target/$LINK/release/RustPanel" > NUL
            cp -r "./locales" "./target/$LINK/release/RustPanel" > NUL

            cp "./target/$LINK/release/rp" "./target/$LINK/release/RustPanel/bin/rp"
            cp "./target/$LINK/release/panel" "./target/$LINK/release/RustPanel/panel"

            ./build/upx -9 -qvf "./target/$LINK/release/RustPanel/panel" "./target/$LINK/release/RustPanel/bin/rp"
             

            echo $(grep 'version =' ./Cargo.toml | grep -o '[0-9]*\.[0-9]*\.[0-9]*' | tail -n1) > "./target/$LINK/release/RustPanel/VERSION"

            rm -rf "./target/RustPanel_$VERSION-$LINK.tar.gz" 
            tar -czvf "./target/RustPanel_$VERSION-$LINK.tar.gz" -C "./target/$LINK/release" RustPanel > NUL

            echo ">>> :) build successful, directory in: ./target/$LINK/release/RustPanel"
        fi
    ;;
    Win*)
            
        LINK="x86_64-pc-windows-msvc"
        VERSION=$(grep 'version =' ./Cargo.toml | grep -o '[0-9]*\.[0-9]*\.[0-9]*')

        rm -rf "./target/$LINK/release/*.*"
        rm -rf "./target/$LINK/release/RustPanel"

        cargo build --target "$LINK" --release
        if [ $? -ne 0 ]; then
            echo ">>> Error: Cargo build failed from Windows"
        else
            #mv "./target/$LINK/release/RustPanel.exe" "./target/$LINK/release/panel.exe"
            echo "Cargo build successful from $LINK"

            mkdir -p "./target/$LINK/release/RustPanel"
            mkdir -p "./target/$LINK/release/RustPanel/bin"
            mkdir -p "./target/$LINK/release/RustPanel/runtime"
            cp -r "./addons" "./target/$LINK/release/RustPanel" > NUL
            cp -r "./config" "./target/$LINK/release/RustPanel" > NUL
            cp -r "./public" "./target/$LINK/release/RustPanel" > NUL
            cp -r "./locales" "./target/$LINK/release/RustPanel" > NUL

            cp "./target/$LINK/release/rp.exe" "./target/$LINK/release/RustPanel/bin/rp.exe"
            cp "./target/$LINK/release/panel.exe" "./target/$LINK/release/RustPanel/panel.exe"
            echo $(grep 'version =' ./Cargo.toml | grep -o '[0-9]*\.[0-9]*\.[0-9]*' | tail -n1) > "./target/$LINK/release/RustPanel/VERSION"

            rm -rf "./target/RustPanel_$VERSION-$LINK.tar.gz" 
            tar -czvf "./target/RustPanel_$VERSION-$LINK.tar.gz" -C "./target/$LINK/release" RustPanel > NUL

            
            echo ">>> :) build successful, directory in: ./target/$LINK/release/RustPanel"
        fi

    ;;
    *)
        VERSION=$(grep 'version =' ./Cargo.toml | grep -o '[0-9]*\.[0-9]*\.[0-9]*')

        rm -rf "./target/release/RustPanel"
        rm -rf "./target/$LINK/release/panel"
        rm -rf "./target/$LINK/release/rp"

        cargo build --release
        if [ $? -ne 0 ]; then
            echo ">>> Error: Cargo build failed from $OS"
        else
            #mv "./target/release/RustPanel" "./target/release/panel"
            echo "Cargo build successful from $LINK"

            mkdir -p "./target/release/RustPanel"
            mkdir -p "./target/release/RustPanel/bin"
            mkdir -p "./target/release/RustPanel/runtime"
            cp -r "./addons" "./target/release/RustPanel" > NUL
            cp -r "./config" "./target/release/RustPanel" > NUL
            cp -r "./public" "./target/release/RustPanel" > NUL
            cp -r "./locales" "./target/release/RustPanel" > NUL

            cp "./target/release/rp" "./target/release/RustPanel/bin/rp"
            cp "./target/release/panel" "./target/release/RustPanel/panel"

             ./build/upx -9 -qvf "./target/release/RustPanel/panel" "./target/release/RustPanel/bin/rp"

            echo $(grep 'version =' ./Cargo.toml | grep -o '[0-9]*\.[0-9]*\.[0-9]*' | tail -n1) > "./target/release/RustPanel/VERSION"

            rm -rf "./target/RustPanel_$VERSION-$LINK.tar.gz" 
            tar -czvf "./target/RustPanel_$VERSION-$LINK.tar.gz" -C "./target/release" RustPanel > NUL
            
            echo ">>> :) build successful, directory in: ./target/release/RustPanel"
        fi
    ;;
esac