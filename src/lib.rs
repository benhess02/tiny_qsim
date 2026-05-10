use num::{Complex, complex::ComplexFloat};

#[derive(Clone)]
pub struct Gate {
    values: Vec<Complex<f32>>,
    size: usize,
}

impl Gate {
    /// Creates a gate from the values of a unitary matrix
    pub fn new<V: Into<Vec<Complex<f32>>>>(values: V) -> Self {
        let _values = values.into();
        if _values.len() < 2 || _values.len().count_ones() != 1 {
            panic!("invalid op matrix size");
        }
        let qubits = _values.len().trailing_zeros() as usize / 2;
        Self {
            values: _values,
            size: 1 << qubits,
        }
    }

    /// Returns the n-th row of this gate's unitary matrix
    pub fn row(&self, n: usize) -> &[Complex<f32>] {
        let base = n * self.size;
        &self.values[base..(base + self.size)]
    }

    /// Returns the rank of this gate's unitary matrix.
    /// This will always equal 2^n where n is the number of target qubits
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the number of target qubits this gate operates on
    pub fn qubits(&self) -> usize {
        self.size.trailing_zeros() as usize
    }

    /// Hadamard gate
    pub fn h() -> Self {
        let inv_sqrt_2 = Complex::new(1. / 2f32.sqrt(), 0.);
        Self::new([inv_sqrt_2, inv_sqrt_2, inv_sqrt_2, -inv_sqrt_2])
    }

    /// Pauli X (NOT) gate
    pub fn x() -> Self {
        Self::new([Complex::ZERO, Complex::ONE, Complex::ONE, Complex::ZERO])
    }

    /// Pauli Y gate
    pub fn y() -> Self {
        Self::new([Complex::ZERO, -Complex::I, Complex::I, Complex::ZERO])
    }

    /// Pauli Z gate
    pub fn z() -> Self {
        Self::new([Complex::ONE, Complex::ZERO, Complex::ZERO, -Complex::ONE])
    }

    /// Phase shift gate
    pub fn p(phi: f32) -> Self {
        Self::new([
            Complex::ONE,
            Complex::ZERO,
            Complex::ZERO,
            Complex::new(phi.cos(), phi.sin()),
        ])
    }

    /// S phase shift gate
    pub fn s() -> Self {
        Self::new([Complex::ONE, Complex::ZERO, Complex::ZERO, Complex::I])
    }

    // T phase shift gate
    pub fn t() -> Self {
        Self::p(std::f32::consts::PI / 4.)
    }

    /// Swap gate
    pub fn swap() -> Self {
        Self::new([
            Complex::ONE,
            Complex::ZERO,
            Complex::ZERO,
            Complex::ZERO,
            Complex::ZERO,
            Complex::ZERO,
            Complex::ONE,
            Complex::ZERO,
            Complex::ZERO,
            Complex::ONE,
            Complex::ZERO,
            Complex::ZERO,
            Complex::ZERO,
            Complex::ZERO,
            Complex::ZERO,
            Complex::ONE,
        ])
    }
}

#[derive(Clone, Copy)]
pub struct Qubit {
    index: usize,
}

pub struct QuantumSystem {
    states: Vec<Complex<f32>>,
}

impl QuantumSystem {
    pub fn new() -> Self {
        Self {
            states: vec![Complex::ONE],
        }
    }

    pub fn new_qubit(&mut self) -> Qubit {
        self.states.resize(self.states.len() * 2, Complex::ZERO);
        Qubit {
            index: self.states.len().trailing_zeros() as usize - 1,
        }
    }

    /// Returns the number of basis states
    /// This will always equal 2^n where n is the number of qubits.
    pub fn count_states(&self) -> usize {
        self.states.len()
    }

    /// Returns the number of qubits
    pub fn count_qubits(&self) -> usize {
        self.count_states().trailing_zeros() as usize
    }

    /// Returns the complex probability amplitudes for all basis states
    pub fn amplitudes(&self) -> &[Complex<f32>] {
        &self.states
    }

    /// Resets all qubits to the 0 state
    pub fn reset(&mut self) {
        self.states.fill(Complex::ZERO);
        self.states[0] = Complex::ONE;
    }

    /// Applies the specified gate to the current quantum state with control qubits
    pub fn apply_controlled(&mut self, gate: &Gate, controls: &[Qubit], targets: &[Qubit]) {
        // The complete matrix to apply is the tensor product of the provided
        // unitary matrix (for the parameter qubits) and the identity matrix
        // (for the unaffected qubits). This means the resulting matrix will be divided
        // into sparse, independent blocks and does not need to be fully computed.
        // Instead, the matrix product of the unitary matrix and each independent
        // block of the state is computed separately.

        if gate.qubits() == 1 && targets.len() > 1 {
            for qubit in targets {
                self.apply_controlled(gate, controls, &[*qubit]);
            }
            return;
        }

        if gate.qubits() != targets.len() {
            panic!("number of targets does not match size of multi-qubit gate");
        }

        let mut control_mask = 0;
        for q in controls {
            if (control_mask & (1 << q.index)) != 0 {
                panic!("duplicate control qubit");
            }
            control_mask |= 1 << q.index;
        }

        let mut target_mask = 0;
        for q in targets {
            if (target_mask & (1 << q.index)) != 0 {
                panic!("duplicate target qubit");
            }
            if (control_mask & (1 << q.index)) != 0 {
                panic!("control qubit cannot be used as target");
            }
            target_mask |= 1 << q.index;
        }

        // Calculate the offsets of each row or column to the corresponding state indices in a block
        let mut block_offsets = Vec::with_capacity(gate.size());
        for r in 0..gate.size() {
            let mut block_offset: usize = 0;
            for i in 0..targets.len() {
                block_offset |= ((r >> i) & 1) << targets[targets.len() - 1 - i].index;
            }
            block_offsets.push(block_offset);
        }

        // Apply the matrix to each block
        let mut block_result: Vec<Complex<f32>> = vec![Complex::ZERO; gate.size()];
        let mut i = 0;
        while i < self.states.len() {
            if (!i & control_mask) == 0 {
                let block_index = i & !target_mask;

                // Multiply the unitary matrix with the current block of state
                block_result.fill(Complex::ZERO);
                for r in 0..gate.size() {
                    for c in 0..gate.size() {
                        let index = block_index | block_offsets[c];
                        block_result[r] += gate.row(r)[c] * self.states[index];
                    }
                }

                // Write the result of the matrix multiplication back into the state
                for r in 0..gate.size() {
                    let index = block_index | block_offsets[r];
                    self.states[index] = block_result[r];
                }
            }
            i = ((i | target_mask) + 1) & !target_mask;
        }
    }

    /// Applies the specified gate to the current quantum state
    pub fn apply(&mut self, gate: &Gate, targets: &[Qubit]) {
        self.apply_controlled(gate, &[], targets);
    }

    /// Measures the values of the provided qubits, collapsing their wavefunction
    pub fn measure(&mut self, qubits: &[Qubit]) -> usize {
        let mut mask = 0;
        for q in qubits {
            mask |= 1 << q.index;
        }

        // Choose a result
        let r: f32 = rand::random();
        let mut sum = 0.;
        let mut collapsed = 0;
        for i in 0..self.states.len() {
            sum += self.states[i].abs().powi(2);
            if r <= sum {
                collapsed = i & mask;
                break;
            }
        }

        // Simulate wave function collapse by zeroing the probabilities for other results
        let mut p = 0.;
        for i in 0..self.states.len() {
            if (i & mask) == collapsed {
                p += self.states[i].abs().powi(2);
            } else {
                self.states[i] = Complex::ZERO;
            }
        }

        // Normalize the state vector to return the total probability to 1
        let mag = p.sqrt(); // Magnitude of the entire state
        for i in 0..self.states.len() {
            self.states[i] /= mag;
        }

        let mut result = 0;
        for i in 0..qubits.len() {
            result |= ((collapsed >> qubits[i].index) & 1) << i;
        }
        return result;
    }

    /// Returns the probability of measuring the provided qubits in the provided state
    pub fn probability(&self, qubits: &[Qubit], state: usize) -> f32 {
        let mut mask = 0;
        for q in qubits {
            mask |= 1 << q.index;
        }

        let mut result = 0.;
        for i in 0..self.states.len() {
            if (i & mask) == state {
                result += self.states[i].abs().powi(2);
            }
        }
        return result;
    }
}
