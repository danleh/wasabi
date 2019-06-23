#!/bin/bash
dot -Ksfdp -Goverlap=false -Tpdf $1 > "${1%.*}.pdf"
