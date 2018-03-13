#! /usr/bin/python3
# -*- coding: utf-8 -*-
import logging, configparser, os
from os.path import expanduser
from slugify import slugify

# Configuration
base_dir = '.'
posts_path = os.path.join(base_dir, 'content/posts/')
templates_dir = os.path.join(base_dir, 'templates')
def post_path(title, category=None):
  return os.path.join(posts_path, slugify(title))

static_path = os.path.join(base_dir, 'static/')
static_images_path = os.path.join(base_dir, 'static/images/')
overlay = os.path.join(static_images_path, 'Wasserzeichen.png')
index_file = 'index.md'
pretty_xml = 'pretty' # 'pretty', 'minimize', None
pagination_size = 5
posts_per_page = 5
image_dimension = (1000, 750)
date_fmt = '%Y-%m-%d'
file_opener = 'xdg-open'    # program to use for file opening
localserver = 'http://localhost:5000/'

config = configparser.ConfigParser()
config.read(expanduser('~/.flauschiversum/auth.conf'))
if 'ftp' in config.sections():
  ftp_host     = config['ftp']['host']
  ftp_user     = config['ftp']['user']
  ftp_password = config['ftp']['password']
else:
  logging.error('Keine Zugangsdaten für den FTP Upload gefunden!')


# Template for new posts
post_header_template = """---
title: "{}"
category: {}
author: {}
date: {}
image: "{}"
excerpt: "{}"
---

"""

