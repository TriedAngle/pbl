import pandas as pd
import numpy as np
from sklearn import preprocessing, svm
from sklearn.model_selection import train_test_split
from sklearn.linear_model import LinearRegression

min_score = 200
split = 0.2

df = pd.read_csv('../../assets/datasets/final.csv')
df = df[df["is_ox"] == False]
df = df[df["retention_time"] != 0]
df["hs_hw"] = -df["hs_hw"]
df = df[df["score"] > min_score]

tdf = df[["retention_time", "hs_kd", "hs_hw", "hs_c"]]
# tdf = df[["retention_time", "seq_full"]]

X = np.array(tdf.drop(columns="retention_time"))
y = np.array(tdf["retention_time"])
# X = preprocessing.scale(X)

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=split)

clf = LinearRegression(n_jobs=-1)
# clf = svm.SVR()
clf.fit(X_train, y_train)

acc = clf.score(X_test, y_test)

print("min score: ", min_score)
print("Data Size: ", tdf.shape[0])
print("Split: ", split)
print("Accuracy:", acc)
# print("X: ", X)
# print("shape X: ", np.shape(np.transpose(X)))
# print("X[0]: ", X[0])
# print("y: ", y)
# test_df = pd.DataFrame({'label': y, 'mass': X[0]}, columns=["label", "mass"])
# print(test_df)