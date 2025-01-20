#ifndef VERY_NOISY_QUBIT_H
#define VERY_NOISY_QUBIT_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Available quantum gates (that probably won't work)
 */
typedef enum {
  GATE_X, // Probably flips the bit... or doesn't
  GATE_H, // Might create superposition, who knows?
  GATE_Z, // Rotates the phase, if we're lucky
} NoiseGate;

/**
 * @brief The world's most unreliable qubit
 */
typedef struct {
  double noise_level; // How much cosmic radiation we've absorbed
  int times_measured; // Number of times we've disturbed this poor qubit
} UnreliableQubit;

/**
 * @brief Attempts to apply a quantum gate (results may vary wildly)
 *
 * @param qubit The unfortunate qubit to manipulate
 * @param gate The gate we'll try to apply
 * @return int Returns 1 if we think it worked (it didn't)
 */
int apply_gate(UnreliableQubit *qubit, NoiseGate gate);

/**
 * @brief Collapses the wavefunction (and your hopes)
 *
 * @param qubit The qubit to measure
 * @return int The measured value (0 or 1) or -1 if the qubit is invalid
 */
int measure_qubit(UnreliableQubit *qubit);

#ifdef __cplusplus
}
#endif

#endif /* VERY_NOISY_QUBIT_H */
