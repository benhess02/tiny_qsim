// This example demonstrates quantum entanglement

use tiny_qsim::{Gate, QuantumState};

fn main() {
    let mut s = QuantumState::new(2);
    for _ in 0..10 {
        // Reset both qubits to the 0 state
        s.reset();

        // Put qubit 0 into an equal superposition
        s.apply(&Gate::h(), &[0]);

        // Apply a controlled NOT gate to qubit 1, controlled by qubit 0.
        // This creates an entangled state where the only information known
        // about the two qubits is that they are equal, but their actual
        // values are still in superposition.
        s.apply_controlled(&Gate::x(), &[0], &[1]);

        // Measure the two qubits, demonstrating that they will always
        // either collapse to both 1s or both 0s
        println!("{:02b}", s.measure(&[1, 3]));
    }
}
