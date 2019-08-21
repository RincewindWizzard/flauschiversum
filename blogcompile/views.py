from jinja2 import Environment, FileSystemLoader
from slugify import slugify
import os
import subprocess
import settings
from blogcompile.query import query_images, query_pages, query_posts, paginate, filtered_dataset, pagination
from blogcompile.urls import get_url_for, get_url_for_pagination
from datetime import datetime
from . import urls

env = Environment(loader=FileSystemLoader('templates'))


@pagination(query_posts, sort_key=lambda post: post.date, reverse=True, pagesize=4)
def render_post_index(index, page, pagecount):


    """
        html = render_template(
      'index.html', 
      posts=cur_page_post_list,
      category=category,
      currentyear=datetime.now().year,
      pagenum=page,
      page_urls=page_urls,
      pagination=list(range(start, end)),
      last_page=len(page_urls) - 1
    )
    """
    start = max([0, index - settings.PAGINATION_SIZE//2])
    end = min([pagecount, index + settings.PAGINATION_SIZE//2])
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
    return (get_url_for(post), env.get_template('post.html').render(post=post, title=post.title).encode('utf-8'))


def style():
    return ('/style.css',
            subprocess.run(['lessc', os.path.join(settings.STYLE_PATH, 'main.less')], stdout=subprocess.PIPE).stdout)
