import shutil
import settings
import os
from . import sourcewalker, query, views

def clean():
    # clears the build folder
    if os.path.isdir(settings.BUILD_PATH):
        for basename in os.listdir(settings.BUILD_PATH):
            dst = os.path.join(settings.BUILD_PATH, basename)
            if os.path.isfile(dst):
                os.remove(dst)
            else:
                shutil.rmtree(dst)

def build():
    # for performance purposes do static copy before
    copy_static()

    dataset = list(sourcewalker.find_sources('src'))
    view_list = [ views.style, views.render_post, views.render_post_index, views.render_image ]

    for view in view_list:
        for url, content in view(dataset):
            print(url)
            export(url, content)

    # and after, so we are sure nothing has been overwritten
    copy_static()

def export(url, content):
    dst = os.path.join(settings.BUILD_PATH, url.lstrip('/'))
    if dst[-1] == '/':
        dst = os.path.join(dst, 'index.html')

    os.makedirs(os.path.dirname(dst), exist_ok=True)
    with open(dst, 'wb') as f:
        f.write(content)

def copy_static():
    os.makedirs(settings.BUILD_PATH, exist_ok=True)
    for dirpath, dirnames, filenames in os.walk(settings.STATIC_PATH):
        dst_path = os.path.join(settings.BUILD_PATH, os.path.relpath(dirpath, settings.STATIC_PATH))
        os.makedirs(dst_path, exist_ok=True)
        for filename in filenames:
            src = os.path.join(dirpath, filename)
            dst = os.path.join(settings.BUILD_PATH, os.path.relpath(src, settings.STATIC_PATH))
            shutil.copyfile(src, dst)