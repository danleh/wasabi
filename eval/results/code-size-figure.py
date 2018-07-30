#!/usr/bin/python
import csv
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns

print pd.__version__

data = pd.read_csv("code-size.csv", skipinitialspace=True)
data = data.drop(["path"], axis=1)
data["overhead"] = np.where()
print data.dtypes
fig, ax = plt.subplots(figsize=(10,4))
plt.xticks(rotation=45)
sns.factorplot(x='filename', y='bytes', hue='hooks', kind='bar', data=data, ax=ax)
fig.savefig("code-size.svg")
plt.close(fig)
