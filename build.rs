fn main() {
    volo_build::Builder::thrift()
        .add_service("./src/idl/redis.thrift")
        .write()
        .unwrap();
}
