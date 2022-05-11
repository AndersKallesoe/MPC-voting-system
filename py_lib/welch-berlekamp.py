import numpy as np
import math
from fractions import Fraction
                
def welch_berlekamp(shares, t):
    shares_np = np.array([Fraction(share) for share in shares])
    alphas = np.arange(start=1, stop= len(shares_np)+1)
    alphas = np.array([Fraction(i) for i in range(1, len(shares_np)+1)])
    errors = math.floor((len(shares_np) - t - 1)/2)
    b = -(shares_np*alphas**errors)
    alpha_mat = -np.array([[alphas[j]**i for i in range(errors + t + 1)] for j in range(len(shares_np))])
    beta_mat = -np.multiply(alpha_mat[:,:errors], shares_np[:, np.newaxis])
    A = np.hstack((beta_mat, alpha_mat))
    coeffs = linalg_solve(A, b)
    error_coeffs = np.hstack((np.array([Fraction(1)]), coeffs[:np.shape(beta_mat)[1]][::-1]))
    Q_coeffs = coeffs[np.shape(beta_mat)[1]:][::-1]
    res = polynomial_division(Q_coeffs, error_coeffs)
    return res

def linalg_solve(A, b):
    mat = np.hstack((A, np.array([b]).T))
    col = 0
    for i in range (np.shape(A)[1]):
        try:
            mat[i] = mat[i] / mat[i][i]
            for j in range(np.shape(A)[0]):
                if j == i:
                    continue
                mat[j]= mat[j]-mat[i]* mat[j][i]
        except ZeroDivisionError:
            for row in mat:
                print(frac.numerator for frac in row)
            raise ZeroDivisionError
    return mat.T[-1]


def polynomial_division(A,B):
    A_ = np.copy(A)
    A_shape = np.shape(A)[0]
    B_shape = np.shape(B)[0]
    diff = A_shape - B_shape 
    B_ = np.append(np.array([Fraction(0)]*diff),B)
    res = np.array([Fraction(0)]*A_shape)
    for i in range(diff+1):
        res[B_shape-1+i] = A_[i]/B_[diff]
        B_temp = np.append(np.array([Fraction(0)]*i),np.append(B*res[B_shape-1+i],np.array([Fraction(0)]*(diff-i))))
        A_ = A_ - B_temp
    return [frac.numerator for frac in res]


