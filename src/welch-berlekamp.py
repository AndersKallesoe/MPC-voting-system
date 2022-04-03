from distutils.log import error
import numpy as np
import math

test = np.array([1,8,5,7])

def welch_berlekamp(shares, t):
    alphas = np.arange(start=1, stop= len(shares)+1)
    errors = math.floor((len(shares) - t - 1)/2)
    alphas = np.arange( stop=len(shares))
    b = -(shares*alphas**errors)
    alpha_mat = -np.array([[alphas[j]**i for i in range(errors + t + 1)] for j in range(len(shares))])
    beta_mat = -np.multiply(alpha_mat[:,:errors], test[:, np.newaxis])
    A = np.hstack((beta_mat, alpha_mat))
    coeffs = np.linalg.solve(A, b)
    #a = build_coefficients(shares, alphas)
    error_coeffs = np.hstack((np.array([1]), coeffs[:np.shape(beta_mat)[1]][::-1]))
    Q_coeffs = coeffs[np.shape(beta_mat)[1]:][::-1]
    print(error_coeffs)
    print(Q_coeffs)
    print(np.polydiv(Q_coeffs, error_coeffs))

def build_coefficients(shares, alphas): 
    for i in range :
        shares
    print("")

welch_berlekamp(test, 1)

# [b_1 b_1a_1 b_1(a_1^2) -1 -a_1 -a_1^2 ]

# 1+2x_1
# 1, 3, 5, 7