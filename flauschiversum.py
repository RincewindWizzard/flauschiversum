#! /usr/bin/python3
# -*- coding: utf-8 -*-

import logging
from builder.routes import app
import builder.database as db

def main(debug=True):
  logging.debug('Started Webserver.')
  db.load_posts(debug=debug)
  try:
    app.run(host="0.0.0.0")
  except OSError as e:
    if e.errno == 98:
      logging.error("Address already in use!")
    else:
      raise e


def as_process():
  from multiprocessing import Process, Queue
  from time import sleep
  p = Process(target = main)
  p.start()
  sleep(5)
  print("Terminate")
  p.terminate()
  p.join()
  print("done")

if __name__ == '__main__':
  logging.basicConfig(format='%(levelname)s:%(message)s', level=logging.ERROR)
  main(debug=False)
