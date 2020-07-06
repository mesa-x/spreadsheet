fn main() {
    tonic_build::compile_protos("proto/hello_world.proto").unwrap();
}
