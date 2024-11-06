# Use official rust image
FROM rust 

# Build ACORN
RUN apt-get update
# RUN apt-get install cmake -y
RUN apt-get install git -y
RUN git clone -b oak https://github.com/csirianni/ACORN.git acorn

WORKDIR acorn-sys

# RUN apt install build-essential -y
RUN apt install libclang-dev -y
RUN cargo install bindgen-cli
RUN rustup component add rustfmt

# Build acorn-sys
COPY . .
CMD ./gen_bindings.sh

