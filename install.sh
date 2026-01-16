#!/bin/sh

# copypaste from
# https://github.com/release-lab/install/

set -e

# eg. release-lab/whatchanged
target=""
owner="cwmoss"
repo="karton-rs"
exe_name="karton"
githubUrl=""
githubApiUrl=""
version="v0.1.0"

# rustup target list
# karton-aarch64-apple-darwin.tar.xz
# karton-x86_64-unknown-linux-gnu.tar.xz
#  => extra win installer karton-x86_64-pc-windows-msvc.zip

get_arch() {
    # darwin/amd64: Darwin axetroydeMacBook-Air.local 20.5.0 Darwin Kernel Version 20.5.0: Sat May  8 05:10:33 PDT 2021; root:xnu-7195.121.3~9/RELEASE_X86_64 x86_64
    # linux/amd64: Linux test-ubuntu1804 5.4.0-42-generic #46~18.04.1-Ubuntu SMP Fri Jul 10 07:21:24 UTC 2020 x86_64 x86_64 x86_64 GNU/Linux
    a=$(uname -m)
    case ${a} in
        "x86_64" | "amd64" )
            # echo "amd64"
            echo "x86_64"
        ;;
        "i386" | "i486" | "i586")
            echo "386"
        ;;
        "aarch64" | "arm64" | "arm")
            # echo "arm64"
            echo "aarch64"
        ;;
        "mips64el")
            echo "mips64el"
        ;;
        "mips64")
            echo "mips64"
        ;;
        "mips")
            echo "mips"
        ;;
        *)
            echo ${NIL}
        ;;
    esac
}

get_os(){
    # darwin: Darwin
    os=$(uname -s | awk '{print tolower($0)}')
    case ${os} in
        "linux" )
            echo "unknown-linux-gnu"
        ;;
        "darwin" )
            echo "apple-darwin"
        ;;
        "windowsnt" )
            echo "pc-windows-msvc"
        ;;
        *)
            echo ${NIL}
        ;;
    esac
}


if [ -z "$exe_name" ]; then
    exe_name=$repo
    echo "INFO: file name is not specified, use '$repo'"
    echo "INFO: if you want to specify the name of the executable, set flag --exe=name"
fi

if [ -z "$githubUrl" ]; then
    githubUrl="https://github.com"
fi
if [ -z "$githubApiUrl" ]; then
    githubApiUrl="https://api.github.com"
fi

downloadFolder="${TMPDIR:-/tmp}"
mkdir -p ${downloadFolder} # make sure download folder exists
os=$(get_os)
arch=$(get_arch)
file_name="${exe_name}-${arch}-${os}.tar.xz" # the file name should be download
downloaded_file="${downloadFolder}/${file_name}" # the file path should be download
executable_folder="/usr/local/bin" # Eventually, the executable file will be placed here

# if version is empty
if [ -z "$version" ]; then
    asset_path=$(
        command curl -L \
            -H "Accept: application/vnd.github+json" \
            -H "X-GitHub-Api-Version: 2022-11-28" \
            ${githubApiUrl}/repos/${owner}/${repo}/releases |
        command grep -o "/${owner}/${repo}/releases/download/.*/${file_name}" |
        command head -n 1
    )
    if [[ ! "$asset_path" ]]; then
        echo "ERROR: unable to find a release asset called ${file_name}"
        exit 1
    fi
    asset_uri="${githubUrl}${asset_path}"
else
    asset_uri="${githubUrl}/${owner}/${repo}/releases/download/${version}/${file_name}"
fi

echo "[1/3] Download ${asset_uri} to ${downloadFolder}"
rm -f ${downloaded_file}
curl --fail --location --output "${downloaded_file}" "${asset_uri}"

echo "[2/3] Install ${exe_name} to the ${executable_folder}"
tar -xJ -f ${downloaded_file} -C ${downloadFolder}
extracted="${file_name%.tar.xz}"
echo "extracted file: ${extracted}"
chmod +x ${downloadFolder}/$extracted/${exe_name}

if [ "$os" == "unknown-linux-gnu" ] ; then
    sudo cp ${downloadFolder}/$extracted/${exe_name} ${executable_folder}/${exe_name}
else
    cp ${downloadFolder}/$extracted/${exe_name} ${executable_folder}/${exe_name}
fi

exe=${executable_folder}/${exe_name}


echo "[3/3] Set environment variables"
echo "${exe_name} was installed successfully to ${exe}"

if [ "$os" == "apple-darwin" ] ; then
    echo "On macOS, you might need to run the following command to allow execution of the binary:"
    echo "  xattr -dr com.apple.quarantine ${exe}"
fi
    #if command -v ldconfig >/dev/null; then
    #    echo "Run 'ldconfig' to update shared library cache"
    #fi
exit 0

if command -v $exe_name --version >/dev/null; then
    echo "Run '$exe_name --help' to get started"
else
    echo "Manually add the directory to your \$HOME/.bash_profile (or similar)"
    echo "  export PATH=${executable_folder}:\$PATH"
    echo "Run '$exe_name --help' to get started"
fi

exit 0