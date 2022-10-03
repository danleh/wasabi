#!/usr/bin/python
import csv
import pandas as pd
import numpy as np
import matplotlib as mpl
import matplotlib.pyplot as plt
import seaborn as sns
import scipy as sp

df = pd.read_csv("runtime-wasabi.csv", skipinitialspace=True)

df.loc[df.file == "pspdfkit.wasm", "program_group"] = "PSPDFKit"
df.loc[df.file == "UE4Game-HTML5-Shipping.wasm", "program_group"] = "Unreal Engine 4"
df.loc[(df.file != "pspdfkit.wasm") & (df.file != "UE4Game-HTML5-Shipping.wasm"), "program_group"] = "PolyBench"

print df[df.num_threads == 1].groupby(["program_group"]).time_ms.describe()
print df[df.num_threads == 4].groupby(["program_group"]).sum()

# average across runs with same file and num_threads
df = df.groupby(["file", "num_threads"]).mean().reset_index()

# add relative runtime column that is relative to single threaded execution
for i, row in df.iterrows():
	seq_ms = df[(df.num_threads == 1) & (df.file == row.file)].time_ms
	assert len(seq_ms) == 1
	df.ix[i, "relative"] = row.time_ms / float(seq_ms.iloc[0])

print df

# drop all runs that are not UE4
# df = df[df.file == "UE4Game-HTML5-Shipping.wasm"]

sns.set_style("ticks")
# sns.set_palette(
	# sns.xkcd_palette(["pale red", "medium green", "denim blue"])
	# sns.color_palette("Set1", n_colors=3)
# )

fp = mpl.font_manager.FontProperties(fname="/usr/share/fonts/truetype/fira/FiraSans-Regular.ttf")
# print fp
print mpl.font_manager.findfont("Fira Sans")

g = sns.pointplot(x="num_threads", y="relative", hue="file", aspect=2, size=2, data=df, legend=False,
	estimator=sp.stats.gmean,	
	# errwidth=0,
	# capsize=.1
)
# g.ax.yaxis.set_minor_locator(mpl.ticker.AutoMinorLocator())
sns.despine(offset=4)
plt.xlabel("Number of Threads", fontproperties=fp, fontsize=11,
	# labelpad=-8
)
plt.ylabel("Relative Runtime", fontproperties=fp, fontsize=11,
#	position=(0,.3)
)
# legend = plt.legend(
# 	loc="upper right",
# 	bbox_to_anchor=(1.02, 1.1),
# 	frameon=1,
# 	framealpha=1,
# 	prop=fp,
# 	fontsize=11
# )
# legend.get_frame().set_linewidth(0)
# plt.tick_params(axis='x', 
	# bottom=False, # ticks along the bottom edge are off
	# labelbottom=True)
plt.ylim((0,1.1))
# g.ax.yaxis.grid(b=True,which="major",color=".7",linewidth=.5)
# g.ax.yaxis.grid(b=True,which="minor",color=".85",linewidth=.5,linestyle="--")
# g.set_xticklabels(rotation=45, ha="center", position=(0,-.02), fontproperties="Inconsolata",fontsize="11", rotation_mode="anchor")
# g.set_xticklabels(fontproperties="Inconsolata",fontsize="10",
# 	rotation=45,
# 	rotation_mode="anchor",
# 	ha="right",
# 	position=(0,0.06),
# )
# g.set_yticklabels(g.ax.get_yticks(), fontproperties=fp)

# def change_width(ax, new_value) :
#     for patch in ax.patches :
#         current_width = patch.get_width()
#         diff = current_width - new_value

#         # we change the bar width
#         patch.set_width(new_value)

#         # we recenter the bar
#         patch.set_x(patch.get_x() + diff * .5)

# change_width(g.ax, .24)

plt.savefig("runtime-wasabi.pdf")
