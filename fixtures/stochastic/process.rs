// Synthetic recurrence. The law of supplied noise is intentionally undeclared.
let drift = -theta * state;
let next_state = state + drift * dt + sigma * supplied_noise;
let error = (next_state - reference).abs();
