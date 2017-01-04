#! /usr/bin/python3
# -*- coding: utf-8 -*-
import logging, io, os
import ftplib
from ftplib import FTP_TLS as FTPS
from settings import ftp_host, ftp_user, ftp_password

class FTPUpload(FTPS):
  def upload_bytes(self, path, bs):
    self.upload(path, io.BytesIO(bs))

  def upload(self, path, f):
    self.mkdir(os.path.dirname(path))
    self.storbinary('STOR {}'.format(path), f)

  def download_bytes(self, path):
    out = io.BytesIO()
    self.download(path, out)
    return out.getvalue()

  def download(self, path, f):
    self.retrbinary('RETR {}'.format(path), f.write)

  def mkdir(self, path):
    folders = [ folder for folder in path.split('/') if folder ]
    path = ''
    for folder in folders:
      path = os.path.join(path, folder)
      try:
        self.mkd(path)
      except ftplib.error_perm as e:
        if e.args[0].find('File exists') > -1:
          ... # pass
        else: 
          raise e

with FTPUpload(ftp_host, ftp_user, ftp_password) as ftp:
  print(f[0] for f in ftp.mlsd('/'))
  ftp.upload_bytes('/fup/fap/fop/foo.txt', b'Haaaallo')
  print(ftp.download_bytes('/fup/fap/fop/foo.txt'))
