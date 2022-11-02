import pandas as pd
import numpy as np
import seaborn as sns
import matplotlib as mpl
import matplotlib.pyplot as plt
from mpl_toolkits import mplot3d

min_score = 200


df = pd.read_csv('../../assets/datasets/final.csv')
df = df[df["is_ox"] == False]
df = df[df["retention_time"] != 0]
df = df[df["score"] > min_score]
df = df.head(10000)
# sns.scatterplot(df_scatter_kd, x="mass", y="retention_time",)
sns.pairplot(df)
# fig = plt.figure()
# ax = plt.axes(projection='3d')
# ax.scatter3D(df["retention_time"], df["hs_hw"], df["hs_kd"])
plt.show()

# ax.
