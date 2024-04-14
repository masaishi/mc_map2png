use mc_map2png::process_image_file;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input> <output>", args[0]);
        std::process::exit(1);
    }

    // Use the function directly with `crate::`
    if let Err(e) = process_image_file(&args[1], &args[2]) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
