// Synthetic CUDA update, inspected as text only.
__global__ void update(float* state, const float* grad, float step) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    state[i] = state[i] - step * grad[i];
}
