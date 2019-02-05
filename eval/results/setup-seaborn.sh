# make sure pip is from python 2 distribution
pip --version
# if not (broken for me after Ubuntu 18.04 upgrade), use explicit pip2 binary
~/.local/bin/pip --version
# use seaborn 0.8.1
~/.local/bin/pip install --user seaborn==0.8.1
