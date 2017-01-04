#! /usr/bin/python3
# -*- coding: utf-8 -*-
import subprocess, os, io, logging
from datetime import datetime
from PIL import Image
from flask import Flask, Response, request, send_file, render_template, send_from_directory, abort, after_this_request
app = Flask(__name__)

import htmlmin
from bs4 import BeautifulSoup as parse_html_string

import settings
from database import dbLock, database
import database as db


def postprocess(html):
  if settings.pretty_xml == 'pretty':
    return parse_html_string(html, 'lxml').prettify()
  elif settings.pretty_xml == 'minimize':
    return htmlmin.minify(html)
  else:
    return html

@app.route('/', defaults={'category': None, 'page': 0})
@app.route('/page/<int:page>/index.html', defaults={'category': None})
@app.route('/basteln/',   defaults={'category': 'basteln', 'page': 0})
@app.route('/brandings/', defaults={'category': 'brandings', 'page': 0})
@app.route('/filzen/',    defaults={'category': 'filzen', 'page': 0})
@app.route('/malen/',     defaults={'category': 'malen', 'page': 0})
@app.route('/nähen/',     defaults={'category': 'nahen', 'page': 0})
@app.route('/naehen/',    defaults={'category': 'nahen', 'page': 0})
@app.route('/nahen/',     defaults={'category': 'nahen', 'page': 0})
@app.route('/wolle/',     defaults={'category': 'wolle', 'page': 0})
@app.route('/<category>/page/<int:page>/index.html')
def index(category, page):
  if page == 0: db.load_posts()
  with dbLock:
    posts = list(reversed(db.posts_by_date(category)))
    page_urls = ['/' + category if category else '/'] + [ 
      os.path.join('/', category if category else '', 'page', str(pagenum), 'index.html#index')
      for pagenum in range(1, max(len(posts)//settings.posts_per_page, 1))
    ]


    # pagination
    start = max(0, page - settings.pagination_size//2)
    end = min(len(page_urls), start + settings.pagination_size)

    html = render_template(
      'index.html', 
      posts=posts[page * settings.posts_per_page:page * settings.posts_per_page + settings.posts_per_page],
      category=category,
      currentyear=datetime.now().year,
      pagenum=page,
      page_urls=page_urls,
      pagination=list(range(start, end)),
      last_page=len(page_urls)-1
    )
    response = Response(postprocess(html))
    response.headers['Last-modified'] = datetime.now().strftime('%a, %d %m %Y %H:%M:%S GMT')
    return response

@app.route('/<int:year>/<int:month>/<title>/')
def render_post(year, month, title):
  with dbLock:
    if request.path in database['post_by_url']:
      post = database['post_by_url'][request.path]
      post.reload()
      #response.headers["Content-Disposition"] = "attachment; filename=books.csv"
      response = Response(
        postprocess(
          render_template('post.html', post=post, title=post.title)
        )
      )
      response.headers['Last-modified'] = datetime.now().strftime('%a, %d %m %Y %H:%M:%S GMT')
      return response
    else:
      logging.error("{} not found!".format(request.path))
      return abort(404)
    

@app.route('/<int:year>/<int:month>/<title>/<int:width>x<int:height>/<image>')
@app.route('/<int:year>/<int:month>/<title>/<int:width>x/<image>', defaults={'height': 1000})
@app.route('/<int:year>/<int:month>/<title>/x<int:height>/<image>', defaults={'width': 1000})
def convert_thumbnail(year, month, title, width, height, image):
  post = database['post_by_url']['/{}/{:02d}/{}/'.format(year, month, title)]
  imgpath = os.path.join(post.path, image)
  if os.path.isfile(imgpath):
    img = Image.open(imgpath)

    overlay = Image.open(settings.overlay)
    overlay.thumbnail(img.size, Image.ANTIALIAS)

    img.paste(overlay, (img.size[0] - overlay.size[0], img.size[1] - overlay.size[1]), overlay)
    img.thumbnail((width, height), Image.ANTIALIAS)

    f = io.BytesIO()
    img.save(f, format= 'JPEG')


    @after_this_request
    def add_header(response):
        response.headers['Last-modified'] = datetime.fromtimestamp(
          os.path.getmtime(imgpath)
        ).strftime('%a, %d %m %Y %H:%M:%S GMT')
        return response

    return send_file(io.BytesIO(f.getvalue()),
                     attachment_filename=image,
                     mimetype='image/jpg')
  else:
    abort(404)

@app.route('/<path>/<basename>')
@app.route('/<basename>', defaults={'path': ''})
def static_files(path, basename):
  return send_from_directory(os.path.join('static', path), basename)

@app.route('/style.css')
def style():
  return Response(
    subprocess.run(['lessc', 'static/style/main.less'], 
      stdout=subprocess.PIPE).stdout,
    mimetype='text/css'
  )

@app.route('/impressum.html')
def impressum():
  return postprocess(
    render_template('impressum.html')
  )

@app.route('/autoren.html')
def autoren():
  return postprocess(
    render_template('autoren.html')
  )

@app.route('/robots.txt')
def robots_txt():
  return ""
