import matplotlib.pyplot as plt
# import seaborn as sns
import os
import numpy as np
import pandas as pd
from sklearn.model_selection import train_test_split
import tensorflow_decision_forests as tfdf

max_length = 58

min_score = 100
split = 0.2

df = pd.read_csv("../../assets/datasets/unique.csv")
df = df[df["is_ox"] == False]
df = df[df["score"] > min_score]
df = df[df["retention_time"] != 0]

df["encoded"] = df["seq_full_ox"].apply(lambda x: [c for c in x])

df = df[["retention_time", "encoded", "is_ox"]]

train_df, test_df = train_test_split(df, test_size=split)

train_ds = tfdf.keras.pd_dataframe_to_tf_dataset(train_df, label="retention_time")
test_ds = tfdf.keras.pd_dataframe_to_tf_dataset(test_df, label="retention_time")

model = tfdf.keras.RandomForestModel()
model.fit(train_ds)
model.summary()
model.evaluate(test_ds)
model.save("../../assets/checkpoints/random_forest1")

# def one_hot_encode(seq):
#     mapping = dict(zip("ACDEFGHIKLMNPQRSTVWY", range(20)))
#     seq2 = [mapping[i] for i in seq]
#     matrix = np.eye(20)[seq2]
#     padded = np.pad(matrix, ((0, max_length - len(seq)), (0, 0)), 'constant')
#     return padded