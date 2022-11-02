import matplotlib.pyplot as plt
# import seaborn as sns
import os
import numpy as np
import pandas as pd
import tensorflow as tf
import tensorflow.keras as keras
from tensorflow.keras import layers
from tensorflow.keras import regularizers
from sklearn.model_selection import train_test_split

physical_devices = tf.config.list_physical_devices('GPU')
tf.config.experimental.set_memory_growth(physical_devices[0], True)

max_length = 58

min_score = 100
split = 0.2

# n2
# layers.Dense(1400, activation="relu"),
# layers.Dropout(0.4),
# layers.Dense(800, activation="relu"),
# layers.Dense(1),

checkpoint_path = "../../assets/checkpoints/nn5/cp.ckpt"
checkpoint_dir = os.path.dirname(checkpoint_path)


df = pd.read_csv("../../assets/datasets/final.csv")
df = df[df["is_ox"] == False]
df = df[df["score"] > min_score]
df = df[df["retention_time"] != 0]


def one_hot_encode(seq):
    mapping = dict(zip("ACDEFGHIKLMNPQRSTVWY", range(20)))
    seq2 = [mapping[i] for i in seq]
    matrix = np.eye(20)[seq2]
    padded = np.pad(matrix, ((0, max_length - len(seq)), (0, 0)), 'constant')
    return padded


df["encoded"] = df["seq_full"].apply(lambda x: one_hot_encode(x))

features = df["encoded"]
labels = df["retention_time"]

cp_callback = tf.keras.callbacks.ModelCheckpoint(filepath=checkpoint_path,
                                                 save_weights_only=True,
                                                 verbose=1)

x_train, x_test, y_train, y_test = train_test_split(features, labels, test_size=split)
x_train = np.reshape(np.stack(x_train), (-1, 58*20))
x_test = np.reshape(np.stack(x_test), (-1, 58*20))

model = keras.Sequential(
    [
        layers.Dense(1500, activation="relu"),
        layers.Dropout(0.4),
        layers.Dense(800, activation="relu"),
        layers.Dense(1),
    ]
)

model.compile(
    optimizer=keras.optimizers.Nadam(learning_rate=0.00001),
    loss=tf.losses.MeanAbsoluteError()
)

latest = tf.train.latest_checkpoint(checkpoint_dir)
model.load_weights(latest)

history = model.fit(x_train, y_train, batch_size=32, epochs=100, verbose=2, validation_split=0.2, callbacks=[cp_callback])
# hist = pd.DataFrame(history.history)
# hist['epoch'] = history.epoch
# print(hist.tail())

model.evaluate(x_test, y_test, verbose=2)


x_predict = x_test[:10]
y_actual = y_test[:10]
y_predicted = model.predict(x_predict)


print("x: ", np.argmax(x_predict, axis=1))
print("actual: ", y_actual)
print("predicted: ", y_predicted)
# print("diff: ", y_diff)

def plot_loss(history):
    plt.plot(history.history['loss'], label='loss')
    plt.plot(history.history['val_loss'], label='val_loss')
    plt.ylim([0, 45])
    plt.xlabel('Epoch')
    plt.ylabel('Error')
    plt.legend()
    plt.grid(True)
#
# plot_loss(history)
# plt.show()











