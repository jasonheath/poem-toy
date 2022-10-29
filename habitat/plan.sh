pkg_name=poem-toy
pkg_origin=jasonheath
pkg_version="0.1.0"
pkg_maintainer="Jason Heath <jason.heath@progress.com>"
pkg_license=("Apache-2.0") # : When last reviewed poem was licensed Apache 2.0 and MIT
pkg_svc_user="root"

pkg_deps=(
  core/gcc
  core/gcc-libs
  core/git 
  core/rust
)

pkg_build_deps=(
  core/gcc
  core/gcc-libs
  core/git 
  core/rust
)

# The default implementation is that the software specified in $pkg_source is
# downloaded, checksum-verified, and placed in $HAB_CACHE_SRC_PATH/$pkgfilename,
# which resolves to a path like /hab/cache/src/filename.tar.gz. You should
# override this behavior if you need to change how your binary source is
# downloaded, if you are not downloading any source code at all, or if your are
# cloning from git. If you do clone a repo from git, you must override
# do_verify() to return 0.
do_download() {
  git clone https://github.com/jasonheath/poem-toy.git
}

# The default implementation tries to verify the checksum specified in the plan
# against the computed checksum after downloading the source tarball to disk.
# If the specified checksum doesn't match the computed checksum, then an error
# and a message specifying the mismatch will be printed to stderr. You should
# not need to override this behavior unless your package does not download
# any files.
do_verify() {
  return 0 # required per do_download() comments
}

# The default implementation is to update the prefix path for the configure
# script to use $pkg_prefix and then run make to compile the downloaded source.
# This means the script in the default implementation does
# ./configure --prefix=$pkg_prefix && make. You should override this behavior
# if you have additional configuration changes to make or other software to
# build and install as part of building your package.
do_build() {
  # do_default_build
  build_line "rm -rf ./habitat/poem-toy"
  rm -rf ./habitat/poem-toy

  build_line "cargo clean"
  cargo clean

  build_line "cargo build --release"
  cargo build --release
}

# The default implementation is to run make install on the source files and
# place the compiled binaries or libraries in HAB_CACHE_SRC_PATH/$pkg_dirname,
# which resolves to a path like /hab/cache/src/packagename-version/. It uses
# this location because of do_build() using the --prefix option when calling the
# configure script. You should override this behavior if you need to perform
# custom installation steps, such as copying files from HAB_CACHE_SRC_PATH to
# specific directories in your package, or installing pre-built binaries into
# your package.
do_install() {
  build_line "cp target/release/${pkg_name} ${pkg_prefix}"
  cp target/release/${pkg_name} ${pkg_prefix}

  build_line "cp -r files ${pkg_prefix}"
  cp -r files ${pkg_prefix}

  build_line "cp -r templates ${pkg_prefix}"
  cp -r templates ${pkg_prefix}
}

# The default implementation is to strip any binaries in $pkg_prefix of their
# debugging symbols. You should override this behavior if you want to change
# how the binaries are stripped, which additional binaries located in
# subdirectories might also need to be stripped, or whether you do not want the
# binaries stripped at all.
do_strip() {
  return 0 # should be fine due to cargo build --release 
}