# install

download from release page, unzip, move binary to a proper location

## mac example

    # download to current folder
    curl -L https://github.com/cwmoss/karton-rs/releases/download/v0.1.0/karton-aarch64-apple-darwin.tar.xz -o ./karton.tar.xz
    # unzip
    tar xfz karton.tar.xz
    # copy to system location
    cp karton-aarch64-apple-darwin/karton /usr/local/bin/
    # allow the file to be executable
    xattr -dr com.apple.quarantine /usr/local/bin/karton
    # remove downloaded file and archive folder
    rm -rf karton.tar.xz karton-aarch64-apple-darwin

## run

    # A
    # cd in a directory with jpg files
    karton

    # B
    # give karton the jpg root
    karton /path/to/images
