import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt
import numpy as np

min_score = 200

dff = pd.read_csv('../../assets/datasets/complete.csv')
dff = dff[dff["is_ox"] == False]
dff = dff[dff["score"] > min_score]
dff = dff[dff["retention_time"] != 0]
heatmap_dff = dff[["retention_time", "score", "mass", "hs_kd", "hs_hw", "hs_c"]]

dfp = pd.read_csv('../../assets/datasets/per_file.csv')
dfp = dfp[dfp["is_ox"] == False]
dfp = dfp[dfp["score"] > min_score]
dfp = dfp[dfp["retention_time"] != 0]
heatmap_dfp = dfp[["retention_time", "score", "mass", "hs_kd", "hs_hw", "hs_c"]]

dfu = pd.read_csv('../../assets/datasets/final.csv')
dfu = dfu[dfu["is_ox"] == False]
dfu = dfu[dfu["score"] > min_score]
dfu = dfu[dfu["retention_time"] != 0]
heatmap_dfu = dfu[["retention_time", "score", "mass", "hs_kd", "hs_hw", "hs_c"]]

count_f, count_p, count_u = heatmap_dff.shape[0], heatmap_dfp.shape[0], heatmap_dfu.shape[0]

fig, ax = plt.subplots(1, 3)
sns.heatmap(heatmap_dfu.corr(), cmap='RdYlGn_r', linewidths=0.5, annot=True, ax=ax[0]).set(title=f"Unique ({count_u})")
sns.heatmap(heatmap_dfp.corr(), cmap='RdYlGn_r', linewidths=0.5, annot=True, ax=ax[1]).set(title=f"Per File ({count_p})")
sns.heatmap(heatmap_dff.corr(), cmap='RdYlGn_r', linewidths=0.5, annot=True, ax=ax[2]).set(title=f"Original ({count_f})")
plt.show()
