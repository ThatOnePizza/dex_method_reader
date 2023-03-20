use dex::DexReader;
use std::env;

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if an dex file path is provided
    if args.len() < 2 {
        println!("Usage: {} <DEX file>", args[0]);
        return;
    }

    let dex_path = &args[1];

    // Parse the DEX file
    let dex = DexReader::from_file(&dex_path).expect("Failed to parse the DEX files");

    // Loop through each class in the DEX file
    for class_def in dex.classes() {
        if let Ok(class) = class_def {
            // Get the class name
            let class_type = class.jtype();

            // Loop through each method in the class
            for method in class.methods() {
                // Get the method name and signature
                let method_name = method.name();
                let method_sig = method
                    .signature()
                    .expect("Couldnt get signature of method")
                    .unwrap_or(String::from("undefined"));

                // Print the method name and signature
                println!(
                    "Class: {} | Method: {} Signature: {}",
                    class_type, method_name, method_sig
                );
            }
        }
    }
}
