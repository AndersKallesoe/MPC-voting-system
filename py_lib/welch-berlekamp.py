import numpy as np
import math



shares = [7, 11, 15, 19, 23, 27, 31]

def welch_berlekamp(shares, t, p):
    shares_np = np.array(shares)
    alphas = np.arange(start=1, stop= len(shares_np)+1)
    errors = math.floor((len(shares_np) - t - 1)/2)
    return welch_berlekamp_inner(shares_np, alphas, errors, p, t)


def welch_berlekamp_inner(shares, alphas, error_degree, p, t):
    b = -(shares*alphas**error_degree) % p
    alpha_mat = -np.array([[alphas[j]**i for i in range(error_degree + t + 1)] for j in range(len(shares))]) % p
    beta_mat = -np.multiply(alpha_mat[:,:error_degree], shares[:, np.newaxis]) % p
    A = np.hstack((beta_mat, alpha_mat))
    mat = linalg_solve(A, b, p)
    diagonal = mat[:,:-1].diagonal()
    ones = np.ones(np.shape(A)[1])
    if not all(diagonal==ones) and error_degree != 0:
        return welch_berlekamp_inner(shares, alphas, error_degree - 1, p, t)
    coeffs = mat.T[-1]
    error_coeffs = np.hstack((np.array([1]), coeffs[:np.shape(beta_mat)[1]][::-1]))
    Q_coeffs = coeffs[np.shape(beta_mat)[1]:][::-1]
    res = polynomial_division(Q_coeffs, error_coeffs,p)
    return res # check remainder

def linalg_solve(A, b, p):
    mat = np.hstack((A, np.array([b]).T)) % p
    for i in range (np.shape(A)[1]):
        if(mat[i][i]==0):
            mat = np.vstack((mat[:i],mat[i+1:],mat[i]))
        mat[i] = (mat[i] * pow(int(mat[i][i]),p-2,p)) % p
        for j in range(np.shape(A)[0]):
            if j == i:
                continue
            mat[j]= (mat[j]-mat[i]* mat[j][i])%p
    return mat


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


welch_berlekamp(shares, 2, 37)