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
    os.makedirs(settings.BUILD_PATH, exist_ok=True)

    dataset = list(sourcewalker.find_sources('src'))
    view_list = [ views.static, views.style, views.render_pages, views.render_post, views.render_post_index, views.render_image ]

    for view in view_list:
        for url, content in view(dataset):
            print(url)
            export(url, content)


def export(url, content):
    dst = os.path.join(settings.BUILD_PATH, url.lstrip('/'))
    if dst[-1] == '/':
        dst = os.path.join(dst, 'index.html')

    os.makedirs(os.path.dirname(dst), exist_ok=True)
    with open(dst, 'wb') as f:
        f.write(content)