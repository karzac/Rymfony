#!/usr/bin/env bash

set -eu

# The goal here is to
# Check releases, and update the associated data here.
# @link https://github.com/caddyserver/caddy/releases
caddy_version="2.4.5"

function output {
    style_start=""
    style_end=""
    if [ "${2:-}" != "" ]; then
    case $2 in
        "success")
            style_start="\033[0;32m"
            style_end="\033[0m"
            ;;
        "error")
            style_start="\033[31;31m"
            style_end="\033[0m"
            ;;
        "info"|"warning")
            style_start="\033[33m"
            style_end="\033[39m"
            ;;
        "heading")
            style_start="\033[1;33m"
            style_end="\033[22;39m"
            ;;
    esac
    fi

    builtin echo -e "${style_start}${1}${style_end}"
}


kernel=$(uname -s 2>/dev/null || /usr/bin/uname -s)
case ${kernel} in
    "Linux"|"linux")
        kernel="linux"
        ;;
    "Darwin"|"darwin")
        kernel="mac"
        ;;
    *)
        output "Your OS '${kernel}' not supported" "error"
        exit 1
        ;;
esac


machine=$(uname -m 2>/dev/null || /usr/bin/uname -m)
case ${machine} in
    arm|armv7*)
        machine="arm"
        ;;
    aarch64*|armv8*|arm64)
        machine="arm64"
        ;;
    i[36]86)
        machine="386"
        ;;
    x86_64)
        machine="amd64"
        ;;
    *)
        output "Your architecture '${machine}' is not currently supported" "error"
        exit 1
        ;;
esac

platform="${kernel}_${machine}"

output "Platform: ${platform}" "info"

release_filename="caddy_${caddy_version}_${platform}.tar.gz"

binary_url="https://github.com/caddyserver/caddy/releases/download/v${caddy_version}/${release_filename}"

output "Downloading ${release_filename}" "info"

output "Downloading from "${binary_url}"" "info"

tmpfile_caddy=$(mktemp /tmp/rymfonycaddy.XXXXXXXXXX)
curl -sSL "${binary_url}" --output "${tmpfile_caddy}"

tar -xvzf "${tmpfile_caddy}" -C bin/ caddy

output "Caddy was successfully downloaded to bin/caddy" "success"
