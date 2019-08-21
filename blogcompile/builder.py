import shutil
import settings
import os
from . import sourcewalker, query, views

def clean():
    if os.path.isdir(settings.BUILD_PATH):
        shutil.rmtree(settings.BUILD_PATH)

def build():
    dataset = sourcewalker.find_sources('src')
    view_list = [ views.render_post, views.render_post_index ]

    for view in view_list:
        for url, content in view(dataset):
            export(url, content)

    export(*views.style())

    copy_static()

def export(url, content):
    dst = os.path.join(settings.BUILD_PATH, url.lstrip('/'))
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