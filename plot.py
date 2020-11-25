import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

df = pd.read_csv("output.txt", sep=" ")
print(df)
pvt = df.pivot(index='n', columns='algo', values='total')
pvt.plot(marker='o')
plt.show()
