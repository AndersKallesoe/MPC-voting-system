import numpy as np
from fractions import Fraction

def echelon(A, b):
    if np.shape(A)[0] != np.shape(A)[1]:
        return

    if np.shape(A)[0] != np.shape(b)[0]:
        return
         
    zero_rows = []
    mat = np.hstack((A, b))
    col = 0
    for i in range (np.shape(A)[1]):
        mat[i] = mat[i] / mat[i][i]
        print(mat)
        for j in range(np.shape(A)[0]):
            if j == i:
                continue
            mat[j]= mat[j]-mat[i]* mat[j][i]
            print(mat)
                
    

echelon(np.array([[Fraction(1), Fraction(2), Fraction(3)], 
                  [Fraction(2), Fraction(2), Fraction(6)], 
                  [Fraction(7), Fraction(1), Fraction(4)]]), np.array([[Fraction(4)], [Fraction(3)], [Fraction(5)]]))

print(np.linalg.solve(np.array([[1, 2, 3], [1, 2, 3], [8, 8, 8]]), np.array([4, 4, 4])))
        
    
    
    



    
    