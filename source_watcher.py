import os, shutil, tempfile, sys, argparse, threading, logging, multiprocessing, queue, re, subprocess
from multiprocessing import Queue, Process
import inotify.adapters
from PIL import Image
from inotify.constants import IN_DELETE, IN_CREATE, IN_MODIFY, IN_CLOSE_WRITE

import settings

def resize_image(img_path):
  #timg = tempfile.mktemp()
  img = Image.open(img_path)
  img.load()

  #shutil.move(img_path, timg)
  prefix, suffix = os.path.splitext(img_path)
  dst_suffix = '.JPG' if suffix == '.JPG' else '.jpg'
  dst = prefix + dst_suffix
  
  if img.size[0] > settings.image_dimension[0] or img.size[1] > settings.image_dimension[1]:
    img.thumbnail(settings.image_dimension, Image.ANTIALIAS)
    logging.debug('Resizing {}'.format(img_path))
    with open(dst, 'wb') as f:
      img.save(f, format= 'JPEG')

    if not img_path == dst:
      os.remove(img_path)

def convert_all(paths, done=None):
  """
  * Converts all images in given folder, that are bigger than settings.image_dimension
  """
  for path in paths:
    for img in os.listdir(path):
      prefix, suffix = os.path.splitext(img)
      if suffix.lower() in ['.jpg', '.png']:
        try:
          resize_image(os.path.join(path, img))
        except FileNotFoundError as e:
          ...
        except Exception as e:
          logging.exception(e)
        except OSError as e:
          logging.exception(e)
 

def watcher_main(queue, stop):
  """
  * Extra Process that watches for source change because of GIL, you know
  """
  logging.basicConfig(format='%(levelname)s:%(message)s', level=logging.DEBUG)
  # convert all images to max size first
  convert_all([ path for path, _, _ in os.walk(settings.posts_path) ])


  watch = inotify.adapters.InotifyTree(
    bytes(settings.posts_path, 'utf-8'),
    mask=IN_CLOSE_WRITE
  )

  # regex for multiple data types
  image_files   = [ re.compile(s) for s in [r'.*\.png$', r'.*\.jpg$', r'.*\.JPG$'] ]
  watched_files = [ re.compile(s) for s in [r'.*\.png$', r'.*\.jpg$', r'.*\.JPG$', r'index\.md$'] ]

  for event in watch.event_gen():
    if stop.is_set(): break
    if event:
      header, type_names, watch_path, filename = event
      watch_path = watch_path.decode('utf-8')
      filename = filename.decode('utf-8')
      filepath = os.path.join(watch_path, filename)


      if any([ r.match(filename) for r in watched_files ]):
        queue.put(filepath)
      if any([ r.match(filename) for r in image_files ]):
        try:
          resize_image(filepath)
        except FileNotFoundError as e:
          ...
        except Exception as e:
          logging.exception(e)
        except OSError as e:
          logging.exception(e)


def watch_sources(progress=None, stopped=None):
  """
  * Watch for changes in the post path.
  * progress is called everytime something changed and code has to be recompiled.
  * Automatically resizes image assets.
  """
  multiprocessing.set_start_method('spawn')
  q = Queue()
  timeout = 1
  stop_event = multiprocessing.Event()
  p = Process(target=watcher_main, args=(q, stop_event))
  p.start()

  while not stopped():
    try:
      filepath = q.get(True, timeout)
      progress(filepath)
    except queue.Empty as e:
      ...

  stop_event.set()


error_line = re.compile(r'.*FEHLER.*')
def check404(*args, **kwargs):
  """
  * Recursively requests all resources of the blog and reports everything missing
  """
  missing_files = []
  temp_path = tempfile.mkdtemp()
  build_path = os.path.join(temp_path, 'build')
  os.mkdir(build_path)
  p = subprocess.Popen(
    ['wget', '-mnH', '-nv', settings.localserver],
    cwd = build_path,
    stderr = subprocess.PIPE
  )
  stdout, stderr = p.communicate()
  lines = stderr.decode('utf-8').split('\n')
  for i, line in enumerate(lines):
    if error_line.match(line):
      missing_files.append(lines[max(i - 1, 0)][:-1])
  p.wait()
  return missing_files

"""

def main():
  parser = argparse.ArgumentParser(
    prog = 'asset_resizer',
    description = 'Resize every image in a folder to a given max size. ' + 
                  'Watch for new Images.'
  )
  parser.add_argument('paths', nargs='+', help='Path to the folder to be watched.')
  args = parser.parse_args()

  paths = []
  for path in args.paths:
    paths.extend([ path for path, _, _ in os.walk(path) ])

  try:
    convert_all(paths)
  except KeyboardInterrupt:
    ...

if __name__ == '__main__':
  logging.basicConfig(format='%(levelname)s:%(message)s', level=logging.DEBUG)
  main()"""
