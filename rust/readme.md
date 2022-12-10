cargo watch -x run
cargo run
docker build -t rust .
docker run --rm -it -p 2000:2000 rust