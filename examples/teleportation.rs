// This example demonstrates using quantum teleportation to
// transfer the complete quantum state of a qubit

use tiny_qsim::{Gate, QuantumSystem};

fn main() {
    let mut s = QuantumSystem::new();

    // *** SETUP ***
    // Qubits A and B are setup as an entangled pair in the Bell state.
    // A is given to the sender, and B to the reciever.
    // This can happen at any time before the teleportation.
    let a = s.new_qubit();
    let b = s.new_qubit();
    s.apply(&Gate::h(), &[a]);
    s.apply_controlled(&Gate::x(), &[a], &[b]);

    // *** SENDER ***
    // Q is the qubit to be teleported. The state of Q does not need to
    // be known to either side.
    // For this example, Q and C are entangled in a Bell state.
    // C is an external qubit which also does not need to be
    // known by the sender to reciever.
    let q = s.new_qubit();
    let c = s.new_qubit();
    s.apply(&Gate::h(), &[q]);
    s.apply_controlled(&Gate::x(), &[q], &[c]);

    // The sender applies a CNOT to Q and A, then a hadamard gate to A.
    s.apply_controlled(&Gate::x(), &[q], &[a]);
    s.apply(&Gate::h(), &[q]);

    // The sender measures A and Q then transmits the results as
    // two classical bits to the reciever.
    let measured_a = s.measure(&[a]) == 1;
    let measured_q = s.measure(&[q]) == 2;

    // *** RECIEVER ***
    // After the measurement on A by the sender, the qubit B is now in
    // one of four possible quantum states. The two classical bits
    // transmitted by the sender encode this information and can be
    // used to transform B into original quantum state of Q.
    if measured_a {
        s.apply(&Gate::x(), &[b]);
    }
    if measured_q {
        s.apply(&Gate::z(), &[b]);
    }

    // B now has the exact same quantum state that Q had originally,
    // including correlations with other qubits.
    // In this case, B and C will always have the same value since
    // they are entangled, as Q and C were originally.
    println!("B = {}", s.measure(&[b]));
    println!("C = {}", s.measure(&[c]));
}
