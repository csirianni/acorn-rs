#!/usr/bin/env sh
# Generate Rust bindings to the ACORN C API
#
# Ensure that the submodule is updated and checked out in the intended revision
if ! which bindgen > /dev/null; then
    echo "ERROR: `bindgen` not found. Please install using cargo:"
    echo "    cargo install bindgen-cli --version=^0.69"
    exit 1
fi

repo_url=https://github.com/csirianni/ACORN.git 
repo_branch=oak
cuda_root=/opt/cuda

if [ ! -d acorn ]; then
    git clone "$repo_url" acorn --branch "$repo_branch" --depth 1
fi

bindgen_opt='--allowlist-function faiss_.* --allowlist-type idx_t|Faiss.* --opaque-type FILE'

headers=`ls /acorn/c_api/*_c.h /acorn/c_api/impl/*_c.h /acorn/c_api/utils/*_c.h`
echo '// Auto-generated, do not edit!' > c_api.h
for header in $headers; do
    echo "#include \""$header"\"" >> c_api.h;
done

cmd="bindgen --rust-target 1.59 $bindgen_opt c_api.h -o src/bindings.rs"
echo ${cmd}
${cmd}

# clean up
rm -f c_api.h

