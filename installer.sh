#!/usr/bin/env sh
# Copyright 2015-2016 Intecture Developers. See the COPYRIGHT file at the
# top-level directory of this distribution and at
# https://intecture.io/COPYRIGHT.
#
# Licensed under the Mozilla Public License 2.0 <LICENSE or
# https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
# modified, or distributed except according to those terms.

# Undefined vars are errors
set -u

# Globals
prefix=""
libdir=""
sysconfdir=""
ostype="$(uname -s)"

case "$ostype" in
    Linux)
        prefix="/usr"
		libdir="$prefix/lib"
        sysconfdir="/etc"

		if [ -d "${libdir}64" ]; then
			libdir="${libdir}64"
		fi
        ;;

    FreeBSD)
        prefix="/usr/local"
		libdir="$prefix/lib"
        sysconfdir="$prefix/etc"
        ;;

    Darwin)
        prefix="/usr/local"
		libdir="$prefix/lib"
        sysconfdir="$prefix/etc"
        ;;

    *)
        echo "unrecognized OS type: $ostype" >&2
        exit 1
        ;;
esac

do_install_c() {
    if ! $(pkg-config --exists libzmq); then
        install -m 755 lib/libzmq.so.5.1.0 $libdir
        ln -s $libdir/libzmq.so.5.1.0 $libdir/libzmq.so.5
        ln -s $libdir/libzmq.so.5.1.0 $libdir/libzmq.so
		install -m 644 lib/pkgconfig/libzmq.pc $libdir/pkgconfig/
        install -m 644 include/zmq.h $prefix/include/
    fi

    if ! $(pkg-config --exists libczmq); then
        install -m 755 lib/libczmq.so.4.0.0 $libdir
        ln -s $libdir/libczmq.so.4.0.0 $libdir/libczmq.so.4
        ln -s $libdir/libczmq.so.4.0.0 $libdir/libczmq.so
		install -m 644 lib/pkgconfig/libczmq.pc $libdir/pkgconfig/
        install -m 644 include/czmq.h $prefix/include/
        install -m 644 include/czmq_library.h $prefix/include/
        install -m 644 include/czmq_prelude.h $prefix/include/
        install -m 644 include/zactor.h $prefix/include/
        install -m 644 include/zarmour.h $prefix/include/
        install -m 644 include/zauth.h $prefix/include/
        install -m 644 include/zbeacon.h $prefix/include/
        install -m 644 include/zcert.h $prefix/include/
        install -m 644 include/zcertstore.h $prefix/include/
        install -m 644 include/zchunk.h $prefix/include/
        install -m 644 include/zclock.h $prefix/include/
        install -m 644 include/zconfig.h $prefix/include/
        install -m 644 include/zdigest.h $prefix/include/
        install -m 644 include/zdir.h $prefix/include/
        install -m 644 include/zdir_patch.h $prefix/include/
        install -m 644 include/zfile.h $prefix/include/
        install -m 644 include/zframe.h $prefix/include/
        install -m 644 include/zgossip.h $prefix/include/
        install -m 644 include/zhash.h $prefix/include/
        install -m 644 include/zhashx.h $prefix/include/
        install -m 644 include/ziflist.h $prefix/include/
        install -m 644 include/zlist.h $prefix/include/
        install -m 644 include/zlistx.h $prefix/include/
        install -m 644 include/zloop.h $prefix/include/
        install -m 644 include/zmonitor.h $prefix/include/
        install -m 644 include/zmsg.h $prefix/include/
        install -m 644 include/zpoller.h $prefix/include/
        install -m 644 include/zproxy.h $prefix/include/
        install -m 644 include/zrex.h $prefix/include/
        install -m 644 include/zsock.h $prefix/include/
        install -m 644 include/zstr.h $prefix/include/
        install -m 644 include/zsys.h $prefix/include/
        install -m 644 include/zuuid.h $prefix/include/
    fi

    local _libext="so"
    if [ -f libinapi.dylib.0.3.0 ]; then
        _libext="dylib"
    fi

    install -m 755 libinapi.$_libext.0.3.0 $libdir
    rm -f $libdir/libinapi.$_libext.0
    ln -s $libdir/libinapi.$_libext.0.3.0 $libdir/libinapi.$_libext.0
    rm -f $libdir/libinapi.$_libext
    ln -s $libdir/libinapi.$_libext.0.3.0 $libdir/libinapi.$_libext

    install -m 644 inapi.h $prefix/include/
}

do_install_php() {
    if ! type php 2>&1 >/dev/null; then
        echo "php-cli not found. You must install it first."
        exit 1
    fi

    do_install_c

    local _extdir=$(php -r "echo ini_get('extension_dir');")
    case $(php --version|grep -E '^PHP'|awk '{split($2, a, "."); print a[1]}') in
        5)
            cp inapi.so.5 $_extdir/inapi.so
            ;;

        7)
            cp inapi.so.7 $_extdir/inapi.so
            ;;

        *)
            echo "Unsupported PHP major version: $1"
            exit 1
            ;;
    esac

    local _phpini=$(php --ini|grep 'Loaded Configuration File'|awk '{print $4}')
    local _additionaldir=$(php --ini|grep 'Scan for additional'|awk '{print $7}')
    if [ -n $_additionaldir ]; then
        echo 'extension=inapi.so' > $_additionaldir/inapi.ini
    elif [ -n $_loadeddir ]; then
        echo 'extension=inapi.so' >> $_phpini
    else
        echo 'Could not find PHP extension ini file'
        exit 1
    fi
}

do_uninstall() {
    local _phpdir=$(php -r "echo ini_get('extension_dir');")
    local _phpini=$(php --ini|grep 'Loaded Configuration File'|awk '{print $4}')
    local _additionaldir=$(php --ini|grep 'Scan for additional'|awk '{print $7}')
    if [ -n $_additionaldir ]; then
        rm -f $_additionaldir/inapi.ini
    elif [ -n $_loadeddir ]; then
        sed 's/extension=inapi.so//' < $_phpini > php.ini.tmp
        mv php.ini.tmp $_phpini
    else
        echo 'Could not find PHP extension ini file'
        exit 1
    fi

    rm -f $libdir/libinapi.dylib \
          $libdir/libinapi.so \
          $prefix/include/inapi.h \
          $_phpdir/inapi.so
}

main() {
	if [ $# -eq 0 ]; then
		echo "Usage: installer.sh <install-c|install-php|uninstall>"
		exit 0
	fi

	case "$1" in
		install-c)
			do_install_c
			;;

        install-php)
			do_install_php
			;;

		uninstall)
			do_uninstall
			;;

		*)
			echo "Unknown option $1"
			exit 1
			;;
	esac
}

main "$@"