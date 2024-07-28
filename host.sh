set -ex



# export EMCC_CFLAGS="-pthread -s PTHREAD_POOL_SIZE=3 --matomics --mbulk-memory -sUSE_SDL_TTF=2 -sUSE_SDL=2 -s USE_PTHREADS -pthread" 
# cargo build --target=wasm32-unknown-emscripten
# cp target/wasm32-unknown-emscripten/debug/webrogue_web.wasm target/wasm32-unknown-emscripten/debug/webrogue_web.js test


emcmake cmake -S . -B build -DCMAKE_BUILD_TYPE=Release && cmake --build build/ --target webrogue
cp build/webrogue.wasm build/webrogue.js root
cd root
python3 -m http.server
