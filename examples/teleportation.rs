// This example demonstrates using quantum teleportation to
// transfer the complete quantum state of a qubit

use tiny_qsim::{Gate, QuantumState};

fn main() {
    const A: usize = 0;
    const B: usize = 1;
    const Q: usize = 2;
    const C: usize = 3;
    let mut s = QuantumState::new(4);

    // *** SETUP ***
    // Qubits A and B are setup as an entangled pair in the Bell state.
    // A is given to the sender, and B to the reciever.
    // This can happen at any time before the teleportation.
    s.apply(&Gate::h(), &[A]);
    s.apply_controlled(&Gate::x(), &[A], &[B]);

    // *** SENDER ***
    // Q is the qubit to be teleported. The state of Q does not need to
    // be known to either side.
    // For this example, Q and C are entangled in a Bell state.
    // C is an external qubit which also does not need to be
    // known by the sender to reciever.
    s.apply(&Gate::h(), &[Q]);
    s.apply_controlled(&Gate::x(), &[Q], &[C]);

    // The sender applies a CNOT to Q and A, then a hadamard gate to A.
    s.apply_controlled(&Gate::x(), &[Q], &[A]);
    s.apply(&Gate::h(), &[Q]);

    // The sender measures A and Q then transmits the results as
    // two classical bits to the reciever.
    let measured_a = s.measure(&[A]) == 1;
    let measured_q = s.measure(&[Q]) == 2;

    // *** RECIEVER ***
    // After the measurement on A by the sender, the qubit B is now in
    // one of four possible quantum states. The two classical bits
    // transmitted by the sender encode this information and can be
    // used to transform B into original quantum state of Q.
    if measured_a {
        s.apply(&Gate::x(), &[B]);
    }
    if measured_q {
        s.apply(&Gate::z(), &[B]);
    }

    // B now has the exact same quantum state that Q had originally,
    // including correlations with other qubits.
    // In this case, B and C will always have the same value since
    // they are entangled, as Q and C were originally.
    println!("B = {}", s.measure(&[B]));
    println!("C = {}", s.measure(&[C]));
}
