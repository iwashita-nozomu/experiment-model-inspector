// Synthetic constrained optimization; never executed by the inspector.
double residual = x - target;
double loss = residual * residual + lambda * x * x;
bool feasible = x >= 0.0;
x = x - step * (2.0 * residual + 2.0 * lambda * x);
