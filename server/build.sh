current=$(pwd)

# client build
cd ../client

cargo +nightly web deploy --target=wasm32-unknown-unknown --release

# server build
cd $current

mkdir -p ./static
cp ../client/target/deploy/client.* ./static/

cargo build