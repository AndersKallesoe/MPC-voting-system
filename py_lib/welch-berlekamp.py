import numpy as np
import math
from fractions import Fraction
            

# Coefficients [1, 5] shares [6, 4, 2, 0]
# Coefficients [1, 2] shares [3, 5, 0, 2]
# Coefficients [0, 1] shares [1, 2, 3, 4]
# Coefficients [0, 2] shares [2, 4, 6, 1]
# Coefficients [0, 1] shares [1, 2, 3, 4]

# def welch_berlekamp(shares, t, p):
#     shares_np = np.array([Fraction(share) for share in shares])
#     alphas = np.arange(start=1, stop= len(shares_np)+1)
#     alphas = np.array([Fraction(i) for i in range(1, len(shares_np)+1)])
#     errors = math.floor((len(shares_np) - t - 1)/2)
#     b = -(shares_np*alphas**errors)
#     alpha_mat = -np.array([[alphas[j]**i for i in range(errors + t + 1)] for j in range(len(shares_np))])
#     beta_mat = -np.multiply(alpha_mat[:,:errors], shares_np[:, np.newaxis])
#     A = np.hstack((beta_mat, alpha_mat))
#     coeffs = linalg_solve(A, b, p)
#     error_coeffs = np.hstack((np.array([Fraction(1)]), coeffs[:np.shape(beta_mat)[1]][::-1]))
#     Q_coeffs = coeffs[np.shape(beta_mat)[1]:][::-1]
#     res = polynomial_division(Q_coeffs, error_coeffs)
#     return res

# def linalg_solve(A, b, p):
#      mat = np.hstack((A, np.array([b]).T))
#      for i in range (np.shape(A)[1]):
#          try:
#              mat[i] = (mat[i] * pow(mat[i][i],p-2,p)) % p
#              for j in range(np.shape(A)[0]):
#                  if j == i:
#                      continue
#                  mat[j]= (mat[j]-mat[i]* mat[j][i])%p
#          except ZeroDivisionError:
#              for row in mat:
#                  print(frac.numerator for frac in row)
#              raise ZeroDivisionError
#      return mat.T[-1]


def welch_berlekamp(shares, t, p):
    shares_np = np.array(shares)
    alphas = np.arange(start=1, stop= len(shares_np)+1)
    errors = math.floor((len(shares_np) - t - 1)/2)
    b = -(shares_np*alphas**errors) % p
    alpha_mat = -np.array([[alphas[j]**i for i in range(errors + t + 1)] for j in range(len(shares_np))]) % p
    beta_mat = -np.multiply(alpha_mat[:,:errors], shares_np[:, np.newaxis]) % p
    A = np.hstack((beta_mat, alpha_mat))
    coeffs = linalg_solve(A, b, p)
    error_coeffs = np.hstack((np.array([1]), coeffs[:np.shape(beta_mat)[1]][::-1]))
    Q_coeffs = coeffs[np.shape(beta_mat)[1]:][::-1]
    #print(error_coeffs)
    #print(Q_coeffs)
    res = polynomial_division(Q_coeffs, error_coeffs,p)
    return res

def linalg_solve(A, b, p):
    mat = np.hstack((A, np.array([b]).T)) % p
    print(mat)
    print()
    mat[0] = mat[0]+mat[1] % p
    for i in range (np.shape(A)[1]):
        mat[i] = (mat[i] * pow(int(mat[i][i]),p-2,p)) % p
        for j in range(np.shape(A)[0]):
            if j == i:
                continue
            mat[j]= (mat[j]-mat[i]* mat[j][i])%p
        print(mat)
        print()
    print(mat.T[-1])
    return mat.T[-1]



def polynomial_division(A,B,p):
    A_ = np.copy(A)
    A_shape = np.shape(A)[0]
    B_shape = np.shape(B)[0]
    diff = A_shape - B_shape 
    B_ = np.append(np.array([0]*diff),B)
    res = np.array([0]*A_shape)
    for i in range(diff+1):
        res[B_shape-1+i] = A_[i]/B_[diff] %p
        B_temp = np.append(np.array([0]*i),np.append(B*res[B_shape-1+i],np.array([0]*(diff-i)))) %p
        A_ = A_ - B_temp %p
    return res % p



shares = [6,3,0,4]
print(welch_berlekamp(shares, 1, 7))


