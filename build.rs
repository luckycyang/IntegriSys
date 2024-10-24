use burn_import::onnx::ModelGen;

fn main() {
    ModelGen::new()
        .input("src/model/test.onnx")
        .out_dir("model/")
        .run_from_script();
}
