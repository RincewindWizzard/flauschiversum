from jinja2 import Environment, FileSystemLoader
from slugify import slugify
import os
import subprocess
import settings
from blogcompile.query import query_images, query_pages, query_posts, paginate, filtered_dataset

env = Environment(loader=FileSystemLoader('templates'))


@filtered_dataset(query_posts, 4)
def render_post_index(index, page):
    url = '/index.html' if index == 0 else '/page/{}/index.html'.format(index)
    return (
        url,
        env.get_template('index.html').render(posts=page, pagenum=index, pagination=[], last_page=1).encode('utf-8')
    )


@filtered_dataset(query_posts)
def render_post(post):
    url = '/{:04d}/{:02d}/{}/index.html'.format(post.date.year, post.date.month, slugify(post.title))
    return (url, env.get_template('post.html').render(post=post, title=post.title).encode('utf-8'))


def style():
    return ('/style.css',
            subprocess.run(['lessc', os.path.join(settings.STYLE_PATH, 'main.less')], stdout=subprocess.PIPE).stdout)
