import settings
import os
import shutil

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
