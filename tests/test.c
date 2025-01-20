#include "../very-noisy-qubit-api/include/very_noisy_qubit.h"
#include <assert.h>
#include <stdio.h>

int main() {
  UnreliableQubit qubit = {.noise_level = 0.0, .times_measured = 0};

  // Test null pointer
  assert(apply_gate(NULL, GATE_X) == 0);

  // Test X gate
  assert(apply_gate(&qubit, GATE_X) == 1);
  assert(qubit.noise_level == 1.0);

  // Test H gate
  assert(apply_gate(&qubit, GATE_H) == 1);
  assert(qubit.noise_level == 2.0);

  // Test Z gate
  assert(apply_gate(&qubit, GATE_Z) == 1);
  assert(qubit.noise_level == 3.0);

  // Test measurement
  int measurement_result = measure_qubit(&qubit);
  assert(measurement_result == 0 || measurement_result == 1);
  assert(qubit.times_measured == 1);
  assert(qubit.noise_level == measurement_result);

  // Test invalid gate
  assert(apply_gate(&qubit, 999) == 0);

  printf("All tests passed!\n");
  return 0;
}
