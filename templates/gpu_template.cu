#include <cuda_runtime.h>
#include <stdio.h>

#define CUDA_CHECK(call) \
    do { \
        cudaError_t err = call; \
        if (err != cudaSuccess) { \
            fprintf(stderr, "CUDA error at %s:%d — %s\n", \
                    __FILE__, __LINE__, cudaGetErrorString(err)); \
            exit(1); \
        } \
    } while(0)

__global__ void kernel(float* d_out, const float* d_in, int n) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < n) {
        d_out[idx] = d_in[idx];
    }
}

int main() {
    const int N = 1 << 20;
    const int THREADS = 256;
    const int BLOCKS = (N + THREADS - 1) / THREADS;

    float *h_in = new float[N];
    float *h_out = new float[N];
    float *d_in, *d_out;

    for (int i = 0; i < N; i++) h_in[i] = (float)i;

    CUDA_CHECK(cudaMalloc(&d_in,  N * sizeof(float)));
    CUDA_CHECK(cudaMalloc(&d_out, N * sizeof(float)));
    CUDA_CHECK(cudaMemcpy(d_in, h_in, N * sizeof(float), cudaMemcpyHostToDevice));

    kernel<<<BLOCKS, THREADS>>>(d_out, d_in, N);
    CUDA_CHECK(cudaGetLastError());
    CUDA_CHECK(cudaDeviceSynchronize());

    CUDA_CHECK(cudaMemcpy(h_out, d_out, N * sizeof(float), cudaMemcpyDeviceToHost));

    cudaFree(d_in);
    cudaFree(d_out);
    delete[] h_in;
    delete[] h_out;
    return 0;
}
