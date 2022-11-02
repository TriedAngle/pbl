import matplotlib.pyplot as plt
# import seaborn as sns
import numpy as np
import pandas as pd
import tensorflow as tf
import tensorflow.keras as keras
from tensorflow.keras import layers
from sklearn.model_selection import train_test_split
physical_devices = tf.config.list_physical_devices('GPU')
tf.config.experimental.set_memory_growth(physical_devices[0], True)

print("Tensorflow: ", tf.__version__)
print("Numpy: ", np.__version__)
print("Pandas: ", pd.__version__)

np.set_printoptions(precision=3, suppress=True)

max_length = 58

min_score = 200
split = 0.2

df = pd.read_csv("../../assets/datasets/final.csv")
df = df[df["is_ox"] == False]
df = df[df["score"] > min_score]
df = df[df["retention_time"] != 0]

label = ["retention_time"]
features_str = ["encoded"]
# features = ["hs_kd", "hs_hw", "hs_c"]


def one_hot_encode(seq):
    mapping = dict(zip("ACDEFGHIKLMNPQRSTVWY", range(20)))
    seq2 = [mapping[i] for i in seq]
    matrix = np.eye(20)[seq2]
    padded = np.pad(matrix, ((0, max_length - len(seq)), (0, 0)), 'constant')
    return padded


df["encoded"] = df["seq_full"].apply(lambda x: one_hot_encode(x))

# features = df["encoded"]
features = df["hs_c"]
labels = df["retention_time"]

x_train, x_test, y_train, y_test = train_test_split(features, labels, test_size=split)

normalizer = layers.Normalization(input_shape=[1,], axis=None)
normalizer.adapt(x_train)
print(normalizer.mean.numpy())

model = keras.Sequential([
    normalizer,
    layers.Dense(units=1)
])

print(model.summary())
print(model.predict(x_train[:10]))

model.compile(
    optimizer=keras.optimizers.Adam(learning_rate=0.1),
    loss='mean_squared_error',
    # metrics=["accuracy"],
)

history = model.fit(x_train, y_train, epochs=100, verbose=2, validation_split=0.2)

hist = pd.DataFrame(history.history)
hist['epoch'] = history.epoch
print(hist.tail())

def plot_loss(history):
    plt.plot(history.history['loss'], label='loss')
    plt.plot(history.history['val_loss'], label='val_loss')
    plt.ylim([50, 100])
    plt.xlabel('Epoch')
    plt.ylabel('Error [MPG]')
    plt.legend()
    plt.grid(True)

plot_loss(history)
plt.show()

def plot_hsc(x, y):
    plt.scatter(x_train, y_train, label='Data')
    plt.plot(x, y, color='k', label='Predictions')
    plt.xlabel('Cornette')
    plt.ylabel('Retention Time')
    plt.legend()


x = tf.linspace(-2.6, 2.9, 251)
y = model.predict(x)
plot_hsc(x, y)
plt.show()

model.evaluate(x_test, y_test, verbose=2)
# x_train, x_test, y_train, y_test = train_test_split(features, labels, test_size=split)
# x_train = np.reshape(np.stack(x_train), (-1, 58*20))
# x_test = np.reshape(np.stack(x_test), (-1, 58*20))
# y_train = y_train.values
# y_test = y_test.values
#
# # Sequential API
#
# model = keras.Sequential(
#     [
#         layers.InputLayer(input_shape=(58*20)),
#         layers.Dense(128, activation="relu"),
#         layers.Dense(126, activation="relu"),
#         layers.Dense(units=1),
#     ]
# )
# print(model.summary())
# model.compile(
#     loss=keras.losses.SparseCategoricalCrossentropy(),
#     optimizer=keras.optimizers.Adam(learning_rate=0.1),
#     metrics=["accuracy"],
# )

# model.fit(x_train, y_train, batch_size=2, epochs=10, verbose=2)
# model.evaluate(x_test, y_test, batch_size=2, verbose=2)
