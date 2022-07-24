# nim plotly is a bit sus so we use python for this :c
import plotly.express as px

# --- Features General

data = [
  [1.0000,   0.7266,    0.1252,   0.0392],
  [0.7266,   1.0000,   -0.0272,   0.0709],
  [0.1252,  -0.0272,    1.0000,   0.9460],
  [0.0392,   0.0709,    0.9460,   1.0000]
]

fig = px.imshow(
  data,
  labels=dict(color="Correlation"),
  text_auto=True,
  color_continuous_scale="viridis",
  x=['Retention Time', 'Gravy', 'Mass', 'length'],
  y=['Retention Time', 'Gravy', 'Mass', 'length']

)

fig.update_layout(
  font=dict(
    size=32
  )
)
fig.update_xaxes(side="top")
fig.show()

# --- Hydrophobicity

data = [
  [ 1.0000,  -0.7925,    0.8625],
  [-0.7925,   1.0000,   -0.7413],
  [ 0.8625,  -0.7413,    1.0000],
]

fig = px.imshow(
  data,
  labels=dict(color="Correlation"),
  text_auto=True,
  # color_continuous_scale="turbo",
  x=['Kyte Doolittle', 'Hopp Woods', 'Cornette'],
  y=['Kyte Doolittle', 'Hopp Woods', 'Cornette']

)

fig.update_layout(
  font=dict(
    size=32
  )
)
fig.update_xaxes(side="top")
fig.show()