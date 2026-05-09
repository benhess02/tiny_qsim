// This example uses Grover's algorithm to solve the following 4-SAT problem:
// (A xor B) and (A xor C) and not(C xor D) and D

use tiny_qsim::{Gate, QuantumState};

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const D: usize = 3;
const A_XOR_B: usize = 4;
const A_XOR_C: usize = 5;
const NOT_C_XOR_D: usize = 6;

// The oracle reflects the quantum state across the superposition of all states
// orthogonal to the goal. In this case, that means only flipping the amplitude
// of the state where A, B, C, and D correctly solve the problem.
fn oracle(s: &mut QuantumState) {
    // A xor B
    s.apply_controlled(&Gate::x(), &[A], &[A_XOR_B]);
    s.apply_controlled(&Gate::x(), &[B], &[A_XOR_B]);

    // A xor C
    s.apply_controlled(&Gate::x(), &[A], &[A_XOR_C]);
    s.apply_controlled(&Gate::x(), &[C], &[A_XOR_C]);

    // not(C xor D)
    s.apply_controlled(&Gate::x(), &[C], &[NOT_C_XOR_D]);
    s.apply_controlled(&Gate::x(), &[D], &[NOT_C_XOR_D]);
    s.apply(&Gate::x(), &[NOT_C_XOR_D]);

    // Flip the amplitude of the state where all result bits are 1.
    // Any of the result bits can be used as the target, the operation will be the same
    s.apply_controlled(&Gate::z(), &[D, A_XOR_B, A_XOR_C], &[NOT_C_XOR_D]);

    // Uncompute to reset result bits by applying the same operations again
    // A xor B
    s.apply_controlled(&Gate::x(), &[A], &[A_XOR_B]);
    s.apply_controlled(&Gate::x(), &[B], &[A_XOR_B]);

    // A xor C
    s.apply_controlled(&Gate::x(), &[A], &[A_XOR_C]);
    s.apply_controlled(&Gate::x(), &[C], &[A_XOR_C]);

    // not(C xor D)
    s.apply_controlled(&Gate::x(), &[C], &[NOT_C_XOR_D]);
    s.apply_controlled(&Gate::x(), &[D], &[NOT_C_XOR_D]);
    s.apply(&Gate::x(), &[NOT_C_XOR_D]);
}

fn diffuser(s: &mut QuantumState) {
    // Grover's diffuser amplifies the result flipped by the oracle by reflecting
    // the quantum state across the equal superposition state
    for i in A..=D {
        s.apply(&Gate::h(), &[i]);
    }
    for i in A..=D {
        s.apply(&Gate::x(), &[i]);
    }
    s.apply_controlled(&Gate::z(), &[A, B, C], &[D]);
    for i in A..=D {
        s.apply(&Gate::x(), &[i]);
    }
    for i in A..=D {
        s.apply(&Gate::h(), &[i]);
    }
}

fn main() {
    let mut s = QuantumState::new(7);
    loop {
        s.reset();

        // Put each of the four variable qubits into an equal superposition of 1 and 0
        for i in A..=D {
            s.apply(&Gate::h(), &[i]);
        }

        // The number of iterations to apply is given by pi/4 * sqrt(N)
        // where N is the number of possible states.
        // In this case N = 4 * 4: pi/4 * sqrt(16) = ~3
        for _ in 0..3 {
            oracle(&mut s);
            diffuser(&mut s);
        }

        // Measure the variable qubits
        let a = s.measure(&[0]) == 1;
        let b = s.measure(&[1]) == 1;
        let c = s.measure(&[2]) == 1;
        let d = s.measure(&[3]) == 1;
        println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);

        // Check the answer
        if (a ^ b) && (a ^ c) && !(c ^ d) && d {
            println!("Correct!");
            break;
        } else {
            println!("Incorrect, trying again...");
        }
    }
}
