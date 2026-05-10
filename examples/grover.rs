// This example uses Grover's algorithm to solve the following 4-SAT problem:
// (A xor B) and (A xor C) and not(C xor D) and D

use tiny_qsim::{Gate, QuantumSystem, Qubit};

struct SatProblem {
    a: Qubit,
    b: Qubit,
    c: Qubit,
    d: Qubit,
    a_xor_b: Qubit,
    a_xor_c: Qubit,
    not_c_xor_d: Qubit,
}

impl SatProblem {
    fn new(s: &mut QuantumSystem) -> Self {
        SatProblem {
            a: s.new_qubit(),
            b: s.new_qubit(),
            c: s.new_qubit(),
            d: s.new_qubit(),
            a_xor_b: s.new_qubit(),
            a_xor_c: s.new_qubit(),
            not_c_xor_d: s.new_qubit(),
        }
    }

    // Computes the results of the SAT problem from A, B, C, and D
    fn compute(&self, s: &mut QuantumSystem) {
        // A xor B
        s.apply_controlled(&Gate::x(), &[self.a], &[self.a_xor_b]);
        s.apply_controlled(&Gate::x(), &[self.b], &[self.a_xor_b]);

        // A xor C
        s.apply_controlled(&Gate::x(), &[self.a], &[self.a_xor_c]);
        s.apply_controlled(&Gate::x(), &[self.c], &[self.a_xor_c]);

        // not(C xor D)
        s.apply_controlled(&Gate::x(), &[self.c], &[self.not_c_xor_d]);
        s.apply_controlled(&Gate::x(), &[self.d], &[self.not_c_xor_d]);
        s.apply(&Gate::x(), &[self.not_c_xor_d]);
    }

    // The oracle reflects the quantum state across the superposition of all states
    // orthogonal to the goal. In this case, that means only flipping the amplitude
    // of the state where A, B, C, and D correctly solve the problem.
    fn oracle(&self, s: &mut QuantumSystem) {
        // Compute the results
        self.compute(s);

        // Flip the amplitude of the state where all result bits are 1.
        // Any of the result bits can be used as the target, the operation will be the same
        s.apply_controlled(
            &Gate::z(),
            &[self.d, self.a_xor_b, self.a_xor_c],
            &[self.not_c_xor_d],
        );

        // Compute again to reverse the computation
        self.compute(s);
    }
}

fn diffuser(s: &mut QuantumSystem, qubits: &[Qubit]) {
    // Grover's diffuser amplifies the result flipped by the oracle by reflecting
    // the quantum state across the equal superposition state
    for q in qubits {
        s.apply(&Gate::h(), &[*q]);
    }
    for q in qubits {
        s.apply(&Gate::x(), &[*q]);
    }
    s.apply_controlled(
        &Gate::z(),
        &qubits[..qubits.len() - 1],
        &[qubits[qubits.len() - 1]],
    );
    for q in qubits {
        s.apply(&Gate::x(), &[*q]);
    }
    for q in qubits {
        s.apply(&Gate::h(), &[*q]);
    }
}

fn main() {
    let mut s = QuantumSystem::new();
    let problem = SatProblem::new(&mut s);
    loop {
        s.reset();

        // Put each of the four variable qubits into an equal superposition of 1 and 0
        s.apply(&Gate::h(), &[problem.a, problem.b, problem.c, problem.d]);

        // The number of iterations to apply is given by pi/4 * sqrt(N)
        // where N is the number of possible states.
        // In this case N = 4 * 4: pi/4 * sqrt(16) = ~3
        for _ in 0..3 {
            problem.oracle(&mut s);
            diffuser(&mut s, &[problem.a, problem.b, problem.c, problem.d]);
        }

        // Measure the variable qubits
        let a = s.measure(&[problem.a]) == 1;
        let b = s.measure(&[problem.b]) == 1;
        let c = s.measure(&[problem.c]) == 1;
        let d = s.measure(&[problem.d]) == 1;
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
