build *ARGS:
  @echo Building binary
  cargo build {{ARGS}}

rr *ARGS:
  ./target/release/numbergame {{ARGS}}

dockerize:
  docker build . -t numbergame

