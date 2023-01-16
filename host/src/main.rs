use methods::{MULTIPLY_ID, MULTIPLY_PATH};
use risc0_zkvm::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};

fn main() {
    let a: u64 = 17;
    let b: u64 = 23;

    // Make the prover.
    let mut prover = Prover::new(&std::fs::read(MULTIPLY_PATH).unwrap(), MULTIPLY_ID).unwrap();
    
    prover.add_input_u32_slice(to_vec(&a).unwrap().as_slice());
    prover.add_input_u32_slice(to_vec(&b).unwrap().as_slice());

    // Run prover & generate receipt
    let receipt = prover.run().unwrap();

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = from_slice(receipt.journal.as_slice()).unwrap();

    // Print an assertion
    println!("Hello, world! I know the factors of {}, and I can prove it!", c);

    // Submit receipt somewhere
}
