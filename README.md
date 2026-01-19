# install

download from release page, unzip, move binary to a proper location

## via install.sh

    curl -fsSL https://raw.githubusercontent.com/cwmoss/karton-rs/refs/heads/main/install.sh | bash -s

## mac manual example

    # download to current folder
    curl -L https://github.com/cwmoss/karton-rs/releases/download/v0.1.0/karton-aarch64-apple-darwin.tar.xz -o ./karton.tar.xz
    # unzip
    tar xJf karton.tar.xz
    # copy to system location
    cp karton-aarch64-apple-darwin/karton /usr/local/bin/
    # allow the file to be executable
    xattr -dr com.apple.quarantine /usr/local/bin/karton
    # remove downloaded file and archive folder
    rm -rf karton.tar.xz karton-aarch64-apple-darwin

## linux manual example

    # download to current folder
    curl -L https://github.com/cwmoss/karton-rs/releases/download/v0.1.0/karton-x86_64-unknown-linux-gnu.tar.xz -o ./karton.tar.xz
    tar xJf karton.tar.xz
    cp karton-x86_64-unknown-linux-gnu/karton /usr/local/bin/
    rm -rf karton.tar.xz karton-x86_64-unknown-linux-gnu

## windows manual example

    # TODO: I have no idea
    curl -L https://github.com/cwmoss/karton-rs/releases/download/v0.1.0/karton-x86_64-pc-windows-msvc.zip -o ./karton.zip
    7z e karton.zip
    # C:\Windows\ ?? is that a good location?
    cp karton-x86_64-pc-windows-msvc/karton.exe C:\Windows\
    rm -rf karton.zip karton-x86_64-pc-windows-msvc

## run

### examples

    # cd in a directory with jpg files
    # -o starts your webbrowser
    karton serve -o

    # give karton the jpg root
    karton -b /path/to/images serve

    # use as local browser
    karton browse

### help

    # see all options
    karton -h

    # see all options for serve command
    karton serve -h
