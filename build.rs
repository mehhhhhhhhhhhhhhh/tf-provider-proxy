fn main() {
    tonic_build::compile_protos("proto/plugin/tfplugin5.0.proto").unwrap();
}
