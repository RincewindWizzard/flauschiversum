#!/usr/bin/env sh

# update sources


# update dependencies
pip3 install -r requirements.txt

# build flauschiversum
python main.py --dst /var/www/flauschiversum/ --cache /var/share/flauschiversum-cache/