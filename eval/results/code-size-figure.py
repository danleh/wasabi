#!/usr/bin/python
import csv
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns

# print pd.__version__

df = pd.read_csv("code-size.csv", skipinitialspace=True)

df = df.drop(["path"], axis=1)
df = df[df.hooks != "all"]
df = df[df.hooks != "instruction-mix"]
df = df[df.hooks != "coverage-branch"]

for i, row in df.iterrows():
	original_bytes = df[(df["hooks"] == "original") & (df["filename"] == row["filename"])].iloc[0]["bytes"]
	df.ix[i, "overhead"] = row["bytes"] / original_bytes

df = df[df.hooks != "original"]

# print df
# print df.dtypes

# fig, ax = plt.subplots(figsize=(10,4))
g = sns.factorplot(x='filename', y='overhead', hue='hooks', kind='bar', aspect=3, data=df)
g.set_xticklabels(rotation=90)
g.savefig("code-size.svg")
# plt.close(fig)
