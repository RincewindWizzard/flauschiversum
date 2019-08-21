import settings
import os
import shutil

"""
It is very resource intensive to load an image and resize it. 
So when creating a new Image object, we look in CACHE_PATH if it has already been resized an read the cached files 
instead of redoing the resize.
This gives us a huge performance boost in comparision with a fresh build.
The images are cached by their sha256 sum.
So changing an image triggers a new resize.
"""


def clean():
    shutil.rmtree(settings.CACHE_PATH)


def store_image(img):
    path = os.path.join(settings.CACHE_PATH, img.sha256sum)
    if not os.path.isdir(path):
        os.makedirs(path)

        store(os.path.join(path, 'small.jpg'), img.small)
        store(os.path.join(path, 'medium.jpg'), img.medium)
        store(os.path.join(path, 'large.jpg'), img.large)


def retrieve_image(img):
    path = os.path.join(settings.CACHE_PATH, img.sha256sum)
    if os.path.isdir(path):
        return (
            retrieve(os.path.join(path, 'small.jpg')),
            retrieve(os.path.join(path, 'medium.jpg')),
            retrieve(os.path.join(path, 'large.jpg'))
        )
    else:
        return (None, None, None)


def store(key, value):
    with open(os.path.join(settings.CACHE_PATH, key), 'wb') as f:
        f.write(value)


def retrieve(key):
    with open(os.path.join(settings.CACHE_PATH, key), 'rb') as f:
        return f.read()
