use rand::{SeedableRng, rngs::StdRng};
use tiny_qsim::{Gate, QuantumSystem, Qubit};

pub struct State {
    quantum_system: QuantumSystem<StdRng>,
    qubits: Vec<Qubit>,
    controls: Vec<Qubit>,
}

#[unsafe(no_mangle)]
pub extern "C" fn create_state(qubits: usize) -> *mut State {
    let mut quantum_system = QuantumSystem::new(StdRng::seed_from_u64(123456));
    let qubits = vec![quantum_system.new_qubit(); qubits];
    Box::into_raw(Box::new(State {
        quantum_system,
        qubits,
        controls: Vec::new(),
    }))
}

#[unsafe(no_mangle)]
pub extern "C" fn add_control(state_ptr: *mut State, qubit_index: usize) {
    let state = unsafe { &mut *state_ptr };
    state.controls.push(state.qubits[qubit_index]);
}

#[unsafe(no_mangle)]
pub extern "C" fn get_probability(state_ptr: *mut State, qubit_index: usize, value: usize) -> f32 {
    let state = unsafe { &mut *state_ptr };
    state
        .quantum_system
        .probability(&[state.qubits[qubit_index]], value)
}

#[unsafe(no_mangle)]
pub extern "C" fn get_concurrence(
    state_ptr: *mut State,
    qubit_index_a: usize,
    qubit_index_b: usize,
) -> f32 {
    let state = unsafe { &mut *state_ptr };
    state
        .quantum_system
        .concurrence(state.qubits[qubit_index_a], state.qubits[qubit_index_b])
}

fn apply(state_ptr: *mut State, gate: &Gate, target_qubit_index: usize) {
    let state = unsafe { &mut *state_ptr };
    state.quantum_system.apply_controlled(
        gate,
        &state.controls,
        &[state.qubits[target_qubit_index]],
    );
    state.controls.clear();
}

#[unsafe(no_mangle)]
pub extern "C" fn apply_h(state_ptr: *mut State, target_qubit_index: usize) {
    apply(state_ptr, &Gate::h(), target_qubit_index);
}
