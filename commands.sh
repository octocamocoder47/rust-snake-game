cd /workspaces/rust/game-dev/emsdk
source ./emsdk_env.sh
cd /workspaces/rust/game-dev/proj1/
# export EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sASSERTIONS=1 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1"
export EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sASSERTIONS=1 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1" && cargo build --target wasm32-unknown-emscripten --release

python3 -m http.server 8000