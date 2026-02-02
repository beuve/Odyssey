#include "umfpack_wrapper.h"
#include "SuiteSparse_config.h"
#include <stddef.h>
#include <stdio.h>

int umfpack_get_numeric(int n, const int *Ap, const int *Ai, const double *Ax,
                        void **Numeric) {
  void *Symbolic;
  double Info[UMFPACK_INFO];
  (void)umfpack_di_symbolic(n, n, Ap, Ai, Ax, &Symbolic, NULL, &Info);
  (void)umfpack_di_numeric(Ap, Ai, Ax, Symbolic, Numeric, NULL, &Info);
  umfpack_di_free_symbolic(&Symbolic);
  printf("######## %e\n", Info[UMFPACK_RCOND]);
  return 0;
}

int umfpack_free_numeric(void **Numeric) {
  umfpack_di_free_numeric(Numeric);
  return 0;
}

int umfpack_save_numeric(void *Numeric, const char *filename) {
  (void)umfpack_di_save_numeric(Numeric, filename);
  return 0;
}

int umfpack_load_numeric(void **Numeric, const char *filename) {
  (void)umfpack_di_load_numeric(Numeric, filename);
  return 0;
}

int umfpack_solve(const int *Ap, const int *Ai, const double *Ax,
                  const double *b, double *x, void *Numeric) {
  double Info[UMFPACK_INFO];
  (void)umfpack_di_solve(UMFPACK_A, Ap, Ai, Ax, x, b, Numeric, NULL, &Info);
  printf("######## %e\n", Info[UMFPACK_RCOND]);
  return 0;
}
