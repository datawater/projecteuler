#include <stdio.h>
#include <immintrin.h>

__m128i simd_term(__m128i d_simd) {
    __m128i four = _mm_set1_epi32(4);
    __m128i one = _mm_set1_epi32(1);

    __m128i d_squared = _mm_mullo_epi32(d_simd, d_simd);
    __m128i term = _mm_mullo_epi32(four, d_squared);
    term = _mm_sub_epi32(term, _mm_mullo_epi32(four, d_simd));
    term = _mm_sub_epi32(term, one);
    return term;
}

void run_main() {
    int sq = 1001;
    int d = 1;
    int x = 0;

    while (2 * d - 1 < sq) {
        __m128i d_simd = _mm_set_epi32(d + 3, d + 2, d + 1, d);
        __m128i term_result = simd_term(d_simd);

        int terms[4];
        _mm_storeu_si128((__m128i*)terms, term_result);

        for (int i = 0; i < 4; i++) {
            x += terms[i];
        }

        d += 4;
    }

    x += sq * sq;

    printf("ANSWER P28: %d\n", x);
}
