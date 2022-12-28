#!/usr/bin/env python3

import json
import shutil
import os

import urllib.request
import py7zr


print('downloading WasmBench filtered dataset...')
urllib.request.urlretrieve("https://github.com/sola-st/WasmBench/releases/download/v1.0/filtered-binaries-metadata.7z", "filtered-binaries-metadata.7z")
print('extracting...')
with py7zr.SevenZipFile("filtered-binaries-metadata.7z", 'r') as archive:
    archive.extractall(path="filtered-binaries-metadata/")


with open('filtered-binaries-metadata/filtered.pretty.json') as f:
    metadata = json.load(f)
print('before filter:', len(metadata))


files_valid_no_extensions = [hash for hash, entry in metadata.items() if entry['wasm_validate_no_extensions'] == True]
print('valid binaries without extensions:', len(files_valid_no_extensions))


os.makedirs('valid-no-extensions/binaries', exist_ok=True)
for hash in files_valid_no_extensions:
    try:
        shutil.copyfile('filtered-binaries-metadata/filtered/' + hash + '.wasm', 'valid-no-extensions/binaries/' + hash + '.wasm')
    except Exception as e:
        print(e)
