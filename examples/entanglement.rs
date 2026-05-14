// This example demonstrates quantum entanglement

use tiny_qsim::{Gate, QuantumSystem};

fn main() {
    let mut s = QuantumSystem::new(rand::rng());
    let a = s.new_qubit();
    let b = s.new_qubit();

    for _ in 0..10 {
        // Reset both qubits to the 0 state
        s.reset();

        // Put qubit 0 into an equal superposition
        s.apply(&Gate::h(), &[a]);

        // Apply a controlled NOT gate to qubit 1, controlled by qubit 0.
        // This creates an entangled state where the only information known
        // about the two qubits is that they are equal, but their actual
        // values are still in superposition.
        s.apply_controlled(&Gate::x(), &[a], &[b]);

        // Measure the two qubits, demonstrating that they will always
        // either collapse to both 1s or both 0s
        println!("{:02b}", s.measure(&[a, b]));
    }
}
