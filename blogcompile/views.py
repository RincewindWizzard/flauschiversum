from jinja2 import Environment, FileSystemLoader
from slugify import slugify
import os
import subprocess
import settings
from blogcompile.query import query_images, query_pages, query_posts, filtered_dataset, pagination, only_once, query_static
from blogcompile.urls import get_url_for_post, get_url_for_pagination, get_url_for
from datetime import datetime
from . import urls
import PIL

env = Environment(loader=FileSystemLoader('templates'))


@pagination(query_posts, sort_key=lambda post: post.date, reverse=True, pagesize=4)
def render_post_index(index, page, pagecount):
    start = max([0, index - settings.PAGINATION_SIZE//2])
    end = min([pagecount, index + settings.PAGINATION_SIZE//2]) + 1
    return (
        get_url_for_pagination(index),
        env.get_template('index.html').render(
            currentyear=datetime.now().year,
            posts=page,
            pagenum=index,
            pagination=list(range(start, end)),
            page_urls=[urls.get_url_for_pagination(index) for index in range(pagecount)],
            last_page=pagecount
        ).encode('utf-8')
    )


@filtered_dataset(query_posts)
def render_post(post):
    return (get_url_for_post(post), env.get_template('post.html').render(post=post, title=post.title).encode('utf-8'))

@filtered_dataset(query_images)
def render_image(img):
    yield urls.get_url_for_image(img, settings.IMAGE_SMALL_WIDTH), img.small
    yield urls.get_url_for_image(img, settings.IMAGE_MEDIUM_WIDTH), img.medium
    yield urls.get_url_for_image(img, settings.IMAGE_LARGE_WIDTH), img.large


@filtered_dataset(query_pages)
def render_pages(page):
    return (get_url_for(page), env.get_template('page.html').render(page=page, title=page.title).encode('utf-8'))

@only_once
def style():
    return ('/style.css',
            subprocess.run(['lessc', os.path.join(settings.STYLE_PATH, 'main.less')], stdout=subprocess.PIPE).stdout)

@filtered_dataset(query_static)
def static(static_file):
    return (get_url_for(static_file), static_file.content)