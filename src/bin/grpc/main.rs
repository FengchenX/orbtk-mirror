
extern crate protoc_rust_grpc;


use protoc_rust_grpc::*;

fn main() {
    protoc_rust_grpc::Codegen::new()
        .out_dir("./src/bin/test")
        .input("./src/bin/grpc/hello.proto")
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc");
}
