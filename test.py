import momos_aedes
import numpy as np

arr = np.array([1, 2, 3, 4, 5], dtype=float)

result = momos_aedes.add_one(arr, 5.)

print(result)