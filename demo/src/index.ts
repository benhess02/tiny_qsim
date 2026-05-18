interface CoreExports {
    create_state(qubits: number): number;
    add_control(state_ptr: number, qubit_index: number): void;
    get_probability(state_ptr: number, qubit_index: number, value: number): number;
    get_concurrence(state_ptr: number, qubit_index_a: number, qubit_index_b: number): number;
    apply_h(state_ptr: number, target_qubit_index: number): void;
    apply_x(state_ptr: number, target_qubit_index: number): void;
    apply_y(state_ptr: number, target_qubit_index: number): void;
    apply_z(state_ptr: number, target_qubit_index: number): void;
    apply_s(state_ptr: number, target_qubit_index: number): void;
    apply_t(state_ptr: number, target_qubit_index: number): void;
}

async function run() {
    let { instance } = await WebAssembly.instantiateStreaming(fetch("core.wasm"));
    let exports = <CoreExports><any>instance.exports;
    let state = exports.create_state(5);
    exports.apply_h(state, 0);
    exports.add_control(state, 0);
    exports.apply_x(state, 1);
    console.log(exports.get_concurrence(state, 0, 1));
}

run();