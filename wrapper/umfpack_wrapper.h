#ifndef UMFPACK_WRAPPER_H
#define UMFPACK_WRAPPER_H

#include "umfpack.h"

int umfpack_get_numeric(int n, const int *Ap, const int *Ai, const double *Ax,
                        void **Numeric);
int umfpack_free_numeric(void **Numeric);
int umfpack_solve(const int *Ap, const int *Ai, const double *Ax,
                  const double *b, double *x, void *Numeric);
int umfpack_load_numeric(void **Numeric, const char *filename);
int umfpack_save_numeric(void *Numeric, const char *filename);

#endif
